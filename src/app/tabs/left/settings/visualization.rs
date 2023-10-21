use crate::app::{context::Context, view::View};
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
        ui.collapsing(RichText::new("📊 Visualization").heading(), |ui| {
            // ui.horizontal(|ui| {
            //     ui.label("Chart:");
            //     ComboBox::from_id_source("chart")
            //         .selected_text(format!("{:?}", context.settings.visualization.chart))
            //         .show_ui(ui, |ui| {
            //             ui.selectable_value(
            //                 &mut context.settings.visualization.chart,
            //                 Chart::Bar,
            //                 "Bar",
            //             );
            //             ui.selectable_value(
            //                 &mut context.settings.visualization.chart,
            //                 Chart::Pie,
            //                 "Pie",
            //             );
            //         });
            // });
            ui.horizontal(|ui| {
                ui.label("Legend:");
                ui.checkbox(&mut context.settings.visualization.legend, "");
            });
            ui.horizontal(|ui| {
                ui.label("Stacked:");
                ui.checkbox(&mut context.settings.visualization.stacked, "");
            });
            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(Slider::new(
                    &mut context.settings.visualization.width,
                    0.0..=1.0,
                ));
            });
        });
    }
}
