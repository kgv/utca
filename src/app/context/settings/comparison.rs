use super::{composition, Order, Sort};
use serde::{Deserialize, Serialize};

pub(in crate::app) const CMN: Group = Group::Cmn;

pub(in crate::app) const NC: Group = Group::Composition(composition::ECNC);
pub(in crate::app) const PNC: Group = Group::Composition(composition::PECNC);
pub(in crate::app) const SNC: Group = Group::Composition(composition::SECNC);

pub(in crate::app) const MC: Group = Group::Composition(composition::MC);
pub(in crate::app) const PMC: Group = Group::Composition(composition::PMC);
pub(in crate::app) const SMC: Group = Group::Composition(composition::SMC);

pub(in crate::app) const TC: Group = Group::Composition(composition::TC);
pub(in crate::app) const PTC: Group = Group::Composition(composition::PTC);
pub(in crate::app) const STC: Group = Group::Composition(composition::STC);

pub(in crate::app) const SC: Group = Group::Composition(composition::SC);
pub(in crate::app) const PSC: Group = Group::Composition(composition::PSC);
pub(in crate::app) const SSC: Group = Group::Composition(composition::SSC);

/// Composition settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) groups: Vec<Group>,
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
            groups: vec![
                // Checkable::new(CMN),
                // Checkable::new(ECN),
                // Checkable::new(M),
                // Checkable::new(PTC),
                // Checkable::new(STC),
                // Checkable::new(TC),
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
    Composition(composition::Composition),
    Cmn,
}

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
