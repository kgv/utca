use crate::{
    app::context::{
        settings::calculation::{From, Normalization, Signedness, Source},
        state::Normalized,
        Context,
    },
    utils::Normalize,
};
use egui::util::cache::{ComputerMut, FrameCache};
use std::{
    cell::LazyCell,
    hash::{Hash, Hasher},
};
use tracing::trace;

// fn signed(f: fn(&f64, &f64) -> f64) -> impl Fn(&f64, &f64) -> f64 {
//     f
// }

// fn unsigned(f: fn(&f64, &f64) -> f64) -> impl Fn(&f64, &f64) -> f64 {
//     move |a: &f64, b: &f64| f(a, b).max(0.0)
// }

// Unsign
// fn unsign(normalized: &mut ArrayBase<impl DataMut<Elem = f64>, impl Dimension>) {
//     normalized.map_inplace(|normalized| *normalized = normalized.max(0.0));
// }

/// Calculated
pub(in crate::app) type Calculated = FrameCache<Value, Calculator>;

/// Calculator
#[derive(Default)]
pub(in crate::app) struct Calculator;

/// Axis:
/// - 0: unnormalized, normalized;
/// - 1: fatty acids;
/// - 2: stereospecific numbering (1,2,3-TAGs; 1,2/2,3-DAGs; 2-MAGs; 1,3-DAGs).
impl ComputerMut<Key<'_>, Value> for Calculator {
    fn compute(&mut self, key: Key) -> Value {
        let Key { context } = key;
        let weights = LazyCell::new(|| {
            context
                .state
                .meta
                .formulas
                .iter()
                .map(|formula| formula.weight())
        });
        // Experimental
        let experimental = |unnormalized: &[f64]| {
            match context.settings.calculation.normalization {
                // s / ∑(s)
                Normalization::Mass => unnormalized.iter().copied().normalize(),
                // (s * m) / ∑(s * m)
                Normalization::Molar => unnormalized
                    .iter()
                    .zip(weights.clone())
                    .map(|(unnormalized, mass)| unnormalized * mass)
                    .normalize(),
                // s / ∑(s * m / 10.0)
                Normalization::Pchelkin => {
                    let sum = unnormalized
                        .iter()
                        .zip(weights.clone())
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

        let tags123 = experimental(&context.state.data.unnormalized.tags123);
        let mut dags1223 = experimental(&context.state.data.unnormalized.dags1223);
        let mut mags2 = experimental(&context.state.data.unnormalized.mags2);
        trace!(?tags123, ?dags1223, ?mags2);
        if let Source::Calculation = context.settings.calculation.sources.dag1223 {
            dags1223 = tags123
                .iter()
                .zip(&mags2)
                .map(|(tag123, mag2)| cast((3.0 * tag123 + mag2) / 4.0))
                .normalize();
        }
        if let Source::Calculation = context.settings.calculation.sources.mags2 {
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
        Value {
            tags123,
            dags1223,
            mags2,
            dags13,
        }
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
        self.context.state.meta.hash(state);
        self.context.state.data.unnormalized.hash(state);
    }
}

impl<'a> std::convert::From<&'a Context> for Key<'a> {
    fn from(value: &'a Context) -> Self {
        Self { context: value }
    }
}

// impl<'a, T: Borrow<Context> + 'a> std::convert::From<T> for Key<'a> {
//     fn from(value: T) -> Self {
//         Self {
//             context: value.borrow(),
//         }
//     }
// }

/// Value
type Value = Normalized;
