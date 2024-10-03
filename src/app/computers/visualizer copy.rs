use crate::{
    app::context::{
        settings::visualization::{Source, X},
        Context,
    },
    r#const::relative_atomic_mass::C3H2,
    tree::Leaf,
};
use egui::{
    emath::round_to_decimals,
    epaint::util::{FloatOrd, OrderedFloat},
    util::cache::{ComputerMut, FrameCache},
};
use itertools::Itertools;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

/// Visualized
pub(in crate::app) type Visualized = FrameCache<Value, Visualizer>;

/// Visualizer
#[derive(Default)]
pub(in crate::app) struct Visualizer;

impl ComputerMut<Key<'_>, Value> for Visualizer {
    fn compute(&mut self, key: Key) -> Value {
        let Key { context } = key;
        match context.settings.visualization.source {
            Source::Composition => {}
            Source::Comparison => {}
        }

        match context.settings.visualization.axes.x {
            X::Mass => context
                .state
                .entry()
                .data
                .composed
                .composition(context.settings.composition.method)
                .leaves()
                .map(|Leaf { data }| {
                    let key = round_to_decimals(
                        C3H2 + context.mass(data.tag).sum() + context.settings.composition.adduct.0,
                        5,
                    );
                    let name = context.species(data.tag).to_string();
                    let value = data.value.0;
                    (key.ord(), (name, value))
                })
                .into_group_map(),
            X::EquivalentCarbonNumber => context
                .state
                .entry()
                .data
                .composed
                .composition(context.settings.composition.method)
                .leaves()
                .map(|Leaf { data }| {
                    let key = context.ecn(data.tag).sum() as f64;
                    let name = context.species(data.tag).to_string();
                    let value = data.value.0;
                    (key.ord(), (name, value))
                })
                .into_group_map(),
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
        self.context.settings.visualization.hash(state);
        self.context.state.entry().meta.hash(state);
        self.context.state.entry().data.composed.hash(state);
    }
}

impl<'a> std::convert::From<&'a Context> for Key<'a> {
    fn from(value: &'a Context) -> Self {
        Self { context: value }
    }
}

/// Value
type Value = HashMap<OrderedFloat<f64>, Vec<(String, f64)>>;
