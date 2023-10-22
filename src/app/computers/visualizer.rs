use crate::{
    acylglycerol::Tag,
    app::context::{
        settings::{
            Group::{Ecn, Occurrence, Ptc},
            Order, Sort,
        },
        state::{Group, Visualized as Value},
        Context,
    },
};
use egui::{
    epaint::util::FloatOrd,
    util::cache::{ComputerMut, FrameCache},
};
use indexmap::IndexMap;
use itertools::Itertools;
use std::{
    cmp::{max, min, Reverse},
    hash::{Hash, Hasher},
    iter::repeat,
};

/// Visualized
pub(in crate::app) type Visualized = FrameCache<Value, Visualizer>;

/// Visualizer
#[derive(Default)]
pub(in crate::app) struct Visualizer;

impl ComputerMut<Key<'_>, Value> for Visualizer {
    fn compute(&mut self, key: Key) -> Value {
        let Key { context } = key;
        let visualized = context
            .state
            .entry()
            .data
            .composed
            .filtered
            .values()
            .flatten()
            .collect();
        // .sorted_by_key(|&value| Reverse(value.ord()))
        Value(visualized)
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

impl<'a> From<&'a Context> for Key<'a> {
    fn from(value: &'a Context) -> Self {
        Self { context: value }
    }
}

// /// Map
// type Map = IndexMap<Option<Group>, IndexMap<Tag<usize>, f64>>;
