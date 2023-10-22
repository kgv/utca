use serde::{Deserialize, Serialize};

/// Calculation settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) normalization: Normalization,
    pub(in crate::app) signedness: Signedness,
    pub(in crate::app) sources: Sources,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resizable: false,
            percent: true,
            precision: 1,
            normalization: Default::default(),
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

impl Normalization {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Mass => "Mass",
            Self::Molar => "Molar",
            Self::Pchelkin => "Pchelkin???",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Mass => "s / ∑(s)",
            Self::Molar => "(s * m) / ∑(s * m)",
            Self::Pchelkin => "s / ∑(s * m / 10.0)",
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
            Self::Signed => "Calculated negative values are as is",
            Self::Unsigned => "Calculated negative values are replaced with zeros",
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
            dag1223: Source::Experimental,
            mags2: Source::Experimental,
            dag13: From::Dag1223,
        }
    }
}

/// Source
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Source {
    Experimental,
    Calculated,
}

impl Source {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Experimental => "Experimental",
            Self::Calculated => "Calculated",
        }
    }

    pub(in crate::app) fn hover_text(self, from: From) -> String {
        match self {
            Self::Experimental => format!("{} {}", self.text(), from.text()),
            Self::Calculated => format!("{} {}", self.text(), from.text()),
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

    pub(in crate::app) fn hover_text(self, source: Source) -> String {
        match self {
            Self::Dag1223 => format!("{} {}", source.text(), self.text()),
            Self::Mag2 => format!("{} {}", source.text(), self.text()),
        }
    }
}
