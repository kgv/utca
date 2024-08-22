use crate::r#const::relative_atomic_mass::CH2;
use egui::util::cache::{ComputerMut, FrameCache};
use polars::prelude::*;
use std::{
    hash::{Hash, Hasher},
    iter::zip,
    sync::Arc,
};
use tracing::trace;

/// Calculated
pub(in crate::app) type Calculated = FrameCache<Value, Calculator>;

/// Calculator
#[derive(Default)]
pub(in crate::app) struct Calculator;

// stereospecific numbering (1,2,3-TAGs; 1,2/2,3-DAGs; 2-MAGs; 1,3-DAGs).\
// FA.Label ┆ FA.Formula  ┆ TAG ┆ DAG ┆ MAG
impl ComputerMut<Key<'_>, Value> for Calculator {
    fn compute(&mut self, key: Key) -> Value {
        key.data_frame
            .clone()
            .lazy()
            // Normalized
            .with_columns([
                (col("TAG") / sum("TAG")).name().suffix(".Normalized"),
                (col("DAG1223") / sum("DAG1223"))
                    .name()
                    .suffix(".Normalized"),
                (col("MAG2") / sum("MAG2")).name().suffix(".Normalized"),
            ])
            // Theoretical
            .with_columns([
                ((lit(4) * col("DAG1223") - col("MAG2")) / lit(3)).alias("TAG.Theoretical"),
                ((lit(3) * col("TAG") + col("MAG2")) / lit(4)).alias("DAG1223.Theoretical"),
                (lit(4) * col("DAG1223") - lit(3) * col("TAG")).alias("MAG2.Theoretical"),
            ])
            // Calculated
            .with_columns([
                (lit(3) * col("TAG") - lit(2) * col("DAG1223")).alias("DAG13.DAG1223.Calculated"),
                ((lit(3) * col("TAG") - col("MAG2")) / lit(2)).alias("DAG13.MAG2.Calculated"),
            ])
            .collect()
            .unwrap()
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
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data_frame.shape().hash(state);
        // self.context.settings.calculation.hash(state);
        // self.context.state.entry().meta.hash(state);
        // self.context.state.entry().data.configured.hash(state);
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
