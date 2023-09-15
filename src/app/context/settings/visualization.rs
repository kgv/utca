use serde::{Deserialize, Serialize};

/// Visualization settings
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) chart: Chart,
    pub(in crate::app) legend: bool,
    pub(in crate::app) normalized: bool,
    pub(in crate::app) width: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            chart: Default::default(),
            legend: true,
            normalized: false,
            width: 0.65,
        }
    }
}

/// Chart
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) enum Chart {
    #[default]
    Bar,
    Pie,
}
