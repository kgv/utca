use egui::epaint::util::FloatOrd;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Visualization settings
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) legend: bool,
    pub(in crate::app) stacked: bool,
    pub(in crate::app) width: f64,
}

impl Hash for Settings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.legend.hash(state);
        self.stacked.hash(state);
        self.width.ord().hash(state);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            legend: true,
            stacked: false,
            width: 0.65,
        }
    }
}

// /// Chart
// #[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
// pub(in crate::app) enum Chart {
//     #[default]
//     Bar,
//     Pie,
// }
