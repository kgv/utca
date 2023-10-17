use crate::{
    app::context::{
        settings::calculation::{From, Normalization, Signedness, Source},
        state::Normalized,
        Context,
    },
    utils::Normalize,
};
use egui::util::cache::{ComputerMut, FrameCache};
use std::cell::LazyCell;
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
        let context = key;
        let tags123 = &context.state.data.unnormalized.tags123;
        let dags1223 = &context.state.data.unnormalized.dags1223;
        let mags2 = &context.state.data.unnormalized.mags2;
        let formulas = &context.state.meta.formulas;
        let weights = LazyCell::new(|| formulas.iter().map(|formula| formula.weight()));
        let normalization = context.settings.calculation.normalization;
        let signedness = context.settings.calculation.signedness;
        let sources = context.settings.calculation.sources;

        // Experimental
        let experimental = |unnormalized: &[f64]| {
            match normalization {
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
        let cast = |value: f64| match signedness {
            Signedness::Signed => value,
            Signedness::Unsigned => value.max(0.0),
        };

        let tags123 = experimental(tags123);
        let mut dags1223 = experimental(dags1223);
        let mut mags2 = experimental(mags2);
        trace!(?tags123, ?dags1223, ?mags2);
        if let Source::Calculation = sources.dag1223 {
            dags1223 = tags123
                .iter()
                .zip(&mags2)
                .map(|(tag123, mag2)| cast((3.0 * tag123 + mag2) / 4.0))
                .normalize();
        }
        if let Source::Calculation = sources.mags2 {
            mags2 = tags123
                .iter()
                .zip(&dags1223)
                .map(|(tag123, dag1223)| cast(4.0 * dag1223 - 3.0 * tag123))
                .normalize();
        }
        trace!(?dags1223, ?mags2);
        let dags13 = match sources.dag13 {
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
type Key<'a> = &'a Context;

/// Value
type Value = Normalized;
