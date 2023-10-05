use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
/// Configuration settings
pub(in crate::app) struct Settings {
    pub(in crate::app) precision: usize,
    pub(in crate::app) resizable: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            precision: Default::default(),
            resizable: Default::default(),
        }
    }
}
