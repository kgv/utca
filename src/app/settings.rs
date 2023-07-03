use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Chart
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(super) enum Chart {
    #[default]
    Bar,
    Pie,
}

/// From
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(super) enum From {
    Dag1223,
    Mag2,
}

/// Normalization
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(super) enum Normalization {
    #[default]
    Mass,
    Molar,
    Pchelkin,
}

impl Display for Normalization {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Mass => f.write_str("Mass"),
            Self::Molar => f.write_str("Molar"),
            Self::Pchelkin => f.write_str("Pchelkin?"),
        }
    }
}

/// Positional сomposition
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(super) enum Positional {
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

/// Representation
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(super) enum Representation {
    Unnormalized,
    #[default]
    Normalized,
}

/// Signedness
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(super) enum Signedness {
    Signed,
    #[default]
    Unsigned,
}

impl Display for Signedness {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Signed => f.write_str("Signed"),
            Self::Unsigned => f.write_str("Unsigned"),
        }
    }
}

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(super) enum Sort {
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

/// Sources
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(super) struct Sources {
    pub(super) dag1223: Source,
    pub(super) mags2: Source,
    pub(super) dag13: From,
}

impl Default for Sources {
    fn default() -> Self {
        Self {
            dag1223: Source::Experiment,
            mags2: Source::Experiment,
            dag13: From::Dag1223,
        }
    }
}

/// Source
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(super) enum Source {
    Experiment,
    Calculation,
}

/// Order
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(super) enum Order {
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

mod trash {
    use super::*;

    /// Sum
    #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub(super) enum Sum {
        #[default]
        Percent,
        Count,
    }
}
