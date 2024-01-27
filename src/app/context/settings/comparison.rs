use super::{
    composition::{self, Checkable},
    Order, Sort,
};
use serde::{Deserialize, Serialize};

pub(in crate::app) const CMN: Group = Group::Cmn;
pub(in crate::app) const ECN: Group = Group::Composition(composition::ECN);
pub(in crate::app) const M: Group = Group::Composition(composition::M);
pub(in crate::app) const PTC: Group = Group::Composition(composition::PTC);
pub(in crate::app) const STC: Group = Group::Composition(composition::STC);
pub(in crate::app) const TC: Group = Group::Composition(composition::TC);

/// Composition settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) groups: [Checkable<Group>; 6],
    pub(in crate::app) sort: Sort,
    pub(in crate::app) column: usize,
    pub(in crate::app) order: Order,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resizable: false,
            percent: true,
            precision: 1,
            groups: [
                Checkable::new(CMN),
                Checkable::new(ECN),
                Checkable::new(M),
                Checkable::new(PTC),
                Checkable::new(STC),
                Checkable::new(TC),
            ],
            sort: Sort::Key,
            column: 0,
            order: Order::Descending,
        }
    }
}

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Group {
    Composition(composition::Group),
    Cmn,
}

impl Group {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Composition(group) => group.text(),
            Self::Cmn => "CMN",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Composition(group) => group.hover_text(),
            Self::Cmn => "Group by CMN (Comparative Major Number)",
        }
    }
}
