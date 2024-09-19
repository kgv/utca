use crate::{acylglycerol::Tag, app::panes::comparison::settings::Settings, utils::DataFrameExt};
use egui::util::cache::{ComputerMut, FrameCache};
use indexmap::IndexMap;
use itertools::Either::{Left, Right};
use ordered_float::OrderedFloat;
use polars::prelude::*;
use std::{
    cmp::Reverse,
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Comparison computer
pub(in crate::app) type Computed = FrameCache<Value, Computer>;

/// Comparison computer
#[derive(Default)]
pub(in crate::app) struct Computer;

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        key.data_frame.clone()
    }
}

/// Filter key
#[derive(Clone, Copy, Debug)]
pub(in crate::app) struct Key<'a> {
    pub(in crate::app) data_frame: &'a DataFrame,
    pub(in crate::app) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for label in self.data_frame.str("Label") {
            label.hash(state);
        }
        // self.context.state.index.hash(state);
        self.settings.hash(state);
    }
}

type Value = DataFrame;
