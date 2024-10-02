use super::{Pane, Settings};
use crate::app::data::Data;
use egui::{CursorIcon, RichText, Sides, Ui, WidgetText};
use egui_phosphor::regular::X;
use egui_tiles::{TileId, UiResponse};

/// Behavior
#[derive(Debug)]
pub(in crate::app) struct Behavior<'a> {
    pub(in crate::app) data: &'a mut Data,
    pub(in crate::app) settings: &'a Settings,
    pub(in crate::app) close: Option<TileId>,
}

impl egui_tiles::Behavior<Pane> for Behavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        pane.title().into()
    }

    fn pane_ui(&mut self, ui: &mut Ui, tile_id: TileId, pane: &mut Pane) -> UiResponse {
        let response = Sides::new()
            .show(
                ui,
                |ui| ui.heading(pane.title()).on_hover_cursor(CursorIcon::Grab),
                |ui| {
                    ui.visuals_mut().button_frame = false;
                    if ui.button(RichText::new(X)).clicked() {
                        self.close = Some(tile_id);
                    }
                },
            )
            .0;
        pane.ui(ui, self);
        if response.dragged() {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }
}
