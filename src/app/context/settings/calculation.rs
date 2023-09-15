use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Calculation settings
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) normalization: Normalization,
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
    pub(in crate::app) resizable: bool,
    pub(in crate::app) signedness: Signedness,
    pub(in crate::app) sources: Sources,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            normalization: Default::default(),
            percent: true,
            precision: 6,
            resizable: Default::default(),
            signedness: Default::default(),
            sources: Default::default(),
        }
    }
}

/// Normalization
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Normalization {
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
            Self::Pchelkin => f.write_str("Pchelkin???"),
        }
    }
}

/// Signedness
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Signedness {
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

/// Sources
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Sources {
    pub(in crate::app) dag1223: Source,
    pub(in crate::app) mags2: Source,
    pub(in crate::app) dag13: From,
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
pub(in crate::app) enum Source {
    Experiment,
    Calculation,
}

/// From
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum From {
    Dag1223,
    Mag2,
}
