use crate::app::{context::Context, tabs::CentralTab, view::View, MAX_PRECISION};
use egui::{RichText, Slider, Ui};

/// Left visualization tab
pub(super) struct Visualization<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Visualization<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Visualization<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        ui.collapsing(
            RichText::new(CentralTab::Visualization.to_string()).heading(),
            |ui| {
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    let precision = &mut context.settings.visualization.precision;
                    if ui.add(Slider::new(precision, 0..=MAX_PRECISION)).changed()
                        && context.settings.link
                    {
                        context.settings.configuration.precision = *precision;
                        context.settings.calculation.precision = *precision;
                        context.settings.composition.precision = *precision;
                        context.settings.comparison.precision = *precision;
                    }
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.horizontal(|ui| {
                    ui.label("Percent:");
                    ui.checkbox(&mut context.settings.visualization.percent, "");
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Legend:");
                    ui.checkbox(&mut context.settings.visualization.legend, "");
                });
                ui.horizontal(|ui| {
                    ui.label("One/Many:");
                    ui.checkbox(&mut context.settings.visualization.multiple, "");
                });
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.add(Slider::new(
                        &mut context.settings.visualization.width,
                        0.0..=1.0,
                    ));
                });
            },
        );
    }
}
