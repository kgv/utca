use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
};

/// Composition settings
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) composition: Option<Positional>,
    pub(in crate::app) filter: Filter,
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
    pub(in crate::app) resizable: bool,
    pub(in crate::app) sort: Sort,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            composition: None,
            filter: Default::default(),
            percent: true,
            precision: 5,
            resizable: false,
            sort: Sort::Key(Order::Ascending),
        }
    }
}

/// Positional сomposition
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Positional {
    /// Positional-species composition (PSC)
    Species,
    /// Positional-type composition (PTC)
    Type,
}

impl Display for Positional {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Species if f.alternate() => f.write_str("PSC"),
            Self::Type if f.alternate() => f.write_str("PTC"),
            Self::Species => f.write_str("species"),
            Self::Type => f.write_str("type"),
        }
    }
}

/// Filter
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Filter {
    pub(in crate::app) part: f64,
    pub(in crate::app) sn13: HashSet<usize>,
    pub(in crate::app) sn2: HashSet<usize>,
}

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Sort {
    Key(Order),
    Value(Order),
}

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Key(order) => write!(f, "Sort by key {order}"),
            Self::Value(order) => write!(f, "Sort by value {order}"),
        }
    }
}

/// Order
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Order {
    Ascending,
    Descending,
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Ascending => f.write_str("ascending"),
            Self::Descending => f.write_str("descending"),
        }
    }
}
