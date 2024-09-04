use crate::{
    app::panes::calculation::settings::{Fraction, Settings, Sign},
    r#const::relative_atomic_mass::{C, CH2, H, O},
};
use egui::{
    emath::OrderedFloat,
    util::cache::{ComputerMut, FrameCache},
};
use polars::prelude::*;
use std::hash::{Hash, Hasher};
use tracing::trace;

/// Calculated
pub(in crate::app) type Calculated = FrameCache<Value, Calculator>;

/// Calculator
#[derive(Default)]
pub(in crate::app) struct Calculator;

// mass fraction
fn mass_fraction(name: &str) -> Expr {
    col(name) / sum(name)
}

// mole fraction
fn mole_fraction(name: &str) -> Expr {
    col(name) / molar_mass("FA.Formula") / (col(name) / molar_mass("FA.Formula")).sum()
}

fn temp_fraction(name: &str) -> Expr {
    col(name) / (col(name) * molar_mass("FA.Formula") / lit(10)).sum()
}

// Fatty acid methyl ester molar mass
fn molar_mass(name: &str) -> Expr {
    (c(name) + lit(1)) * lit(C) + (h(name) + lit(2)) * lit(H) + lit(2) * lit(O)
}

fn c(name: &str) -> Expr {
    col(name).list().len() + lit(1)
}

fn h(name: &str) -> Expr {
    lit(2) * c(name) - lit(2) * col(name).list().eval(col("").abs(), true).list().sum()
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
            Fraction::Mass => lazy_frame.with_columns([
                mass_fraction("TAG").name().suffix(".Experimental"),
                mass_fraction("DAG1223").name().suffix(".Experimental"),
                mass_fraction("MAG2").name().suffix(".Experimental"),
            ]),
            Fraction::Mole { mixture: false } => lazy_frame.with_columns([
                mole_fraction("TAG").name().suffix(".Experimental"),
                mole_fraction("DAG1223").name().suffix(".Experimental"),
                mole_fraction("MAG2").name().suffix(".Experimental"),
            ]),
            Fraction::Mole { mixture: true } => lazy_frame
                .with_columns([
                    temp_fraction("TAG").name().suffix(".Experimental"),
                    temp_fraction("DAG1223").name().suffix(".Experimental"),
                    temp_fraction("MAG2").name().suffix(".Experimental"),
                ])
                .with_columns([
                    (col("TAG.Experimental") / sum("TAG.Experimental")),
                    (col("DAG1223.Experimental") / sum("DAG1223.Experimental")),
                    (col("MAG2.Experimental") / sum("MAG2.Experimental")),
                ]),
        };
        lazy_frame = lazy_frame.with_column(molar_mass("FA.Formula").alias("FA.MolarMass"));
        println!("key.data_frame: {}", lazy_frame.clone().collect().unwrap());
        // Theoretical
        lazy_frame = lazy_frame
            .with_columns([
                clip((lit(4) * col("DAG1223.Experimental") - col("MAG2.Experimental")) / lit(3))
                    .alias("TAG.Theoretical"),
                ((lit(3) * col("TAG.Experimental") + col("MAG2.Experimental")) / lit(4))
                    .alias("DAG1223.Theoretical"),
                clip(lit(4) * col("DAG1223.Experimental") - lit(3) * col("TAG.Experimental"))
                    .alias("MAG2.Theoretical"),
            ])
            .with_columns([
                (col("TAG.Theoretical") / sum("TAG.Theoretical")),
                (col("DAG1223.Theoretical") / sum("DAG1223.Theoretical")),
                (col("MAG2.Theoretical") / sum("MAG2.Theoretical")),
            ])
            // Calculated
            .with_columns([
                clip(lit(3) * col("TAG.Experimental") - lit(2) * col("DAG1223.Experimental"))
                    .alias("DAG13.DAG1223.Calculated"),
                clip((lit(3) * col("TAG.Experimental") - col("MAG2.Experimental")) / lit(2))
                    .alias("DAG13.MAG2.Calculated"),
            ])
            .with_columns([
                (col("DAG13.DAG1223.Calculated") / sum("DAG13.DAG1223.Calculated")),
                (col("DAG13.MAG2.Calculated") / sum("DAG13.MAG2.Calculated")),
            ]);
        lazy_frame.collect().unwrap()
        // // Fractioner
        // let fractioner = Fractioner {
        //     fraction: context.settings.calculation.fraction,
        //     masses: context
        //         .state
        //         .entry()
        //         .meta
        //         .formulas
        //         .iter()
        //         .map(|formula| formula.weight() + CH2)
        //         .collect(),
        // };
        // // Cast
        // let cast = match context.settings.calculation.signedness {
        //     Signedness::Signed => |value| value,
        //     Signedness::Unsigned => |value: f64| value.max(0.0),
        // };

        // let configured = &context.state.entry().data.configured;
        // let mut calculated = Value::default();
        // // Experimental
        // calculated
        //     .tags123
        //     .experimental
        //     .unnormalized(configured.tags123().fractionize(&fractioner));
        // calculated
        //     .dags1223
        //     .experimental
        //     .unnormalized(configured.dags1223().fractionize(&fractioner));
        // calculated
        //     .mags2
        //     .experimental
        //     .unnormalized(configured.mags2().fractionize(&fractioner));

        // // Theoretical
        // calculated.tags123.theoretical.unnormalized(
        //     zip(
        //         &calculated.dags1223.experimental.unnormalized,
        //         &calculated.mags2.experimental.unnormalized,
        //     )
        //     .map(|(dag1223, mag2)| cast((4.0 * dag1223 - mag2) / 3.0)),
        // );
        // calculated.dags1223.theoretical.unnormalized(
        //     zip(
        //         &calculated.tags123.experimental.unnormalized,
        //         &calculated.mags2.experimental.unnormalized,
        //     )
        //     .map(|(tag123, mag2)| (3.0 * tag123 + mag2) / 4.0),
        // );
        // calculated.mags2.theoretical.unnormalized(
        //     zip(
        //         &calculated.tags123.experimental.unnormalized,
        //         &calculated.dags1223.experimental.unnormalized,
        //     )
        //     .map(|(tag123, dag1223)| cast(4.0 * dag1223 - 3.0 * tag123)),
        // );
        // trace!(?calculated.tags123, ?calculated.dags1223, ?calculated.mags2);

        // // 1,3-DAGs
        // calculated.dags13.dag1223.unnormalized(
        //     zip(
        //         &calculated.tags123.experimental.unnormalized,
        //         &calculated.dags1223.experimental.unnormalized,
        //     )
        //     .map(|(tag123, dag1223)| cast(3.0 * tag123 - 2.0 * dag1223)),
        // );
        // calculated.dags13.mag2.unnormalized(
        //     zip(
        //         &calculated.tags123.experimental.unnormalized,
        //         &calculated.mags2.experimental.unnormalized,
        //     )
        //     .map(|(tag123, mag2)| cast((3.0 * tag123 - mag2) / 2.0)),
        // );
        // trace!(?calculated.dags13);
        // Arc::new(calculated)
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
        for label in self.data_frame["Label"].str().unwrap() {
            label.hash(state);
        }
        for label in self.data_frame["Carbons"].str().unwrap() {
            label.hash(state);
        }
        for label in self.data_frame["Doubles"].str().unwrap() {
            label.hash(state);
        }
        for label in self.data_frame["Triples"].str().unwrap() {
            label.hash(state);
        }
        for tag in self.data_frame["TAG"].f64().unwrap() {
            tag.map(OrderedFloat).hash(state);
        }
        for dag1223 in self.data_frame["DAG1223"].f64().unwrap() {
            dag1223.map(OrderedFloat).hash(state);
        }
        for mag2 in self.data_frame["MAG2"].f64().unwrap() {
            mag2.map(OrderedFloat).hash(state);
        }
        self.settings.hash(state);
    }
}

/// Value
type Value = DataFrame;

// /// Fractioner
// struct Fractioner {
//     fraction: Fraction,
//     // Masses of methyl esters
//     masses: Vec<f64>,
// }

// /// Fractionize
// trait Fractionize {
//     fn fractionize(&mut self, fractioner: &Fractioner) -> Vec<f64>;
// }

// impl<'a, I: Iterator<Item = &'a f64>> Fractionize for I {
//     fn fractionize(&mut self, fractioner: &Fractioner) -> Vec<f64> {
//         let mut dividends = Vec::new();
//         let mut divisor = 0.0;
//         for (&dividend, mass) in zip(self, &fractioner.masses) {
//             dividends.push(match fractioner.fraction {
//                 Fraction::Molar { mixture: false } => dividend * mass,
//                 _ => dividend,
//             });
//             divisor += match fractioner.fraction {
//                 Fraction::Mass => dividend,
//                 _ => dividend * mass,
//             };
//         }
//         for dividend in &mut dividends {
//             *dividend /= divisor;
//         }
//         dividends
//     }
// }
