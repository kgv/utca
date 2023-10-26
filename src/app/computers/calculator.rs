use crate::{
    app::context::{
        settings::calculation::{From, Normalization, Signedness, Source},
        state::Normalized as Value,
        Context,
    },
    utils::Normalize,
};
use egui::util::cache::{ComputerMut, FrameCache};
use std::{
    hash::{Hash, Hasher},
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
        let masses: Vec<_> = context
            .state
            .entry()
            .meta
            .formulas
            .iter()
            .map(|formula| formula.weight())
            .collect();
        // Experimental
        let experimental = |unnormalized: &[f64]| {
            match context.settings.calculation.normalization {
                // s / ∑(s)
                Normalization::Mass => unnormalized.iter().copied().normalize(),
                // (s * m) / ∑(s * m)
                Normalization::Molar => unnormalized
                    .iter()
                    .zip(&masses)
                    .map(|(unnormalized, mass)| unnormalized * mass)
                    .normalize(),
                // s / ∑(s * m / 10.0)
                Normalization::Pchelkin => {
                    let sum = unnormalized
                        .iter()
                        .zip(&masses)
                        .fold(0.0, |sum, (unnormalized, mass)| {
                            sum + unnormalized * mass / 10.0
                        });
                    unnormalized
                        .iter()
                        .map(|unnormalized| unnormalized / sum)
                        .collect()
                }
            }
        };
        // Cast
        let cast = |value: f64| match context.settings.calculation.signedness {
            Signedness::Signed => value,
            Signedness::Unsigned => value.max(0.0),
        };

        let tags123 = experimental(&context.state.entry().data.unnormalized.tags123);
        let mut dags1223 = experimental(&context.state.entry().data.unnormalized.dags1223);
        let mut mags2 = experimental(&context.state.entry().data.unnormalized.mags2);
        trace!(?tags123, ?dags1223, ?mags2);
        if let Source::Calculated = context.settings.calculation.sources.dag1223 {
            dags1223 = tags123
                .iter()
                .zip(&mags2)
                .map(|(tag123, mag2)| cast((3.0 * tag123 + mag2) / 4.0))
                .normalize();
        }
        if let Source::Calculated = context.settings.calculation.sources.mags2 {
            mags2 = tags123
                .iter()
                .zip(&dags1223)
                .map(|(tag123, dag1223)| cast(4.0 * dag1223 - 3.0 * tag123))
                .normalize();
        }
        trace!(?dags1223, ?mags2);
        let dags13 = match context.settings.calculation.sources.dag13 {
            From::Dag1223 => tags123
                .iter()
                .zip(&dags1223)
                .map(|(tag123, dag1223)| cast(3.0 * tag123 - 2.0 * dag1223))
                .normalize(),
            From::Mag2 => tags123
                .iter()
                .zip(&mags2)
                .map(|(tag123, mag2)| cast((3.0 * tag123 - mag2) / 2.0))
                .normalize(),
        };
        trace!(?dags13);
        Arc::new(Value {
            tags123,
            dags1223,
            mags2,
            dags13,
        })
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
        self.context.state.entry().data.unnormalized.hash(state);
    }
}

impl<'a> std::convert::From<&'a Context> for Key<'a> {
    fn from(value: &'a Context) -> Self {
        Self { context: value }
    }
}
