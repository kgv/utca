use crate::{acylglycerol::Tag, tree::Tree};
use molecule::Saturation;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
};

/// Composed data
pub(in crate::app) type Composed = Tree<Meta, Data>;

/// Data
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Data {
    pub(in crate::app) tag: Tag<usize>,
    pub(in crate::app) value: OrderedFloat<f64>,
}

/// Meta
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Meta {
    pub(in crate::app) group: Option<Group>,
    pub(in crate::app) count: Count,
    pub(in crate::app) value: Value,
}

impl Merge for Meta {
    fn merge(&mut self, other: Self) {
        self.count.merge(other.count);
        self.value.merge(other.value);
    }
}

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Group {
    Ecn(usize),
    Ptc(Tag<Saturation>),
}

// impl Statable for Group {
//     type Output = [Option<Group>; 2];
//     fn state(context: &Context, tag: Tag<usize>) -> Self::Output {
//         context.settings.composition.groups.optional().map(|group| {
//             Some(match group? {
//                 ECN => Group::Ecn(context.ecn(tag).sum()),
//                 PTC => Group::Ptc(context.ptc(tag)),
//             })
//         })
//     }
// }

impl Display for Group {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Ecn(ecn) => Display::fmt(ecn, f),
            Self::Ptc(ptc) => Display::fmt(ptc, f),
        }
    }
}

/// Count
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Count {
    pub(in crate::app) filtered: usize,
    pub(in crate::app) unfiltered: usize,
}

impl Merge for Count {
    fn merge(&mut self, other: Self) {
        self.filtered += other.filtered;
        self.unfiltered += other.unfiltered;
    }
}

impl Merge<bool> for Count {
    fn merge(&mut self, other: bool) {
        if other {
            self.filtered += 1;
        }
        self.unfiltered += 1;
    }
}

impl From<Rounded<OrderedFloat<f64>>> for Value {
    fn from(value: Rounded<OrderedFloat<f64>>) -> Self {
        Self {
            rounded: value.rounded().into(),
            unrounded: value.unrounded,
        }
    }
}

/// Value
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, Serialize)]
pub(in crate::app) struct Value {
    pub(in crate::app) rounded: OrderedFloat<f64>,
    pub(in crate::app) unrounded: OrderedFloat<f64>,
}

impl Merge for Value {
    fn merge(&mut self, other: Self) {
        self.rounded += other.rounded;
        self.unrounded += other.unrounded;
    }
}

impl Merge<Rounded<OrderedFloat<f64>>> for Value {
    fn merge(&mut self, other: Rounded<OrderedFloat<f64>>) {
        self.rounded += other.rounded();
        self.unrounded += other.unrounded;
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rounded
            .partial_cmp(&other.rounded)
            .and(self.unrounded.partial_cmp(&other.unrounded))
    }
}

/// Rounded
#[derive(Clone, Copy, Debug, Default)]
pub(in crate::app) struct Rounded<T> {
    pub(in crate::app) unrounded: T,
    pub(in crate::app) precision: usize,
}

impl<T> Rounded<T> {
    pub(in crate::app) fn new(unrounded: T, precision: usize) -> Self {
        Self {
            unrounded,
            precision,
        }
    }
}

impl Rounded<OrderedFloat<f64>> {
    fn rounded(&self) -> f64 {
        let power = 10.0f64.powi(self.precision as _);
        (self.unrounded * power).round() / power
    }
}

/// Merge
pub(in crate::app) trait Merge<T = Self> {
    fn merge(&mut self, other: T);
}

impl<T: Merge> Merge for Option<T> {
    fn merge(&mut self, other: Self) {
        if let Some(other) = other {
            match self {
                Some(value) => value.merge(other),
                value => *value = Some(other),
            }
        }
    }
}
