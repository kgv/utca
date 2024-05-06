use super::{Order, Sort};
use crate::acylglycerol::Tag;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Composition settings
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) set: IndexSet<Tag<String>>,
    pub(in crate::app) sort: Option<Sort>,
    pub(in crate::app) column: usize,
    pub(in crate::app) order: Order,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resizable: false,
            percent: true,
            precision: 1,

            set: Default::default(),
            sort: None,
            column: 0,
            order: Order::Descending,
        }
    }
}

impl Hash for Settings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.resizable.hash(state);
        self.percent.hash(state);
        self.precision.hash(state);
        self.set.as_slice().hash(state);
        self.sort.hash(state);
        self.column.hash(state);
        self.order.hash(state);
    }
}

// /// Group
// #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
// pub(in crate::app) enum Group {
//     Composition(composition::Composition),
//     Cmn,
// }

// impl Group {
//     pub(in crate::app) fn text(self) -> &'static str {
//         match self {
//             Self::Composition(group) => group.text(),
//             Self::Cmn => "CMN",
//         }
//     }

//     pub(in crate::app) fn hover_text(self) -> &'static str {
//         match self {
//             Self::Composition(group) => group.hover_text(),
//             Self::Cmn => "Group by CMN (Comparative Major Number)",
//         }
//     }
// }
