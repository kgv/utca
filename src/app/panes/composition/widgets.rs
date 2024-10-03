use crate::localization::localize;
use egui::{Color32, Grid, Response, RichText, Ui, Widget};
use polars::prelude::*;

/// Cell widget
pub(in crate::app) struct Cell<'a> {
    pub(in crate::app) value: Option<f64>,
    pub(in crate::app) species: Option<&'a StringChunked>,
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
}

impl Widget for Cell<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut value = self.value.unwrap_or(f64::NAN);
        if self.percent {
            value *= 100.0;
        }
        let mut text = RichText::from(format!("{value:.*}", self.precision));
        if value.is_nan() {
            text = text.color(Color32::RED);
        }
        ui.label(text).on_hover_ui(|ui| {
            ui.heading(localize!("properties"));
            Grid::new(ui.next_auto_id()).show(ui, |ui| {
                ui.label(localize!("value"));
                ui.label(value.to_string());
                ui.end_row();

                if let Some(species) = self.species {
                    for (index, species) in species.into_iter().enumerate() {
                        if index == 0 {
                            ui.label(localize!("species"));
                        } else {
                            ui.label("");
                        }
                        let text = if let Some(species) = species {
                            RichText::new(species)
                        } else {
                            RichText::new("None").color(Color32::RED)
                        };
                        ui.label(text);
                        ui.end_row();
                    }
                }
            });
        })
    }
}
