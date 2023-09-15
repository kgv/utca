use serde::{Deserialize, Serialize};

/// Settings
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) configuration: configuration::Settings,
    pub(in crate::app) calculation: calculation::Settings,
    pub(in crate::app) composition: composition::Settings,
    pub(in crate::app) visualization: visualization::Settings,
}

pub(in crate::app) mod calculation;
pub(in crate::app) mod composition;
pub(in crate::app) mod configuration;
pub(in crate::app) mod visualization;
