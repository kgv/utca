use super::{Order, Sort};
use egui::epaint::util::FloatOrd;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    hash::{Hash, Hasher},
};

/// Composition settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) ecn: bool,
    pub(in crate::app) mass: bool,
    pub(in crate::app) mirror: bool,
    pub(in crate::app) symmetrical: bool,

    pub(in crate::app) group: Option<Group>,
    pub(in crate::app) sort: Sort,
    pub(in crate::app) order: Order,

    pub(in crate::app) filter: Filter,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resizable: false,

            percent: true,
            precision: 1,

            ecn: false,
            mass: false,
            mirror: true,
            symmetrical: false,

            group: None,
            sort: Sort::Value,
            order: Order::Descending,

            filter: Default::default(),
        }
    }
}

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Group {
    Ecn,
    Ptc,
}

impl Group {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Ecn => "ECN",
            Self::Ptc => "PTC",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Ecn => "Group by ECN (Equivalent Carbon Number)",
            Self::Ptc => "Group by PTC (Positional-Type Composition)",
        }
    }
}

/// Filter
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Filter {
    pub(in crate::app) sn1: BTreeSet<usize>,
    pub(in crate::app) sn2: BTreeSet<usize>,
    pub(in crate::app) sn3: BTreeSet<usize>,
    pub(in crate::app) value: f64,
}

impl Hash for Filter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sn1.hash(state);
        self.sn2.hash(state);
        self.sn3.hash(state);
        self.value.ord().hash(state);
    }
}
