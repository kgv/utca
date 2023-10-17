use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
/// Configuration settings
pub(in crate::app) struct Settings {
    pub(in crate::app) precision: usize,
    pub(in crate::app) resizable: bool,
}
