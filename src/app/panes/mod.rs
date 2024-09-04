use crate::localization::{CALCULATION, COMPOSITION, CONFIGURATION, EDIT, RESIZE};
use egui::{menu::bar, RichText, Ui, WidgetText};
use egui_phosphor::regular::{ARROWS_HORIZONTAL, CALCULATOR, INTERSECT_THREE, NOTE_PENCIL, PENCIL};
use egui_tiles::{Tile, TileId, Tree, UiResponse};
use polars::prelude::DataFrame;
use serde::{Deserialize, Serialize};

use super::data::Data;

const SIZE: f32 = 16.0;

/// Central pane
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum Pane {
    Configuration(configuration::Pane),
    Calculation(calculation::Pane),
    Composition(composition::Pane),
}

impl Pane {
    pub(crate) const fn icon(&self) -> &str {
        match self {
            Self::Configuration(_) => NOTE_PENCIL,
            Self::Calculation(_) => CALCULATOR,
            Self::Composition(_) => INTERSECT_THREE,
        }
    }

    pub(crate) fn title(&self) -> &str {
        match self {
            Self::Configuration(_) => &CONFIGURATION,
            Self::Calculation(_) => &CALCULATION,
            Self::Composition(_) => &COMPOSITION,
        }
    }

    fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) -> UiResponse {
        match self {
            Self::Configuration(pane) => pane.ui(ui, behavior),
            Self::Calculation(pane) => pane.ui(ui, behavior),
            Self::Composition(pane) => pane.ui(ui, behavior),
        }
    }

    fn settings(&mut self, ui: &mut Ui) -> UiResponse {
        match self {
            Self::Configuration(pane) => pane.settings.ui(ui),
            Self::Calculation(pane) => pane.settings.ui(ui),
            Self::Composition(pane) => pane.settings.ui(ui),
        }
    }
}

impl From<&Pane> for Kind {
    fn from(value: &Pane) -> Self {
        match value {
            Pane::Configuration(_) => Self::Configuration,
            Pane::Calculation(_) => Self::Calculation,
            Pane::Composition(_) => Self::Composition,
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
pub(crate) enum Kind {
    Configuration,
    Calculation,
    Composition,
}

/// Behavior
#[derive(Debug)]
pub(crate) struct Behavior<'a> {
    pub(crate) data: &'a mut Data,
    pub(crate) settings: &'a Settings,
    pub(crate) close: Option<TileId>,
}

impl egui_tiles::Behavior<Pane> for Behavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        pane.title().into()
    }

    fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        pane.ui(ui, self)
    }
}

/// Settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) resizable: bool,
    pub(crate) editable: bool,
}

impl Settings {
    pub(crate) fn ui(&mut self, ui: &mut Ui, tree: &mut Tree<Pane>) {
        bar(ui, |ui| {
            ui.toggle_value(
                &mut self.resizable,
                RichText::new(ARROWS_HORIZONTAL).size(SIZE),
            )
            .on_hover_text(&RESIZE);
            ui.toggle_value(&mut self.editable, RichText::new(PENCIL).size(SIZE))
                .on_hover_text(&EDIT);
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
        Self {
            resizable: false,
            editable: false,
        }
    }
}

/// [`Tree`] extension methods
pub(crate) trait TreeExt {
    fn insert_pane(&mut self, pane: Pane);
}

impl TreeExt for Tree<Pane> {
    fn insert_pane(&mut self, pane: Pane) {
        let child = self.tiles.insert_pane(pane);
        if let Some(root) = self.root {
            if let Some(tile) = self.tiles.get_mut(root) {
                if let Tile::Container(container) = tile {
                    container.add_child(child);
                } else {
                    self.root = Some(self.tiles.insert_vertical_tile(vec![root, child]));
                }
            }
        } else {
            self.root = Some(child)
        }
    }
}

pub(crate) mod calculation;
pub(crate) mod composition;
pub(crate) mod configuration;
