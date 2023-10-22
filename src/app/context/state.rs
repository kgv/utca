use crate::{acylglycerol::Tag, parsers::toml::FattyAcid};
use egui::epaint::util::FloatOrd;
use indexmap::IndexMap;
use itertools::izip;
use molecule::{Counter, Saturation};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

/// State
#[derive(Debug, Deserialize, Hash, Serialize)]
pub(in crate::app) struct State {
    pub(in crate::app) entries: Vec<Entry>,
    pub(in crate::app) index: usize,
    pub(in crate::app) compared: Compared,
}

impl State {
    pub(in crate::app) fn entry(&self) -> &Entry {
        &self.entries[self.index]
    }

    pub(in crate::app) fn entry_mut(&mut self) -> &mut Entry {
        &mut self.entries[self.index]
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            entries: vec![Default::default()],
            index: 0,
            compared: Default::default(),
        }
    }
}

/// Entry
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Entry {
    pub(in crate::app) meta: Meta,
    pub(in crate::app) data: Data,
}

impl Entry {
    pub(in crate::app) fn fatty_acids(&self) -> Vec<FattyAcid> {
        self.meta
            .zip()
            .zip(self.data.unnormalized.zip())
            .map(|((label, formula), (tag123, dag1223, mag2))| {
                FattyAcid::new(label.clone(), formula.clone(), *tag123, *dag1223, *mag2)
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

    pub(in crate::app) fn len(&self) -> usize {
        let len = self.meta.labels.len();
        assert_eq!(len, self.meta.formulas.len());
        assert_eq!(len, self.data.unnormalized.tags123.len());
        assert_eq!(len, self.data.unnormalized.dags1223.len());
        assert_eq!(len, self.data.unnormalized.mags2.len());
        len
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
    pub(in crate::app) unfiltered: IndexMap<Option<Group>, IndexMap<Tag<usize>, f64>>,
    pub(in crate::app) filtered: IndexMap<Option<Group>, IndexMap<Tag<usize>, f64>>,
}

impl Composed {
    pub(in crate::app) fn unfiltered(&self, tag: &Tag<usize>) -> Option<f64> {
        self.unfiltered
            .values()
            .find_map(|value| value.get(tag))
            .copied()
    }
}

impl Hash for Composed {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, values) in &self.unfiltered {
            key.hash(state);
            for (key, value) in values {
                key.hash(state);
                value.ord().hash(state);
            }
        }
        for (key, values) in &self.filtered {
            key.hash(state);
            for (key, value) in values {
                key.hash(state);
                value.ord().hash(state);
            }
        }
    }
}

/// Compared data
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Compared(
    pub(in crate::app) IndexMap<Option<Group>, IndexMap<Tag<usize>, Vec<Option<f64>>>>,
);

impl Hash for Compared {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in &self.0 {
            key.hash(state);
            for (key, values) in value {
                key.hash(state);
                for value in values {
                    value.map(FloatOrd::ord).hash(state);
                }
            }
        }
    }
}

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Group {
    Ecn(usize),
    Ptc(Tag<Saturation>),
    Occurrence(usize),
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Group::Ecn(ecn) => f.write_fmt(format_args!("{ecn}")),
            Group::Ptc(r#type) => f.write_fmt(format_args!("{type}")),
            Group::Occurrence(occurrence) => f.write_fmt(format_args!("{occurrence}")),
        }
    }
}
