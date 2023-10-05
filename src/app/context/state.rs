use crate::{
    acylglycerol::Tag,
    cu::Saturation,
    parsers::toml::{FattyAcid, Parsed},
};
use egui::epaint::util::FloatOrd;
use indexmap::IndexMap;
use itertools::izip;
use molecule::Counter;
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    ops::Range,
};

// State
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct State {
    pub(in crate::app) meta: Meta,
    pub(in crate::app) data: Data,
}

impl State {
    pub(in crate::app) fn fatty_acids(&self) -> Vec<FattyAcid> {
        izip!(self.meta.zip(), self.data.unnormalized.zip())
            .map(|((label, formula), (&tag123, &dag1223, &mag2))| {
                FattyAcid::new(label.clone(), formula.clone(), tag123, dag1223, mag2)
            })
            .collect()
    }

    pub(in crate::app) fn add(&mut self) {
        self.meta.labels.push(Default::default());
        self.meta.formulas.push(Default::default());
        self.data.unnormalized.tags123.push(Default::default());
        self.data.unnormalized.dags1223.push(Default::default());
        self.data.unnormalized.mags2.push(Default::default());
    }

    pub(in crate::app) fn del(&mut self, index: usize) {
        self.meta.labels.remove(index);
        self.meta.formulas.remove(index);
        self.data.unnormalized.tags123.remove(index);
        self.data.unnormalized.dags1223.remove(index);
        self.data.unnormalized.mags2.remove(index);
    }
}

impl From<Parsed> for State {
    fn from(value: Parsed) -> Self {
        let Parsed { name, fatty_acids } = value;
        let (labels, (formulas, (tags123, (dags1223, mags2)))): (
            Vec<_>,
            (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))),
        ) = fatty_acids
            .into_iter()
            .map(|fatty_acid| {
                (
                    fatty_acid.label,
                    (
                        fatty_acid.formula,
                        (
                            fatty_acid.data.tag123,
                            (fatty_acid.data.dag1223, fatty_acid.data.mag2),
                        ),
                    ),
                )
            })
            .unzip();
        Self {
            meta: Meta {
                name,
                labels,
                formulas,
            },
            data: Data {
                unnormalized: Unnormalized {
                    tags123,
                    dags1223,
                    mags2,
                },
                ..Default::default()
            },
        }
    }
}

/// Meta
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Meta {
    pub(in crate::app) name: String,
    pub(in crate::app) labels: Vec<String>,
    pub(in crate::app) formulas: Vec<Counter>,
}

impl Meta {
    pub(in crate::app) fn zip(&self) -> impl Iterator<Item = (&String, &Counter)> {
        izip!(&self.labels, &self.formulas,)
    }
}

/// Data
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Data {
    pub(in crate::app) unnormalized: Unnormalized,
    pub(in crate::app) normalized: Normalized,
    pub(in crate::app) composed: Composed,
}

/// Unnormalized data
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Unnormalized {
    pub(in crate::app) tags123: Vec<f64>,
    pub(in crate::app) dags1223: Vec<f64>,
    pub(in crate::app) mags2: Vec<f64>,
}

impl Unnormalized {
    pub(in crate::app) fn zip(&self) -> impl Iterator<Item = (&f64, &f64, &f64)> {
        izip!(&self.tags123, &self.dags1223, &self.mags2)
    }

    pub(in crate::app) fn iter(&self) -> [&[f64]; 3] {
        [&self.tags123, &self.dags1223, &self.mags2]
    }

    pub(in crate::app) fn iter_mut(&mut self) -> [&mut [f64]; 3] {
        [&mut self.tags123, &mut self.dags1223, &mut self.mags2]
    }
}

impl Hash for Unnormalized {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for tag123 in &self.tags123 {
            tag123.ord().hash(state);
        }
        for dag1223 in &self.dags1223 {
            dag1223.ord().hash(state);
        }
        for mag2 in &self.mags2 {
            mag2.ord().hash(state);
        }
    }
}

/// Normalized data
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Normalized {
    pub(in crate::app) tags123: Vec<f64>,
    pub(in crate::app) dags1223: Vec<f64>,
    pub(in crate::app) mags2: Vec<f64>,
    pub(in crate::app) dags13: Vec<f64>,
}

impl Normalized {
    pub(in crate::app) fn zip(&self) -> impl Iterator<Item = (&f64, &f64, &f64, &f64)> {
        izip!(&self.tags123, &self.dags1223, &self.mags2, &self.dags13)
    }
}

impl Hash for Normalized {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for tag123 in &self.tags123 {
            tag123.ord().hash(state);
        }
        for dag1223 in &self.dags1223 {
            dag1223.ord().hash(state);
        }
        for mag2 in &self.mags2 {
            mag2.ord().hash(state);
        }
        for dag13 in &self.dags13 {
            dag13.ord().hash(state);
        }
    }
}

/// Composed data
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Composed {
    pub(in crate::app) unfiltered: IndexMap<Tag<usize>, f64>,
    pub(in crate::app) filtered: IndexMap<Tag<usize>, f64>,
    pub(in crate::app) grouped: IndexMap<Tag<Saturation>, Range<usize>>,
}

impl Hash for Composed {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in &self.unfiltered {
            (key, value.ord()).hash(state);
        }
        for (key, value) in &self.filtered {
            (key, value.ord()).hash(state);
        }
        for (key, value) in &self.grouped {
            (key, value).hash(state);
        }
    }
}
