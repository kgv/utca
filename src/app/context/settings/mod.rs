use serde::{Deserialize, Serialize};

/// Settings
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) link: bool,
    pub(in crate::app) configuration: configuration::Settings,
    pub(in crate::app) calculation: calculation::Settings,
    pub(in crate::app) composition: composition::Settings,
    pub(in crate::app) visualization: visualization::Settings,
    pub(in crate::app) comparison: comparison::Settings,
}

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Group {
    Composition(composition::Group),
    Occurrence,
}

impl Group {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Composition(group) => group.text(),
            Self::Occurrence => "Occurrence",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Composition(group) => group.hover_text(),
            Self::Occurrence => "Group by occurrence ()",
        }
    }
}

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Sort {
    Key,
    Value,
}

impl Sort {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Key => "Key",
            Self::Value => "Value",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Key => "Sort by key",
            Self::Value => "Sort by value",
        }
    }
}

/// Order
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Order {
    Ascending,
    Descending,
}

impl Order {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Ascending => "⬈ Ascending",
            Self::Descending => "⬊ Descending",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Ascending => "Direct order (from min to max)",
            Self::Descending => "Reverse order (from max to min)",
        }
    }
}

pub(in crate::app) mod calculation;
pub(in crate::app) mod comparison;
pub(in crate::app) mod composition;
pub(in crate::app) mod configuration;
pub(in crate::app) mod visualization;
