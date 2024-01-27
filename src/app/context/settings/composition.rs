use super::{Order, Sort};
use egui::epaint::util::FloatOrd;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    hash::{Hash, Hasher},
};

pub(in crate::app) const ECN: Group = Group::Structure(Structure::EquivalentCarbonNumber);
pub(in crate::app) const M: Group = Group::Structure(Structure::Mass);
pub(in crate::app) const STC: Group = Group::Type(Saturation::StereoComposition);
pub(in crate::app) const PTC: Group = Group::Type(Saturation::PositionalComposition);
pub(in crate::app) const TC: Group = Group::Type(Saturation::Composition);

/// Composition settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) adduct: OrderedFloat<f64>,
    pub(in crate::app) r#type: Type,
    pub(in crate::app) symmetrical: bool,

    pub(in crate::app) method: Method,
    pub(in crate::app) temp: bool,

    pub(in crate::app) groups: Vec<Group>,
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

            adduct: OrderedFloat(0.0),
            r#type: Type::Positional,
            symmetrical: false,

            method: Method::VanderWal,
            temp: false,

            groups: Vec::new(),
            sort: Sort::Value,
            order: Order::Descending,

            filter: Default::default(),
        }
    }
}

/// Checkable
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Checkable<T> {
    pub(in crate::app) value: T,
    pub(in crate::app) checked: bool,
}

impl<T> Checkable<T> {
    pub(in crate::app) fn new(value: T) -> Self {
        Self {
            value,
            checked: false,
        }
    }
}

impl<T> From<Checkable<T>> for Option<T> {
    fn from(Checkable { value, checked }: Checkable<T>) -> Self {
        if checked {
            Some(value)
        } else {
            None
        }
    }
}

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Group {
    Structure(Structure),
    Type(Saturation),
}

impl Group {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            ECN => "ECN",
            M => "M",
            PTC => "PTC",
            STC => "STC",
            TC => "TC",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            ECN => "Group by ECN (Equivalent Carbon Number)",
            M => "Group by M (Mass)",
            PTC => "Group by PTC (Positional-Type Composition)",
            STC => "Group by STC (Stereo-Type Composition)",
            TC => "Group by TC (Type Composition)",
        }
    }
}

/// Saturation
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Saturation {
    StereoComposition,
    PositionalComposition,
    Composition,
}

/// Structure
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Structure {
    EquivalentCarbonNumber,
    Mass,
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

/// Method
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Method {
    Gunstone,
    VanderWal,
    KazakovSidorov,
}

impl Method {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Gunstone => "Gunstone",
            Self::KazakovSidorov => "Kazakov-Sidorov",
            Self::VanderWal => "Vander Wal",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Gunstone => "Calculate by Gunstone's theory",
            Self::KazakovSidorov => "Calculate by Kazakov-Sidorov's theory",
            Self::VanderWal => "Calculate by Vander Wal's theory",
        }
    }
}

/// Type
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Type {
    Stereo,
    Positional,
}

impl Type {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Stereo => "Stereo",
            Self::Positional => "Positional",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Stereo => "Stereo composition",
            Self::Positional => "Positional composition",
        }
    }
}
