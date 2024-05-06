use super::composition::{self, Count};
use crate::acylglycerol::Tag;
use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Compared data
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Compared {
    pub data: IndexMap<Tag<String>, Vec<Option<OrderedFloat<f64>>>>,
    pub meta: Vec<Meta>,
}

impl Hash for Compared {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.as_slice().hash(state);
        self.meta.hash(state);
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub struct Meta {
    pub count: Count,
    pub sum: Option<OrderedFloat<f64>>,
}

/// Data
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Data {
    pub(in crate::app) tag: Tag<String>,
    pub(in crate::app) values: Vec<Option<OrderedFloat<f64>>>,
}

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Group {
    Composition(composition::Group),
    Cmn(u32),
}
