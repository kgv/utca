use crate::{
    app::panes::calculation::settings::{Fraction, From, Settings, Sign},
    r#const::relative_atomic_mass::{C, H, O},
    utils::{DataFrameExt, ExprExt},
};
use egui::{
    emath::OrderedFloat,
    util::cache::{ComputerMut, FrameCache},
};
use polars::prelude::*;
use std::hash::{Hash, Hasher};

/// Calculated
pub(in crate::app) type Calculated = FrameCache<Value, Calculator>;

/// Calculator
#[derive(Default)]
pub(in crate::app) struct Calculator;

// n = m / M
fn to_mole(name: &str) -> Expr {
    col(name) / molar_mass()
}

// m = n * M
fn to_mass(name: &str) -> Expr {
    col(name) * molar_mass()
}

fn pchelkin_fraction(name: &str) -> Expr {
    col(name) / (col(name) * molar_mass() / lit(10)).sum()
}

// Fatty acid methyl ester molar mass
fn molar_mass() -> Expr {
    c() * lit(C) + h() * lit(H) + lit(2) * lit(O)
}

fn c() -> Expr {
    col("Carbons")
}

fn h() -> Expr {
    lit(2) * c() - lit(2) * d() - lit(4) * t()
}

fn d() -> Expr {
    col("Doubles").list().len()
}

fn t() -> Expr {
    col("Triples").list().len()
}

impl ComputerMut<Key<'_>, Value> for Calculator {
    fn compute(&mut self, key: Key) -> Value {
        // Clip
        let clip = |expr: Expr| match key.settings.signedness {
            Sign::Signed => expr,
            Sign::Unsigned => expr.clip_min(lit(0)),
        };

        let mut lazy_frame = key.data_frame.clone().lazy();
        // Experimental
        lazy_frame = match key.settings.fraction {
            Fraction::AsIs => lazy_frame.with_columns([
                col("TAG").normalize().suffix(".Experimental"),
                col("DAG1223").normalize().suffix(".Experimental"),
                col("MAG2").normalize().suffix(".Experimental"),
            ]),
            Fraction::ToMole => lazy_frame.with_columns([
                to_mole("TAG").normalize().suffix(".Experimental"),
                to_mole("DAG1223").normalize().suffix(".Experimental"),
                to_mole("MAG2").normalize().suffix(".Experimental"),
            ]),
            Fraction::ToMass => lazy_frame.with_columns([
                to_mass("TAG").normalize().suffix(".Experimental"),
                to_mass("DAG1223").normalize().suffix(".Experimental"),
                to_mass("MAG2").normalize().suffix(".Experimental"),
            ]),
            Fraction::Pchelkin => lazy_frame.with_columns([
                pchelkin_fraction("TAG").normalize().suffix(".Experimental"),
                pchelkin_fraction("DAG1223")
                    .normalize()
                    .suffix(".Experimental"),
                pchelkin_fraction("MAG2")
                    .normalize()
                    .suffix(".Experimental"),
            ]),
        };
        // Theoretical
        lazy_frame = lazy_frame
            .with_columns([
                clip((lit(4) * col("DAG1223.Experimental") - col("MAG2.Experimental")) / lit(3))
                    .normalize()
                    .alias("TAG.Theoretical"),
                ((lit(3) * col("TAG.Experimental") + col("MAG2.Experimental")) / lit(4))
                    .normalize()
                    .alias("DAG1223.Theoretical"),
                clip(lit(4) * col("DAG1223.Experimental") - lit(3) * col("TAG.Experimental"))
                    .normalize()
                    .alias("MAG2.Theoretical"),
            ])
            .with_columns([
                clip(lit(3) * col("TAG.Experimental") - lit(2) * col("DAG1223.Experimental"))
                    .normalize()
                    .alias("DAG13.DAG1223.Theoretical"),
                clip((lit(3) * col("TAG.Experimental") - col("MAG2.Experimental")) / lit(2))
                    .normalize()
                    .alias("DAG13.MAG2.Theoretical"),
            ]);
        // Calculated
        lazy_frame = lazy_frame.with_columns([
            col("MAG2.Experimental").alias("MAG2.Calculated"),
            match key.settings.from {
                From::Dag1223 => col("DAG13.DAG1223.Theoretical").alias("DAG13.Calculated"),
                From::Mag2 => col("DAG13.MAG2.Theoretical").alias("DAG13.Calculated"),
            },
        ]);
        lazy_frame.collect().expect("calculate data frame")
    }
}

/// Key
#[derive(Clone, Copy, Debug)]
pub struct Key<'a> {
    pub(crate) data_frame: &'a DataFrame,
    pub(crate) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for label in self.data_frame.str("Label") {
            label.hash(state);
        }
        for carbons in self.data_frame.u8("Carbons") {
            carbons.hash(state);
        }
        // for label in self.data_frame["Doubles"].list().unwrap() {
        //     label.hash(state);
        // }
        // for label in self.data_frame["Triples"].list().unwrap() {
        //     label.hash(state);
        // }
        for tag in self.data_frame.f64("TAG") {
            tag.map(OrderedFloat).hash(state);
        }
        for dag1223 in self.data_frame.f64("DAG1223") {
            dag1223.map(OrderedFloat).hash(state);
        }
        for mag2 in self.data_frame.f64("MAG2") {
            mag2.map(OrderedFloat).hash(state);
        }
        self.settings.hash(state);
    }
}

/// Value
type Value = DataFrame;
