use egui::{CollapsingHeader, Id, Label, Response, RichText, Sense, Ui, Widget};
use egui_dnd::dnd;
use egui_phosphor::regular::{
    ARROWS_OUT, ARROWS_OUT_CARDINAL, CARET_UP_DOWN, HAND, HAND_GRABBING, TRASH,
};

use crate::{app::data::Data, localization::titlecase};

/// Files
#[derive(Debug)]
pub(crate) struct Files<'a> {
    pub(crate) data: &'a mut Vec<Data>,
    pub(crate) index: &'a mut usize,
}

impl<'a> Files<'a> {
    pub(crate) fn new(data: &'a mut Vec<Data>, index: &'a mut usize) -> Self {
        Self { data, index }
    }
}

impl Widget for Files<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.collapsing(RichText::new(titlecase!("files")).heading(), |ui| {
            let mut remove = None;
            // dnd(ui, Id::new("files")).show_vec(self.data, |ui, item, handle, state| {
            //     ui.horizontal(|ui| {
            //         handle.ui(ui, |ui| {
            //             let _ = ui.label(ARROWS_OUT_CARDINAL);
            //         });
            //         ui.radio_value(self.index, state.index, "");
            //         ui.add(Label::new(format!("{:?}", item.fatty_acids.shape())).truncate());
            //         let amount = ui.available_width()
            //             - ui.spacing().interact_size.y
            //             - 2.0 * ui.spacing().button_padding.x;
            //         ui.add_space(amount);
            //         if ui.button(TRASH).clicked() {
            //             remove = Some(state.index);
            //         }
            //     });
            // });
            for (index, data) in self.data.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.radio_value(self.index, index, "");
                    ui.add(Label::new(format!("{:?}", data.fatty_acids.shape())).truncate());
                    let amount = ui.available_width()
                        - ui.spacing().interact_size.y
                        - 2.0 * ui.spacing().button_padding.x;
                    ui.add_space(amount);
                    if ui.button(TRASH).clicked() {
                        remove = Some(index);
                    }
                });
            }
            if let Some(index) = remove {
                self.data.remove(index);
                if index <= *self.index {
                    *self.index = self.index.saturating_sub(1);
                }
                ui.ctx().request_repaint();
            }
        });
        ui.allocate_response(Default::default(), Sense::hover())
        // dnd(ui, Id::new("dnd").with("files")).show_vec(
        //     &mut self.data,
        //     |ui, item, handle, state| {
        //         ui.horizontal(|ui| {
        //             handle.ui(ui, |ui| {
        //                 let _ = ui.button(if state.dragged { "👊" } else { "✋" });
        //             });
        //             ui.radio_value(&mut context.state.index, state.index, "");
        //             ui.add(Label::new(&item.meta.name).truncate(true));
        //             if ui.button("🗑").clicked() {
        //                 remove = Some(state.index);
        //             }
        //         });
        //     },
        // );
        // if let Some(index) = remove {
        //     context.state.entries.remove(index);
        //     if index <= context.state.index {
        //         context.state.index = context.state.index.saturating_sub(1);
        //     }
        //     if context.state.entries.is_empty() {
        //         context.state.entries.push(Default::default());
        //     }
        // }
    }
}
