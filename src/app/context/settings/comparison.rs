use super::{Group, Order, Sort};
use serde::{Deserialize, Serialize};

/// Composition settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,

    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) group: Option<Group>,
    pub(in crate::app) sort: Sort,
    pub(in crate::app) mode: Mode,
    pub(in crate::app) order: Order,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resizable: false,
            percent: true,
            precision: 1,
            group: None,
            sort: Sort::Key,
            mode: Mode::MinMax,
            order: Order::Descending,
        }
    }
}

/// Mode
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Mode {
    MinMax,
    Sum,
}

impl Mode {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::MinMax => "Min/Max",
            Self::Sum => "Sum",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::MinMax => "Max or min value depending on order",
            Self::Sum => "Sum of values",
        }
    }
}
