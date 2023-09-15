use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
/// Configuration settings
pub(in crate::app) struct Settings {
    pub(in crate::app) precision: usize,
    pub(in crate::app) resizable: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            precision: 3,
            resizable: Default::default(),
        }
    }
}
