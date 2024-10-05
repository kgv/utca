use crate::{
    localization::localize,
    utils::{ColumnExt, SeriesExt},
};
use egui::{Color32, Grid, Response, RichText, ScrollArea, Sides, Ui, Widget};
use egui_phosphor::regular::LIST;
use polars::prelude::*;
use std::{f64::NAN, iter::zip};

/// Cell widget
pub(in crate::app) struct Cell<'a> {
    pub(in crate::app) column: &'a Column,
    pub(in crate::app) row: usize,
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
}

impl Widget for Cell<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let fields = self.column.r#struct().fields_as_series();
        let (species, values) = fields.split_last().unwrap();
        let values: Vec<_> = values
            .into_iter()
            .map(|field| (field.name(), field.f64().unwrap().get(self.row)))
            .collect();

        let mut value = values.last().and_then(|&(_, value)| value).unwrap_or(NAN);
        if self.percent {
            value *= 100.0;
        }
        let mut text = RichText::from(format!("{value:.*}", self.precision));
        if value.is_nan() {
            text = text.color(Color32::RED);
        }
        Sides::new()
            .show(
                ui,
                |ui| {
                    ui.label(text).on_hover_ui(|ui| {
                        ui.heading(localize!("values"));
                        Grid::new(ui.next_auto_id()).show(ui, |ui| {
                            // Values
                            for (name, value) in values {
                                ui.label(name.to_string());
                                let mut value = value.unwrap_or(NAN);
                                if self.percent {
                                    value *= 100.0;
                                }
                                ui.label(AnyValue::from(value).to_string());
                                ui.end_row();
                            }
                        });
                    })
                },
                |ui| {
                    ui.add(Species {
                        species: species.list().unwrap().get_as_series(self.row),
                        percent: self.percent,
                    })
                },
            )
            .0
    }
}

/// Species widget
struct Species {
    pub(in crate::app) species: Option<Series>,
    pub(in crate::app) percent: bool,
}

impl Widget for Species {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.visuals_mut().button_frame = false;
        ui.menu_button(LIST, |ui| {
            ui.heading(localize!("species"));
            ScrollArea::vertical().show(ui, |ui| {
                Grid::new(ui.next_auto_id()).show(ui, |ui| {
                    if let Some(series) = self.species {
                        let fields = series.r#struct().fields_as_series();
                        let species = fields[0].str().unwrap();
                        let value = fields[1].f64().unwrap();
                        for (species, value) in zip(species, value) {
                            let text = if let Some(species) = species {
                                RichText::new(species)
                            } else {
                                RichText::new("None").color(Color32::RED)
                            };
                            ui.label(text);
                            let mut value = value.unwrap_or(NAN);
                            if self.percent {
                                value *= 100.0;
                            }
                            ui.label(AnyValue::from(value).to_string());
                            ui.end_row();
                        }
                    }
                });
            });
        })
        .response
    }
}
