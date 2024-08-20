use egui::{menu::bar, RichText, Ui, WidgetText};
use egui_phosphor::regular::{ARROWS_HORIZONTAL, CALCULATOR, NOTE_PENCIL, PENCIL};
use egui_tiles::{Tile, TileId, Tree, UiResponse};
use serde::{Deserialize, Serialize};

const SIZE: f32 = 16.0;

/// Central pane
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum Pane {
    Configuration(configuration::Pane),
    Calculation(calculation::Pane),
}

impl Pane {
    pub(crate) const fn icon(&self) -> &str {
        match self {
            Self::Configuration(_) => NOTE_PENCIL,
            Self::Calculation(_) => CALCULATOR,
        }
    }

    pub(crate) const fn title(&self) -> &str {
        match self {
            Self::Configuration(_) => configuration::TITLE,
            Self::Calculation(_) => calculation::TITLE,
        }
    }

    fn ui(&mut self, ui: &mut Ui, settings: &Settings) -> UiResponse {
        match self {
            Self::Configuration(pane) => pane.ui(ui, settings),
            Self::Calculation(pane) => pane.ui(ui, settings),
        }
    }

    fn settings(&mut self, ui: &mut Ui) -> UiResponse {
        match self {
            Self::Configuration(pane) => pane.settings.ui(ui),
            Self::Calculation(pane) => pane.settings.ui(ui),
        }
    }
}

impl From<&Pane> for Kind {
    fn from(value: &Pane) -> Self {
        match value {
            Pane::Configuration(_) => Self::Configuration,
            Pane::Calculation(_) => Self::Calculation,
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
}

/// Behavior
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Behavior {
    pub(crate) settings: Settings,
    pub(crate) close: Option<TileId>,
}

impl Behavior {
    pub(crate) fn settings(&mut self, ui: &mut Ui, tree: &mut Tree<Pane>) {
        bar(ui, |ui| {
            ui.toggle_value(
                &mut self.settings.resizable,
                RichText::new(ARROWS_HORIZONTAL).size(SIZE),
            )
            .on_hover_text("Resize table columns");
            ui.toggle_value(
                &mut self.settings.editable,
                RichText::new(PENCIL).size(SIZE),
            )
            .on_hover_text("Edit table");
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

impl egui_tiles::Behavior<Pane> for Behavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        pane.title().into()
    }

    fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        pane.ui(ui, &self.settings)
    }
}

/// Settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) resizable: bool,
    pub(crate) editable: bool,
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

// /// Settings behavior
// #[derive(Clone, Copy, Debug, Default)]
// pub(crate) struct SettingsBehavior;

// impl egui_tiles::Behavior<Pane> for SettingsBehavior {
//     fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
//         pane.name().into()
//     }

//     fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
//         pane.settings_ui(ui)
//     }
// }

pub(crate) mod calculation;
pub(crate) mod configuration;
