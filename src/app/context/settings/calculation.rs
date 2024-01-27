use serde::{Deserialize, Serialize};

/// Calculation settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) fraction: Fraction,
    pub(in crate::app) selectivity: bool,
    pub(in crate::app) theoretical: bool,
    pub(in crate::app) unnormalized: bool,
    pub(in crate::app) pchelkin: bool,
    pub(in crate::app) signedness: Signedness,
    pub(in crate::app) from: From,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resizable: false,
            percent: true,
            precision: 1,
            fraction: Fraction::Molar { mixture: true },
            selectivity: true,
            theoretical: false,
            unnormalized: false,
            pchelkin: false,
            signedness: Default::default(),
            from: From::Mag2,
        }
    }
}

/// Fraction
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Fraction {
    /// [wikipedia.org](https://en.wikipedia.org/wiki/Mole_fraction#Mass_fraction)
    Mass,
    /// [wikipedia.org](https://en.wikipedia.org/wiki/Mole_fraction)
    Molar { mixture: bool },
}

impl Fraction {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Mass => "Mass",
            Self::Molar { mixture: false } => "Molar mass",
            Self::Molar { mixture: true } => "Mixture molar mass",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Mass => "S / ∑ S",
            Self::Molar { mixture: false } => "S * M / ∑(S * M)",
            Self::Molar { mixture: true } => "S / ∑(S * M)",
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

impl Signedness {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Signed => "Signed",
            Self::Unsigned => "Unsigned",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Signed => "Theoretically calculated negative values are as is",
            Self::Unsigned => "Theoretically calculated negative values are replaced with zeros",
        }
    }
}

/// From
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum From {
    Dag1223,
    Mag2,
}

impl From {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Dag1223 => "1,2/2,3-DAGs",
            Self::Mag2 => "2-MAGs",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Dag1223 => "Calculate 1,3-DAGs from 1,2/2,3-DAGs",
            Self::Mag2 => "Calculate 1,3-DAGs from 2-MAGs",
        }
    }
}
