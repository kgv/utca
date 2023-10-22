use egui::epaint::util::FloatOrd;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Visualization settings
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) legend: bool,
    pub(in crate::app) multiple: bool,
    pub(in crate::app) width: f64,
}

impl Hash for Settings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.percent.hash(state);
        self.precision.hash(state);
        self.legend.hash(state);
        self.multiple.hash(state);
        self.width.ord().hash(state);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            percent: true,
            precision: 1,
            legend: true,
            multiple: false,
            width: 0.65,
        }
    }
}
