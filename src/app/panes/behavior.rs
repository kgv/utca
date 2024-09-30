use super::{Pane, Settings};
use crate::app::data::{Data, FattyAcids};
use egui::{vec2, Button, CursorIcon, Frame, Margin, RichText, Sides, Ui, Vec2, WidgetText};
use egui_phosphor::regular::X;
use egui_tiles::{TileId, Tiles, Tree, UiResponse};

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
