use crate::app::{
    context::{
        settings::calculation::{Normalization, Signedness},
        Context,
    },
    MAX_PRECISION,
};
use egui::{ComboBox, RichText, Slider, Ui};

/// Left calculation tab
pub(super) struct Calculation<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Calculation<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl Calculation<'_> {
    pub(super) fn view(self, ui: &mut Ui) {
        ui.collapsing(RichText::new("🖩 Calculation").heading(), |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(
                    &mut self.context.settings.calculation.resizable,
                    "↔ Resizable",
                )
                .on_hover_text("Resize table columns")
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Precision:");
                ui.add(Slider::new(
                    &mut self.context.settings.calculation.precision,
                    0..=MAX_PRECISION,
                ));
            });
            ui.horizontal(|ui| {
                ui.label("Percent:");
                ui.checkbox(&mut self.context.settings.calculation.percent, "");
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Normalization:");
                ComboBox::from_id_source("normalization")
                    .selected_text(self.context.settings.calculation.normalization.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.context.settings.calculation.normalization,
                            Normalization::Mass,
                            "Mass",
                        )
                        .on_hover_text("Mass parts");
                        ui.selectable_value(
                            &mut self.context.settings.calculation.normalization,
                            Normalization::Molar,
                            "Molar",
                        )
                        .on_hover_text("Molar parts");
                        ui.selectable_value(
                            &mut self.context.settings.calculation.normalization,
                            Normalization::Pchelkin,
                            "Pchelkin",
                        )
                        .on_hover_text("Molar parts (Pchelkin)");
                    })
                    .response
                    .on_hover_text(format!(
                        "{:#}",
                        self.context.settings.calculation.normalization
                    ));
            });
            ui.horizontal(|ui| {
                ui.label("Signedness:");
                ComboBox::from_id_source("signedness")
                    .selected_text(self.context.settings.calculation.signedness.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.context.settings.calculation.signedness,
                            Signedness::Signed,
                            "Signed",
                        );
                        ui.selectable_value(
                            &mut self.context.settings.calculation.signedness,
                            Signedness::Unsigned,
                            "Unsigned",
                        );
                    })
                    .response
                    .on_hover_text(format!(
                        "{:#}",
                        self.context.settings.calculation.signedness
                    ));
            });
        });
    }
}
