use crate::app::{
    context::{
        settings::calculation::{Normalization, Signedness},
        Context,
    },
    tabs::CentralTab,
    view::View,
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

impl View for Calculation<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        ui.collapsing(
            RichText::new(CentralTab::Calculation.to_string()).heading(),
            |ui| {
                ui.horizontal(|ui| {
                    ui.toggle_value(&mut context.settings.calculation.resizable, "↔ Resizable")
                        .on_hover_text("Resize table columns")
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    let precision = &mut context.settings.calculation.precision;
                    if ui.add(Slider::new(precision, 0..=MAX_PRECISION)).changed()
                        && context.settings.link
                    {
                        context.settings.configuration.precision = *precision;
                        context.settings.composition.precision = *precision;
                        context.settings.visualization.precision = *precision;
                        context.settings.comparison.precision = *precision;
                    }
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.horizontal(|ui| {
                    ui.label("Percent:");
                    ui.checkbox(&mut context.settings.calculation.percent, "");
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Normalization:");
                    ComboBox::from_id_source("normalization")
                        .selected_text(context.settings.calculation.normalization.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.calculation.normalization,
                                Normalization::Mass,
                                Normalization::Mass.text(),
                            )
                            .on_hover_text(Normalization::Mass.hover_text());
                            ui.selectable_value(
                                &mut context.settings.calculation.normalization,
                                Normalization::Molar,
                                Normalization::Molar.text(),
                            )
                            .on_hover_text(Normalization::Molar.hover_text());
                            ui.selectable_value(
                                &mut context.settings.calculation.normalization,
                                Normalization::Pchelkin,
                                Normalization::Pchelkin.text(),
                            )
                            .on_hover_text(Normalization::Pchelkin.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.calculation.normalization.hover_text());
                });
                ui.horizontal(|ui| {
                    ui.label("Signedness:");
                    ComboBox::from_id_source("signedness")
                        .selected_text(context.settings.calculation.signedness.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.calculation.signedness,
                                Signedness::Signed,
                                Signedness::Signed.text(),
                            )
                            .on_hover_text(Signedness::Signed.hover_text());
                            ui.selectable_value(
                                &mut context.settings.calculation.signedness,
                                Signedness::Unsigned,
                                Signedness::Unsigned.text(),
                            )
                            .on_hover_text(Signedness::Unsigned.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.calculation.signedness.hover_text());
                });
            },
        );
    }
}
