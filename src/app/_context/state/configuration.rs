use crate::fatty_acid::Data;
use serde::{Deserialize, Serialize};
use std::{
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

/// Configured data
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Configured(Vec<Data>);

impl Configured {
    pub(in crate::app) fn len(&self) -> usize {
        self.0.len()
    }

    pub(in crate::app) fn push(&mut self, data: Data) {
        self.0.push(data);
    }

    pub(in crate::app) fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }

    pub(in crate::app) fn swap(&mut self, from: usize, to: usize) {
        self.0.swap(from, to);
    }

    pub(in crate::app) fn tags123(&self) -> impl Iterator<Item = &f64> {
        self.0.iter().map(|Data { tag123, .. }| tag123)
    }

    pub(in crate::app) fn dags1223(&self) -> impl Iterator<Item = &f64> {
        self.0.iter().map(|Data { dag1223, .. }| dag1223)
    }

    pub(in crate::app) fn mags2(&self) -> impl Iterator<Item = &f64> {
        self.0.iter().map(|Data { mag2, .. }| mag2)
    }
}

impl Extend<Data> for Configured {
    fn extend<T: IntoIterator<Item = Data>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl Index<usize> for Configured {
    type Output = Data;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Configured {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<'a> IntoIterator for &'a Configured {
    type Item = &'a Data;

    type IntoIter = Iter<'a, Data>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Configured {
    type Item = &'a mut Data;

    type IntoIter = IterMut<'a, Data>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
