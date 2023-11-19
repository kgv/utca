use crate::app::{context::Context, view::View};
use egui::{Id, Label, Ui};
use egui_dnd::dnd;

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
        let mut remove = None;
        dnd(ui, Id::new("dnd").with("files")).show_vec(
            &mut context.state.entries,
            |ui, item, handle, state| {
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
            },
        );
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
