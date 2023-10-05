use crate::app::context::Context;
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

impl Visualization<'_> {
    pub(super) fn view(self, ui: &mut Ui) {
        ui.collapsing(RichText::new("📊 Visualization").heading(), |ui| {
            // ui.horizontal(|ui| {
            //     ui.label("Chart:");
            //     ComboBox::from_id_source("chart")
            //         .selected_text(format!("{:?}", self.context.settings.visualization.chart))
            //         .show_ui(ui, |ui| {
            //             ui.selectable_value(
            //                 &mut self.context.settings.visualization.chart,
            //                 Chart::Bar,
            //                 "Bar",
            //             );
            //             ui.selectable_value(
            //                 &mut self.context.settings.visualization.chart,
            //                 Chart::Pie,
            //                 "Pie",
            //             );
            //         });
            // });
            ui.horizontal(|ui| {
                ui.label("Legend:");
                ui.checkbox(&mut self.context.settings.visualization.legend, "");
            });
            ui.horizontal(|ui| {
                ui.label("Stacked:");
                ui.checkbox(&mut self.context.settings.visualization.stacked, "");
            });
            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(Slider::new(
                    &mut self.context.settings.visualization.width,
                    0.0..=1.0,
                ));
            });
        });
    }
}
