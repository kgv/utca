use self::{settings::Settings, state::State};
use serde::{Deserialize, Serialize};

/// Context
#[derive(Debug, Default, Deserialize, Serialize)]
pub(super) struct Context {
    pub(super) settings: Settings,
    pub(super) state: State,
}

pub(super) mod settings;
pub(super) mod state;
