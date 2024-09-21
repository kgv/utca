use super::{Pane, Settings};
use crate::app::data::{Data, FattyAcids};
use egui::{Button, CursorIcon, RichText, Ui, WidgetText};
use egui_phosphor::regular::X;
use egui_tiles::{TileId, UiResponse};

/// Behavior
#[derive(Debug)]
pub(in crate::app) struct Behavior<'a> {
    pub(in crate::app) fatty_acids: &'a mut FattyAcids,
    pub(in crate::app) settings: &'a Settings,
    pub(in crate::app) close: Option<TileId>,
}

impl egui_tiles::Behavior<Pane> for Behavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        pane.title().into()
    }

    fn pane_ui(&mut self, ui: &mut Ui, tile_id: TileId, pane: &mut Pane) -> UiResponse {
        let response = ui
            .horizontal(|ui| {
                let response = ui.heading(pane.title()).on_hover_cursor(CursorIcon::Grab);
                ui.add_space(
                    ui.available_width()
                        - 2.0 * ui.spacing().button_padding.x
                        - ui.spacing().interact_size.y,
                );
                ui.visuals_mut().button_frame = false;
                if ui.button(RichText::new(X)).clicked() {
                    self.close = Some(tile_id);
                }
                response
            })
            .inner;
        pane.ui(ui, self);
        if response.dragged() {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }
}
