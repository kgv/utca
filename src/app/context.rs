use crate::acylglycerol::Tag;
use indexmap::IndexMap;
use molecule::Counter;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    default::default,
    hash::{Hash, Hasher},
};

/// Context
#[derive(Debug, Default, Deserialize, Serialize)]
pub(super) struct Context {
    pub(super) labels: Vec<String>,
    pub(super) formulas: Vec<Counter>,
    pub(super) unnormalized: Unnormalized,
    pub(super) normalized: Normalized,
    pub(super) composed: IndexMap<Tag<usize>, f64>,
}

impl Context {
    pub(super) fn push_default(&mut self) {
        self.labels.push(default());
        self.formulas.push(default());
        self.unnormalized.tags123.push(default());
        self.unnormalized.dags1223.push(default());
        self.unnormalized.mags2.push(default());
    }

    pub(super) fn remove(&mut self, index: usize) {
        self.labels.remove(index);
        self.formulas.remove(index);
        self.unnormalized.tags123.remove(index);
        self.unnormalized.dags1223.remove(index);
        self.unnormalized.mags2.remove(index);
    }
}

/// Unnormalized
#[derive(Debug, Default, Deserialize, Serialize)]
pub(super) struct Unnormalized {
    pub(super) tags123: Vec<f64>,
    pub(super) dags1223: Vec<f64>,
    pub(super) mags2: Vec<f64>,
}

impl Unnormalized {
    pub(super) fn iter(&self) -> [&[f64]; 3] {
        [&self.tags123, &self.dags1223, &self.mags2]
    }

    pub(super) fn iter_mut(&mut self) -> [&mut [f64]; 3] {
        [&mut self.tags123, &mut self.dags1223, &mut self.mags2]
    }
}

impl Hash for Unnormalized {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for &tag123 in &self.tags123 {
            OrderedFloat(tag123).hash(state);
        }
        for &dag1223 in &self.dags1223 {
            OrderedFloat(dag1223).hash(state);
        }
        for &mag2 in &self.mags2 {
            OrderedFloat(mag2).hash(state);
        }
    }
}

/// Normalized
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(super) struct Normalized {
    pub(super) tags123: Vec<f64>,
    pub(super) dags1223: Vec<f64>,
    pub(super) mags2: Vec<f64>,
    pub(super) dags13: Vec<f64>,
}
