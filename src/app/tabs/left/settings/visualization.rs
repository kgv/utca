use crate::app::{
    context::{settings::visualization::Source, Context},
    tabs::CentralTab,
    view::View,
    MAX_PRECISION,
};
use egui::{ComboBox, RichText, Slider, Ui};

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
            RichText::new(CentralTab::Visualization.title()).heading(),
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
                    ui.label("Drag:");
                    ui.checkbox(&mut context.settings.visualization.drag.x, "")
                        .on_hover_text("x");
                    ui.checkbox(&mut context.settings.visualization.drag.y, "")
                        .on_hover_text("y");
                });
                ui.horizontal(|ui| {
                    ui.label("Scroll:");
                    ui.checkbox(&mut context.settings.visualization.scroll, "");
                });
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.add(Slider::new(
                        &mut context.settings.visualization.width,
                        0.0..=1.0,
                    ));
                });
                ui.horizontal(|ui| {
                    ui.label("Source:");
                    ComboBox::from_id_source("source")
                        .selected_text(context.settings.visualization.source.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.visualization.source,
                                Source::Composition,
                                Source::Composition.text(),
                            )
                            .on_hover_text(Source::Comparison.hover_text());
                            ui.selectable_value(
                                &mut context.settings.visualization.source,
                                Source::Comparison,
                                Source::Comparison.text(),
                            )
                            .on_hover_text(Source::Comparison.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.visualization.source.hover_text());
                });
                ui.separator();
                if context.settings.visualization.source == Source::Comparison {
                    ui.horizontal(|ui| {
                        ui.label("Links:");
                        ui.horizontal(|ui| {
                            ui.label("Axis:");
                            ui.checkbox(&mut context.settings.visualization.links.axis.x, "")
                                .on_hover_text("x");
                            ui.checkbox(&mut context.settings.visualization.links.axis.y, "")
                                .on_hover_text("y");
                        });
                        ui.horizontal(|ui| {
                            ui.label("Cursor:");
                            ui.checkbox(&mut context.settings.visualization.links.cursor.x, "")
                                .on_hover_text("x");
                            ui.checkbox(&mut context.settings.visualization.links.cursor.y, "")
                                .on_hover_text("y");
                        });
                    });
                }
            },
        );
    }
}
