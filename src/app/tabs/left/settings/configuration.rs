use crate::app::{
    context::{
        settings::configuration::{C, U},
        Context,
    },
    tabs::CentralTab,
    view::View,
    MAX_PRECISION,
};
use egui::{DragValue, RichText, Slider, Ui};

/// Left configuration tab
pub(super) struct Configuration<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Configuration<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Configuration<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        ui.collapsing(
            RichText::new(CentralTab::Configuration.to_string()).heading(),
            |ui| {
                ui.horizontal(|ui| {
                    ui.toggle_value(&mut context.settings.configuration.resizable, "↔ Resizable")
                        .on_hover_text("Resize table columns");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    let precision = &mut context.settings.configuration.precision;
                    if ui.add(Slider::new(precision, 0..=MAX_PRECISION)).changed()
                        && context.settings.link
                    {
                        context.settings.calculation.precision = *precision;
                        context.settings.composition.precision = *precision;
                        context.settings.visualization.precision = *precision;
                        context.settings.comparison.precision = *precision;
                    }
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("C:");
                    ui.add(
                        DragValue::new(&mut context.settings.configuration.c.start)
                            .clamp_range(C::MIN..=context.settings.configuration.c.end),
                    )
                    .on_hover_text("Min");
                    ui.add(
                        DragValue::new(&mut context.settings.configuration.c.end)
                            .clamp_range(context.settings.configuration.c.start..=C::MAX),
                    )
                    .on_hover_text("Max");
                    ui.label("U:");
                    ui.add(
                        DragValue::new(&mut context.settings.configuration.u)
                            .clamp_range(0..=U::max(context.settings.configuration.c.end)),
                    )
                    .on_hover_text("Max");
                });
            },
        );
    }
}
