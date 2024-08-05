use super::{Order, Sort};
use crate::acylglycerol::Stereospecificity;
use egui::epaint::util::FloatOrd;
use indexmap::{indexmap, IndexMap, IndexSet};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    hash::{Hash, Hasher},
    sync::LazyLock,
};

pub(in crate::app) static BRANCHES: LazyLock<IndexMap<Scope, Vec<Composition>>> =
    LazyLock::new(|| {
        indexmap! {
            Scope::EquivalentCarbonNumber => vec![NC, PNC, SNC],
            Scope::Mass => vec![MC, PMC, SMC],
            Scope::Type => vec![TC, PTC, STC],
            Scope::Species => vec![SC, PSC],
        }
    });

pub(in crate::app) const NC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::EquivalentCarbonNumber,
};
pub(in crate::app) const PNC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::EquivalentCarbonNumber,
};
pub(in crate::app) const SNC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::EquivalentCarbonNumber,
};

pub(in crate::app) const MC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Mass,
};
pub(in crate::app) const PMC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Mass,
};
pub(in crate::app) const SMC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Mass,
};

pub(in crate::app) const SC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Species,
};
pub(in crate::app) const PSC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Species,
};
pub(in crate::app) const SSC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Species,
};

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

/// Composition settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
    pub(in crate::app) empty: bool,

    pub(in crate::app) adduct: OrderedFloat<f64>,
    pub(in crate::app) method: Method,
    pub(in crate::app) window: bool,

    pub(in crate::app) tree: Tree,
    pub(in crate::app) sort: Sort,
    pub(in crate::app) order: Order,

    pub(in crate::app) discrimination: Discrimination,
    pub(in crate::app) filter: Filter,
}

impl Settings {
    pub(in crate::app) fn compositions(&self) -> Vec<Composition> {
        self.tree.branches.iter().copied().collect()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resizable: false,

            percent: true,
            precision: 1,
            empty: false,

            adduct: OrderedFloat(0.0),
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

/// Tree
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(in crate::app) struct Tree {
    pub(in crate::app) branches: IndexSet<Composition>,
    pub(in crate::app) leafs: Composition,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            branches: IndexSet::new(),
            leafs: PSC,
        }
    }
}

impl Hash for Tree {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.branches.as_slice().hash(state);
        self.leafs.hash(state);
    }
}

/// Composition
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Composition {
    pub(in crate::app) scope: Scope,
    pub(in crate::app) stereospecificity: Option<Stereospecificity>,
}

impl Composition {
    pub(in crate::app) fn text(&self) -> &'static str {
        match *self {
            NC => "NC",
            PNC => "PNC",
            SNC => "SNC",

            MC => "MC",
            PMC => "PMC",
            SMC => "SMC",

            TC => "TC",
            PTC => "PTC",
            STC => "STC",

            SC => "SC",
            PSC => "PSC",
            SSC => "SSC",
        }
    }

    pub(in crate::app) fn hover_text(&self) -> &'static str {
        match *self {
            NC => "Equivalent carbon number composition",
            PNC => "Positional equivalent carbon number composition",
            SNC => "Stereo equivalent carbon number composition",

            MC => "Mass composition",
            PMC => "Positional mass composition",
            SMC => "Stereo mass composition",

            TC => "Type composition",
            PTC => "Positional type composition",
            STC => "Stereo type composition",

            SC => "Species composition",
            PSC => "Positional species composition",
            SSC => "Stereo species composition",
        }
    }
}

impl Default for Composition {
    fn default() -> Self {
        Self {
            stereospecificity: Some(Stereospecificity::Positional),
            scope: Scope::Species,
        }
    }
}

/// Scope
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Scope {
    EquivalentCarbonNumber,
    Mass,
    Type,
    Species,
}

impl Scope {
    pub(in crate::app) fn text(&self) -> &'static str {
        match self {
            Self::EquivalentCarbonNumber => "Equivalent carbon number",
            Self::Mass => "Mass",
            Self::Species => "Species",
            Self::Type => "Type",
        }
    }

    pub(in crate::app) fn hover_text(&self) -> &'static str {
        match self {
            Self::EquivalentCarbonNumber => "ECN",
            Self::Mass => "M",
            Self::Species => "S",
            Self::Type => "T",
        }
    }
}

// /// Operation
// #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
// pub(in crate::app) enum Operation {
//     Concatenation,
//     Sum,
// }

// Desecrimination
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Discrimination {
    pub(in crate::app) sn1: BTreeMap<usize, f64>,
    pub(in crate::app) sn2: BTreeMap<usize, f64>,
    pub(in crate::app) sn3: BTreeMap<usize, f64>,
}

impl Hash for Discrimination {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in &self.sn1 {
            key.hash(state);
            value.ord().hash(state);
        }
        for (key, value) in &self.sn2 {
            key.hash(state);
            value.ord().hash(state);
        }
        for (key, value) in &self.sn3 {
            key.hash(state);
            value.ord().hash(state);
        }
    }
}

/// Filter
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Filter {
    pub(in crate::app) sn1: BTreeSet<usize>,
    pub(in crate::app) sn2: BTreeSet<usize>,
    pub(in crate::app) sn3: BTreeSet<usize>,
    pub(in crate::app) value: f64,
    pub(in crate::app) symmetrical: bool,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            sn1: Default::default(),
            sn2: Default::default(),
            sn3: Default::default(),
            value: Default::default(),
            symmetrical: false,
        }
    }
}

impl Hash for Filter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sn1.hash(state);
        self.sn2.hash(state);
        self.sn3.hash(state);
        self.value.ord().hash(state);
        self.symmetrical.hash(state);
    }
}

/// Method
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Method {
    Gunstone,
    VanderWal,
}

impl Method {
    pub(in crate::app) fn text(&self) -> &'static str {
        match self {
            Self::Gunstone => "Gunstone",
            Self::VanderWal => "Vander Wal",
        }
    }

    pub(in crate::app) fn hover_text(&self) -> &'static str {
        match self {
            Self::Gunstone => "Calculate by Gunstone's theory",
            Self::VanderWal => "Calculate by Vander Wal's theory",
        }
    }
}
