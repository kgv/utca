use self::{
    calculation::Calculated, comparison::Compared, composition::Composed, configuration::Configured,
};
use crate::fatty_acid::{self, FattyAcid};
use itertools::izip;
use molecule::Counter;
use serde::{Deserialize, Serialize};
use std::{iter::zip, sync::Arc};

/// State
#[derive(Debug, Deserialize, Hash, Serialize)]
pub(in crate::app) struct State {
    pub(in crate::app) entries: Vec<Entry<Meta, Data>>,
    pub(in crate::app) index: usize,
    pub(in crate::app) compared: Arc<Compared>,
}

impl State {
    pub(in crate::app) fn entry(&self) -> &Entry<Meta, Data> {
        &self.entries[self.index]
    }

    pub(in crate::app) fn entry_mut(&mut self) -> &mut Entry<Meta, Data> {
        &mut self.entries[self.index]
    }

    pub(in crate::app) fn configured_mut(
        &mut self,
    ) -> impl Iterator<Item = (&mut String, &mut Counter, &mut fatty_acid::Data)> {
        let entry = self.entry_mut();
        izip!(
            &mut entry.meta.labels,
            &mut entry.meta.formulas,
            &mut entry.data.configured,
        )
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
pub(in crate::app) struct Entry<M, D> {
    pub(in crate::app) meta: M,
    pub(in crate::app) data: D,
}

impl Entry<Meta, Data> {
    pub(in crate::app) fn fatty_acids(&self) -> Vec<FattyAcid> {
        zip(self.meta.zip(), &self.data.configured)
            .map(|((label, formula), &data)| FattyAcid {
                label: label.clone(),
                formula: formula.clone(),
                data,
            })
            .collect()
    }

    pub(in crate::app) fn add(&mut self) {
        self.meta.labels.push(Default::default());
        self.meta.formulas.push(Default::default());
        self.data.configured.push(Default::default());
    }

    pub(in crate::app) fn del(&mut self, index: usize) {
        self.meta.labels.remove(index);
        self.meta.formulas.remove(index);
        self.data.configured.remove(index);
    }

    pub(in crate::app) fn swap(&mut self, from: usize, to: usize) {
        self.meta.labels.swap(from, to);
        self.meta.formulas.swap(from, to);
        self.data.configured.swap(from, to);
    }

    pub(in crate::app) fn len(&self) -> usize {
        let len = self.meta.labels.len();
        assert_eq!(len, self.meta.formulas.len());
        assert_eq!(len, self.data.configured.len());
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
        zip(&self.labels, &self.formulas)
    }
}

/// Data
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Data {
    pub(in crate::app) configured: Configured,
    pub(in crate::app) calculated: Arc<Calculated>,
    pub(in crate::app) composed: Arc<Composed>,
}

pub(in crate::app) mod calculation;
pub(in crate::app) mod comparison;
pub(in crate::app) mod composition;
pub(in crate::app) mod configuration;
