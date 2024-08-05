use egui::{Ui, WidgetText};
use egui_tiles::{TileId, UiResponse};
use serde::{Deserialize, Serialize};

/// Central pane
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum Pane {
    Configuration(configuration::Pane),
}

impl Pane {
    pub(crate) const fn name(&self) -> &str {
        match self {
            Self::Configuration(pane) => pane.name(),
        }
    }

    fn ui(&mut self, ui: &mut Ui) -> UiResponse {
        match self {
            Self::Configuration(pane) => pane.ui(ui),
        }
    }

    fn settings_ui(&mut self, ui: &mut Ui) -> UiResponse {
        match self {
            Self::Configuration(pane) => pane.settings_ui(ui),
        }
    }
}

impl PartialEq for Pane {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Configuration(_), Self::Configuration(_)) => true,
        }
    }
}

/// Behavior
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Behavior {
    pub close: Option<TileId>,
}

impl egui_tiles::Behavior<Pane> for Behavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        pane.name().into()
    }

    fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        pane.ui(ui)
    }
}

/// Settings behavior
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct SettingsBehavior;

impl egui_tiles::Behavior<Pane> for SettingsBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        pane.name().into()
    }

    fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        pane.settings_ui(ui)
    }
}

pub(crate) mod configuration;
