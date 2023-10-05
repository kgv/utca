use egui::epaint::util::FloatOrd;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

/// Composition settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) filter: Filter,
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
    pub(in crate::app) resizable: bool,
    pub(in crate::app) ecn: bool,
    pub(in crate::app) mass: bool,
    pub(in crate::app) positional: Option<Positional>,
    pub(in crate::app) sort: Sort,
    pub(in crate::app) order: Order,
    pub(in crate::app) mirror: bool,
}

impl Settings {
    pub(in crate::app) fn is_positional_type(&self) -> bool {
        self.positional != Some(Positional::Species)
    }

    pub(in crate::app) fn is_positional_species(&self) -> bool {
        self.positional != Some(Positional::Type)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            filter: Default::default(),
            mirror: false,
            percent: true,
            precision: 1,
            resizable: false,
            ecn: false,
            mass: false,
            positional: None,
            sort: Sort::Value,
            order: Order::Descending,
        }
    }
}

/// Positional composition
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Positional {
    /// Positional-type composition (PTC)
    Type,
    /// Positional-species composition (PSC)
    Species,
}

/// Filter
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Filter {
    pub(in crate::app) value: f64,
    pub(in crate::app) sn13: BTreeSet<usize>,
    pub(in crate::app) sn2: BTreeSet<usize>,
}

impl Hash for Filter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.ord().hash(state);
        self.sn13.hash(state);
        self.sn13.hash(state);
    }
}

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Sort {
    Key,
    Value,
}

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Key => f.write_str("Key"),
            Self::Value => f.write_str("Value"),
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
            Self::Ascending => f.write_str("Ascending"),
            Self::Descending => f.write_str("Descending"),
        }
    }
}
