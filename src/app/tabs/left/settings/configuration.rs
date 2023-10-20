use crate::app::{context::Context, MAX_PRECISION};
use egui::{DragValue, RichText, Slider, Ui};

const MAX_C: usize = 99;

/// Left configuration tab
pub(super) struct Configuration<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Configuration<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl Configuration<'_> {
    pub(super) fn view(self, ui: &mut Ui) {
        ui.collapsing(RichText::new("📝 Configuration").heading(), |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(
                    &mut self.context.settings.configuration.resizable,
                    "↔ Resizable",
                )
                .on_hover_text("Resize table columns");
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Precision:");
                ui.add(Slider::new(
                    &mut self.context.settings.configuration.precision,
                    0..=MAX_PRECISION,
                ));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("C:");
                ui.add(
                    DragValue::new(&mut self.context.settings.configuration.c.start)
                        .clamp_range(0..=self.context.settings.configuration.c.end),
                )
                .on_hover_text("Start");
                ui.add(
                    DragValue::new(&mut self.context.settings.configuration.c.end)
                        .clamp_range(self.context.settings.configuration.c.start..=MAX_C),
                )
                .on_hover_text("End");
                ui.label("U:");
                ui.add(
                    DragValue::new(&mut self.context.settings.configuration.u)
                        .clamp_range(0..=self.context.settings.configuration.c.end - 2),
                )
                .on_hover_text("End");
            });
        });
    }
}
