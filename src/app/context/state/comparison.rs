use super::composition::{self, Merge, Value};
use crate::{acylglycerol::Tag, tree::Tree};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result, Write},
    iter::zip,
};

/// Compared data
pub(in crate::app) type Compared = Tree<Meta, Data>;

/// Data
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Data {
    pub(in crate::app) tag: Tag<usize>,
    pub(in crate::app) values: Vec<Option<OrderedFloat<f64>>>,
}

/// Meta
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Meta {
    pub(in crate::app) group: Option<Group>,
    pub(in crate::app) count: Count,
    pub(in crate::app) counts: Vec<Count>,
    pub(in crate::app) values: Vec<Option<Value>>,
}

impl Meta {
    pub(in crate::app) fn with_length(length: usize) -> Self {
        Self {
            counts: vec![Default::default(); length],
            values: vec![Default::default(); length],
            ..Default::default()
        }
    }
}

impl Merge<&Self> for Meta {
    fn merge(&mut self, other: &Self) {
        self.count.merge(other.count);
        for (count, &other) in zip(&mut self.counts, &other.counts) {
            count.merge(other);
        }
        for (value, &other) in zip(&mut self.values, &other.values) {
            value.merge(other);
        }
    }
}

/// Count
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Count {
    pub(in crate::app) branches: usize,
    pub(in crate::app) leafs: usize,
}

impl Merge for Count {
    fn merge(&mut self, other: Self) {
        self.branches += 1;
        self.leafs += other.leafs;
    }
}

impl Display for Count {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.branches != 0 {
            write!(f, "{} (", self.branches)?;
        }
        write!(f, "{}", self.leafs)?;
        if self.branches != 0 {
            f.write_char(')')?;
        }
        Ok(())
    }
}

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Group {
    Composition(composition::Group),
    Cmn(u32),
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Group::Composition(composition) => Display::fmt(&composition, f),
            Group::Cmn(cmn) => f.write_fmt(format_args!("{cmn}")),
        }
    }
}
