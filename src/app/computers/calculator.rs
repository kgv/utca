use crate::{
    app::context::{
        settings::calculation::{Fraction, Signedness},
        state::calculation::Calculated as Value,
        Context,
    },
    r#const::CH2,
};
use egui::util::cache::{ComputerMut, FrameCache};
use std::{
    hash::{Hash, Hasher},
    iter::zip,
    sync::Arc,
};
use tracing::trace;

/// Calculated
pub(in crate::app) type Calculated = FrameCache<Arc<Value>, Calculator>;

/// Calculator
#[derive(Default)]
pub(in crate::app) struct Calculator;

// stereospecific numbering (1,2,3-TAGs; 1,2/2,3-DAGs; 2-MAGs; 1,3-DAGs).
impl ComputerMut<Key<'_>, Arc<Value>> for Calculator {
    fn compute(&mut self, key: Key) -> Arc<Value> {
        let Key { context } = key;
        // Fractioner
        let fractioner = Fractioner {
            fraction: context.settings.calculation.fraction,
            masses: context
                .state
                .entry()
                .meta
                .formulas
                .iter()
                .map(|formula| formula.weight() + CH2)
                .collect(),
        };
        // Cast
        let cast = match context.settings.calculation.signedness {
            Signedness::Signed => |value| value,
            Signedness::Unsigned => |value: f64| value.max(0.0),
        };

        let configured = &context.state.entry().data.configured;
        let mut calculated = Value::default();
        // Experimental
        calculated
            .tags123
            .experimental
            .unnormalized(configured.tags123().fractionize(&fractioner));
        calculated
            .dags1223
            .experimental
            .unnormalized(configured.dags1223().fractionize(&fractioner));
        calculated
            .mags2
            .experimental
            .unnormalized(configured.mags2().fractionize(&fractioner));
        // Theoretical
        calculated.tags123.theoretical.unnormalized(
            zip(
                &calculated.dags1223.experimental.unnormalized,
                &calculated.mags2.experimental.unnormalized,
            )
            .map(|(dag1223, mag2)| cast((4.0 * dag1223 - mag2) / 3.0)),
        );
        calculated.dags1223.theoretical.unnormalized(
            zip(
                &calculated.tags123.experimental.unnormalized,
                &calculated.mags2.experimental.unnormalized,
            )
            .map(|(tag123, mag2)| (3.0 * tag123 + mag2) / 4.0),
        );
        calculated.mags2.theoretical.unnormalized(
            zip(
                &calculated.tags123.experimental.unnormalized,
                &calculated.dags1223.experimental.unnormalized,
            )
            .map(|(tag123, dag1223)| cast(4.0 * dag1223 - 3.0 * tag123)),
        );
        trace!(?calculated.tags123, ?calculated.dags1223, ?calculated.mags2);
        // 1,3-DAGs
        calculated.dags13.dag1223.unnormalized(
            zip(
                &calculated.tags123.experimental.unnormalized,
                &calculated.dags1223.experimental.unnormalized,
            )
            .map(|(tag123, dag1223)| cast(3.0 * tag123 - 2.0 * dag1223)),
        );
        calculated.dags13.mag2.unnormalized(
            zip(
                &calculated.tags123.experimental.unnormalized,
                &calculated.mags2.experimental.unnormalized,
            )
            .map(|(tag123, mag2)| cast((3.0 * tag123 - mag2) / 2.0)),
        );
        trace!(?calculated.dags13);
        Arc::new(calculated)
    }
}

/// Key
#[derive(Clone, Copy, Debug)]
pub struct Key<'a> {
    context: &'a Context,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context.settings.calculation.hash(state);
        self.context.state.entry().meta.hash(state);
        self.context.state.entry().data.configured.hash(state);
    }
}

impl<'a> std::convert::From<&'a Context> for Key<'a> {
    fn from(value: &'a Context) -> Self {
        Self { context: value }
    }
}

/// Fractioner
struct Fractioner {
    fraction: Fraction,
    // Masses of methyl esters
    masses: Vec<f64>,
}

/// Fractionize
trait Fractionize {
    fn fractionize(&mut self, fractioner: &Fractioner) -> Vec<f64>;
}

impl<'a, I: Iterator<Item = &'a f64>> Fractionize for I {
    fn fractionize(&mut self, fractioner: &Fractioner) -> Vec<f64> {
        let mut dividends = Vec::new();
        let mut divisor = 0.0;
        for (&dividend, mass) in zip(self, &fractioner.masses) {
            dividends.push(match fractioner.fraction {
                Fraction::Molar { mixture: false } => dividend * mass,
                _ => dividend,
            });
            divisor += match fractioner.fraction {
                Fraction::Mass => dividend,
                _ => dividend * mass,
            };
        }
        for dividend in &mut dividends {
            *dividend /= divisor;
        }
        dividends
    }
}
