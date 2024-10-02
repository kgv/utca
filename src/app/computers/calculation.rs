use super::fatty_acid::ExprExt as _;
use crate::{
    app::{
        data::FattyAcids,
        panes::settings::calculation::{Fraction, From, Settings, Sign},
    },
    utils::ExprExt as _,
};
use egui::util::cache::{ComputerMut, FrameCache};
use polars::prelude::*;
use std::hash::{Hash, Hasher};

/// Calculation computed
pub(in crate::app) type Computed = FrameCache<Value, Computer>;

/// Calculation computer
#[derive(Default)]
pub(in crate::app) struct Computer;

// n = m / M
fn to_mole(name: &str) -> Expr {
    col(name) / col("FA").mass()
}

// m = n * M
fn to_mass(name: &str) -> Expr {
    col(name) * col("FA").mass()
}

fn pchelkin_fraction(name: &str) -> Expr {
    col(name) / (col(name) * col("FA").mass() / lit(10)).sum()
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        // Clip
        let clip = |expr: Expr| match key.settings.signedness {
            Sign::Signed => expr,
            Sign::Unsigned => expr.clip_min(lit(0)),
        };

        let mut lazy_frame = key.fatty_acids.0.clone().lazy();
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

/// Calculation key
#[derive(Clone, Copy, Debug)]
pub(in crate::app) struct Key<'a> {
    pub(in crate::app) fatty_acids: &'a FattyAcids,
    pub(in crate::app) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for fatty_acid in self.fatty_acids["FA"].phys_iter() {
            fatty_acid.hash(state);
        }
        for tag in self.fatty_acids["TAG"].phys_iter() {
            tag.hash(state);
        }
        for dag1223 in self.fatty_acids["DAG1223"].phys_iter() {
            dag1223.hash(state);
        }
        for mag2 in self.fatty_acids["MAG2"].phys_iter() {
            mag2.hash(state);
        }
        self.settings.hash(state);
    }
}

/// Calculation value
type Value = DataFrame;
