// use crate::localization::{CALCULATION, COMPOSITION, CONFIGURATION, EDIT, RESIZE};
use self::behavior::Behavior;
use crate::localization::localize;
use egui::{menu::bar, RichText, Ui, WidgetText};
use egui_phosphor::regular::{
    ARROWS_HORIZONTAL, CALCULATOR, INTERSECT_THREE, NOTE_PENCIL, PENCIL, TABLE,
};
use egui_tiles::{Tile, TileId, Tree, UiResponse};
use serde::{Deserialize, Serialize};

const SIZE: f32 = 16.0;

/// Central pane
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(in crate::app) enum Pane {
    Configuration(configuration::Pane),
    Calculation(calculation::Pane),
    Composition(composition::Pane),
    Comparison(comparison::Pane),
}

impl Pane {
    pub(in crate::app) const fn icon(&self) -> &str {
        match self {
            Self::Configuration(_) => NOTE_PENCIL,
            Self::Calculation(_) => CALCULATOR,
            Self::Composition(_) => INTERSECT_THREE,
            Self::Comparison(_) => TABLE,
        }
    }

    pub(in crate::app) fn title(&self) -> String {
        match self {
            Self::Configuration(_) => localize!("configuration"),
            Self::Calculation(_) => localize!("calculation"),
            Self::Composition(_) => localize!("composition"),
            Self::Comparison(_) => localize!("comparison"),
        }
    }

    fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) {
        match self {
            Self::Configuration(pane) => pane.ui(ui, behavior),
            Self::Calculation(pane) => pane.ui(ui, behavior),
            Self::Composition(pane) => pane.ui(ui, behavior),
            Self::Comparison(pane) => pane.ui(ui, behavior),
        }
    }

    fn settings(&mut self, ui: &mut Ui) -> UiResponse {
        match self {
            Self::Configuration(pane) => pane.settings.ui(ui),
            Self::Calculation(pane) => pane.settings.ui(ui),
            Self::Composition(pane) => pane.settings.ui(ui),
            Self::Comparison(pane) => pane.settings.ui(ui),
        }
    }
}

impl From<&Pane> for Kind {
    fn from(value: &Pane) -> Self {
        match value {
            Pane::Configuration(_) => Self::Configuration,
            Pane::Calculation(_) => Self::Calculation,
            Pane::Composition(_) => Self::Composition,
            Pane::Comparison(_) => Self::Comparison,
        }
    }
}

impl PartialEq for Pane {
    fn eq(&self, other: &Self) -> bool {
        Kind::from(self) == Kind::from(other)
    }
}

/// Central pane kind
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::app) enum Kind {
    Configuration,
    Calculation,
    Composition,
    Comparison,
}

/// Settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) resizable: bool,
    pub(in crate::app) editable: bool,
}

impl Settings {
    pub(in crate::app) const fn new() -> Self {
        Self {
            resizable: false,
            editable: false,
        }
    }
}

impl Settings {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui, tree: &mut Tree<Pane>) {
        bar(ui, |ui| {
            ui.toggle_value(
                &mut self.resizable,
                RichText::new(ARROWS_HORIZONTAL).size(SIZE),
            )
            .on_hover_text(localize!("resize"));
            ui.toggle_value(&mut self.editable, RichText::new(PENCIL).size(SIZE))
                .on_hover_text(localize!("edit"));
        });
        ui.separator();
        for tile_id in tree.active_tiles() {
            if let Some(tile) = tree.tiles.get_mut(tile_id) {
                if let Tile::Pane(pane) = tile {
                    let _ = pane.settings(ui);
                }
            }
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

pub(in crate::app) mod behavior;
pub(in crate::app) mod calculation;
pub(in crate::app) mod comparison;
pub(in crate::app) mod composition;
pub(in crate::app) mod configuration;
