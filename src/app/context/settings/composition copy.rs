use super::{Order, Sort};
use egui::epaint::util::FloatOrd;
use itertools::Either;
use maplit::btreeset;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    hash::{Hash, Hasher},
};

pub(in crate::app) const UNIONS: [Union; 2] = [ECN, M];
pub(in crate::app) const COMPOSITIONS: [Composition; 6] = [TC, PTC, STC, SC, PSC, SSC];
pub(in crate::app) const TYPE_COMPOSITIONS: [Composition; 3] = [TC, PTC, STC];
pub(in crate::app) const SPECIES_COMPOSITIONS: [Composition; 3] = [SC, PSC, SSC];

pub(in crate::app) const ECN: Union = Union::EquivalentCarbonNumber;
pub(in crate::app) const M: Union = Union::Mass;

pub(in crate::app) const TC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Type,
};
pub(in crate::app) const PTC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Type,
};
pub(in crate::app) const STC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Type,
};
pub(in crate::app) const SSC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Species,
};
pub(in crate::app) const PSC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Species,
};
pub(in crate::app) const SC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Species,
};

/// Composition settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) adduct: OrderedFloat<f64>,
    pub(in crate::app) composition: Scope,
    pub(in crate::app) method: Method,
    pub(in crate::app) window: bool,

    pub(in crate::app) tree: Tree,
    pub(in crate::app) sort: Sort,
    pub(in crate::app) order: Order,

    pub(in crate::app) discrimination: Discrimination,
    pub(in crate::app) filter: Filter,
}

impl Settings {
    pub(in crate::app) fn groups(&self) -> Vec<Group> {
        self.tree
            .branches
            .iter()
            .flat_map(|branch| match branch {
                Branch::Union => {
                    Either::Left(self.tree.unions.iter().map(|&union| Group::Union(union)))
                }
                Branch::Composition => Either::Right(
                    self.tree
                        .compositions
                        .iter()
                        .map(|&composition| Group::Composition(composition)),
                ),
            })
            .collect()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resizable: false,

            percent: true,
            precision: 1,

            adduct: OrderedFloat(0.0),
            composition: Scope::Species,
            method: Method::VanderWal,
            window: false,

            tree: Default::default(),
            sort: Sort::Value,
            order: Order::Descending,

            discrimination: Default::default(),
            filter: Default::default(),
        }
    }
}

// /// Checkable
// #[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
// pub(in crate::app) struct Checkable<T> {
//     pub(in crate::app) value: T,
//     pub(in crate::app) checked: bool,
// }
// impl<T> Checkable<T> {
//     pub(in crate::app) fn new(value: T) -> Self {
//         Self {
//             value,
//             checked: false,
//         }
//     }
// }
// impl<T> From<Checkable<T>> for Option<T> {
//     fn from(Checkable { value, checked }: Checkable<T>) -> Self {
//         if checked {
//             Some(value)
//         } else {
//             None
//         }
//     }
// }

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Group {
    Union(Union),
    Composition(Composition),
}

// impl Group {
//     pub(in crate::app) fn text(self) -> &'static str {
//         match self {
//             ECN => "ECN",
//             M => "M",
//             TC => "TC",
//             PTC => "PTC",
//             STC => "STC",
//             SC => "SC",
//             PSC => "PSC",
//             SSC => "SSC",
//         }
//     }

//     pub(in crate::app) fn hover_text(self) -> &'static str {
//         match self {
//             ECN => "Group by ECN (Equivalent Carbon Number)",
//             M => "Group by M (Mass)",
//             TC => "Group by TC (Type Composition)",
//             PTC => "Group by PTC (Positional-Type Composition)",
//             STC => "Group by STC (Stereo-Type Composition)",
//             SC => "Group by SC (Species Composition)",
//             PSC => "Group by PSC (Positional-Species Composition)",
//             SSC => "No group SSC (Stereo-Species Composition)",
//         }
//     }
// }

/// Tree
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Tree {
    pub(in crate::app) branches: Vec<Branch>,
    pub(in crate::app) unions: BTreeSet<Union>,
    pub(in crate::app) compositions: BTreeSet<Composition>,
    pub(in crate::app) leafs: Composition,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            branches: vec![Branch::Union, Branch::Composition],
            unions: btreeset![],
            compositions: btreeset![],
            leafs: PSC,
        }
    }
}

/// Branch
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Branch {
    Union,
    Composition,
}

impl Branch {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Union => "Union",
            Self::Composition => "Composition",
        }
    }
}

/// Composition
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Composition {
    stereospecificity: Option<Stereospecificity>,
    scope: Scope,
}

impl Composition {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            TC => "TC",
            PTC => "PTC",
            STC => "STC",
            SC => "SC",
            PSC => "PSC",
            SSC => "SSC",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            TC => "Type composition",
            PTC => "Positional type composition",
            STC => "Stereo type composition",
            SC => "Species composition",
            PSC => "Positional species composition",
            SSC => "Stereo species composition",
        }
    }
}

/// Stereospecificity
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Stereospecificity {
    Positional,
    Stereo,
}

/// Scope
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Scope {
    Type,
    Species,
}

/// Union
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Union {
    EquivalentCarbonNumber,
    Mass,
}

impl Union {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::EquivalentCarbonNumber => "ECN",
            Self::Mass => "M",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::EquivalentCarbonNumber => "Equivalent Carbon Number",
            Self::Mass => "Mass",
        }
    }
}

// Desecrimination
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Discrimination {
    pub(in crate::app) sn1: BTreeSet<usize>,
    pub(in crate::app) sn2: BTreeSet<usize>,
    pub(in crate::app) sn3: BTreeSet<usize>,
}

impl Hash for Discrimination {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sn1.hash(state);
        self.sn2.hash(state);
        self.sn3.hash(state);
    }
}

/// Filter
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Filter {
    pub(in crate::app) psc: bool,
    pub(in crate::app) symmetrical: bool,
    pub(in crate::app) sn1: BTreeSet<usize>,
    pub(in crate::app) sn2: BTreeSet<usize>,
    pub(in crate::app) sn3: BTreeSet<usize>,
    pub(in crate::app) value: f64,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            psc: true,
            symmetrical: false,
            sn1: Default::default(),
            sn2: Default::default(),
            sn3: Default::default(),
            value: Default::default(),
        }
    }
}

impl Hash for Filter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.psc.hash(state);
        self.sn1.hash(state);
        self.sn2.hash(state);
        self.sn3.hash(state);
        self.value.ord().hash(state);
    }
}

/// Method
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Method {
    Gunstone,
    VanderWal,
}

impl Method {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Gunstone => "Gunstone",
            Self::VanderWal => "Vander Wal",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Gunstone => "Calculate by Gunstone's theory",
            Self::VanderWal => "Calculate by Vander Wal's theory",
        }
    }
}
