use crate::app::{context::Context, view::View};
use egui::{DroppedFile, Id, Label, Sense, TextStyle, Ui};
use egui_dnd::dnd;
use egui_extras::{Size, StripBuilder};

/// Files
#[derive(Debug)]
pub(super) struct Files<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Files<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Files<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        // let height = ui.spacing().interact_size.y;
        // StripBuilder::new(ui)
        //     .sizes(Size::exact(height), count)
        //     .vertical(|mut strip| {
        //         for index in 0..count {
        //             let mut keep = true;
        //             strip.strip(|builder| {
        //                 let text = *item;
        //                 dnd(ui, "dnd").show_custom_vec(&mut items, |ui, item, handle, state| {
        //                     builder
        //                         .size(Size::exact(height))
        //                         .size(Size::remainder())
        //                         .size(Size::exact(height))
        //                         .horizontal(|mut strip| {
        //                             strip.cell(|ui| {
        //                                 let mut i = 3;
        //                                 ui.radio_value(&mut i, index, "");
        //                             });
        //                             strip.cell(|ui| {
        //                                 ui.add(Label::new(text).truncate(true));
        //                             });
        //                             strip.cell(|ui| {
        //                                 keep = !ui.button("🗑").clicked();
        //                             });
        //                         });
        //                 });
        //             });
        //         }
        //     });

        // let mut remove = None;
        // let response =
        //     dnd(ui, "dnd").show(context.state.entries.iter(), |ui, item, handle, state| {
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
        //     });
        // if let Some(index) = remove {
        //     context.state.entries.remove(index);
        //     if index <= context.state.index {
        //         context.state.index = context.state.index.saturating_sub(1);
        //     }
        // }
        // response.update_vec(&mut context.state.entries);

        let mut remove = None;
        dnd(ui, "dnd").show_vec(&mut context.state.entries, |ui, item, handle, state| {
            ui.horizontal(|ui| {
                handle.ui(ui, |ui| {
                    let _ = ui.button(if state.dragged { "👊" } else { "✋" });
                });
                ui.radio_value(&mut context.state.index, state.index, "");
                ui.add(Label::new(&item.meta.name).truncate(true));
                if ui.button("🗑").clicked() {
                    remove = Some(state.index);
                }
            });
        });
        if let Some(index) = remove {
            context.state.entries.remove(index);
            if index <= context.state.index {
                context.state.index = context.state.index.saturating_sub(1);
            }
            if context.state.entries.is_empty() {
                context.state.entries.push(Default::default());
            }
        }
    }
}
