use self::settings::Settings;
use super::Behavior;
use crate::{
    app::computers::{CompositionComputed, CompositionKey},
    localization::localize,
    utils::{DataFrameExt, SeriesExt},
};
use anyhow::Result;
use egui::{
    collapsing_header::paint_default_icon, Align, Align2, Color32, Frame, Grid, Id, Layout, Margin,
    Pos2, RichText, ScrollArea, Sense, Sides, TextWrapMode, Ui, Vec2,
};
use egui_extras::TableBuilder;
use egui_phosphor::regular::LIST;
use egui_table::{
    AutoSizeMode, CellInfo, Column, HeaderCellInfo, HeaderRow, PrefetchInfo, Table, TableDelegate,
};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    f64::NAN,
    iter::{once, zip},
};
use tracing::error;

/// Central composition pane
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Pane {
    /// Composition special settings
    pub(in crate::app) settings: Settings,
}

impl Pane {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui, behavior: &mut Behavior) {
        let Some(entry) = behavior.data.entries.iter_mut().find(|entry| entry.checked) else {
            return;
        };
        TableDemo::new(behavior, &self.settings).ui(ui);
        // if let Err(error) = || -> Result<()> {
        //     if self.settings.compositions.is_empty() {
        //         return Ok(());
        //     }
        //     let data_frame = ui.memory_mut(|memory| {
        //         memory
        //             .caches
        //             .cache::<CompositionComputed>()
        //             .get(CompositionKey {
        //                 data_frame: &entry.fatty_acids,
        //                 settings: &self.settings,
        //             })
        //     });
        //     let height = ui.spacing().interact_size.y;
        //     let width = ui.spacing().interact_size.x;
        //     let total_rows = data_frame.height();
        //     let mut compositions = Vec::new();
        //     for (index, composition) in self.settings.compositions.iter().enumerate() {
        //         compositions.push((
        //             composition,
        //             data_frame.destruct(&format!("Composition{index}")),
        //         ));
        //     }
        //     TableBuilder::new(ui)
        //         .columns(Column::auto().at_least(width), compositions.len() + 1)
        //         // .column(Column::auto_with_initial_suggestion(width))
        //         .auto_shrink(false)
        //         .resizable(behavior.settings.resizable)
        //         .striped(true)
        //         .header(height, |mut row| {
        //             // Compositions
        //             for (composition, _) in &compositions {
        //                 row.col(|ui| {
        //                     ui.heading(composition.text())
        //                         .on_hover_text(composition.hover_text());
        //                 });
        //             }
        //             // Value
        //             row.col(|ui| {
        //                 ui.heading(localize!("value"));
        //             });
        //             // // Species
        //             // row.col(|ui| {
        //             //     ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
        //             //     ui.heading(localize!("species"));
        //             // });
        //         })
        //         .body(|mut body| {
        //             body.ui_mut().visuals_mut().button_frame = false;
        //             let precision = |value| format!("{value:.*}", self.settings.precision);
        //             body.rows(height, total_rows + 1, |mut row| {
        //                 let row_index = row.index();
        //                 if row_index < total_rows {
        //                     // Compositions
        //                     for (composition_index, (_, composition)) in
        //                         compositions.iter().enumerate()
        //                     {
        //                         row.col(|ui| {
        //                             ui.horizontal(|ui| {
        //                                 let mut value =
        //                                     composition.f64("Value").get(row_index).unwrap();
        //                                 let is_leaf = composition_index + 1 == compositions.len();
        //                                 if is_leaf {
        //                                     let species = data_frame
        //                                         .list("Species")
        //                                         .get_as_series(row_index)
        //                                         .unwrap();
        //                                     let rect = ui
        //                                         .menu_button(LIST, |ui| {
        //                                             ScrollArea::vertical().show(ui, |ui| {
        //                                                 Grid::new("species").show(ui, |ui| {
        //                                                     let species = species.r#struct();
        //                                                     let labels = species
        //                                                         .field_by_name("Label")
        //                                                         .unwrap();
        //                                                     let labels = labels.str().unwrap();
        //                                                     let values = species
        //                                                         .field_by_name("Value")
        //                                                         .unwrap();
        //                                                     let values = values.f64().unwrap();
        //                                                     for (index, (label, value)) in
        //                                                         zip(labels, values).enumerate()
        //                                                     {
        //                                                         ui.label(index.to_string());
        //                                                         ui.label(label.unwrap());
        //                                                         let value = value.unwrap();
        //                                                         ui.label(precision(value))
        //                                                             .on_hover_text(
        //                                                                 value.to_string(),
        //                                                             );
        //                                                         ui.end_row();
        //                                                     }
        //                                                 });
        //                                             });
        //                                         })
        //                                         .response
        //                                         .on_hover_text(species.len().to_string())
        //                                         .rect;
        //                                     let painter = ui.painter_at(rect);
        //                                     painter.text(
        //                                         // Pos2::ZERO,
        //                                         Pos2::new(10.0, 10.0),
        //                                         Align2::CENTER_CENTER,
        //                                         precision(value),
        //                                         Default::default(),
        //                                         Color32::WHITE,
        //                                     );
        //                                     // paint
        //                                 }
        //                                 let response = ui.label(
        //                                     composition["Key"].str_value(row_index).unwrap(),
        //                                 );
        //                                 if !is_leaf {
        //                                     response.on_hover_ui(|ui| {
        //                                         // composition.group_by(["column_name"]).f64("Value");
        //                                         // let mut value =
        //                                         //     composition.f64("Value").get(index).unwrap();
        //                                         if self.settings.percent {
        //                                             value *= 100.0;
        //                                         }
        //                                         Grid::new(ui.auto_id_with("composition")).show(
        //                                             ui,
        //                                             |ui| {
        //                                                 ui.label("Value");
        //                                                 ui.label(precision(value))
        //                                                     .on_hover_text(value.to_string());
        //                                             },
        //                                         );
        //                                     });
        //                                 }
        //                             });
        //                         });
        //                     }
        //                     // Value
        //                     if let Some((_, composition)) = compositions.last() {
        //                         row.col(|ui| {
        //                             let mut value =
        //                                 composition.f64("Value").get(row_index).unwrap_or(NAN);
        //                             if self.settings.percent {
        //                                 value *= 100.0;
        //                             }
        //                             ui.label(precision(value)).on_hover_text(value.to_string());
        //                         });
        //                     }
        //                 } else {
        //                     // Compositions
        //                     for _ in 0..compositions.len() {
        //                         row.col(|_| {});
        //                     }
        //                     // Value
        //                     if let Some((_, composition)) = compositions.last() {
        //                         row.col(|ui| {
        //                             let values = composition.f64("Value");
        //                             let mut sum = values.sum().unwrap_or(NAN);
        //                             if self.settings.percent {
        //                                 sum *= 100.;
        //                             }
        //                             ui.heading(precision(sum)).on_hover_ui(|ui| {
        //                                 ui.heading(localize!("properties"));
        //                                 ui.label(format!("Count: {}", values.len()));
        //                             });
        //                         });
        //                     }
        //                 }
        //             });
        //         });
        //     Ok(())
        // }() {
        //     error!(%error);
        // }
    }
}

struct TableDemo<'a, 'b> {
    behavior: &'a mut Behavior<'b>,
    settings: &'a Settings,
    // is_row_expanded: BTreeMap<u64, bool>,
    // prefetched: Vec<PrefetchInfo>,
}

impl<'a, 'b> TableDemo<'a, 'b> {
    fn new(behavior: &'a mut Behavior<'b>, settings: &'a Settings) -> Self {
        Self { behavior, settings }
    }

    fn ui(&mut self, ui: &mut Ui) {
        let Some(entry) = self
            .behavior
            .data
            .entries
            .iter_mut()
            .find(|entry| entry.checked)
        else {
            return;
        };
        let id_salt = Id::new("CompositionTable");
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        let data_frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<CompositionComputed>()
                .get(CompositionKey {
                    data_frame: &entry.fatty_acids,
                    settings: &self.settings,
                })
        });
        const RANGE: usize = 3;
        let num_rows = data_frame.height() as _;
        let num_columns = 1 + self.behavior.data.entries.len() * RANGE;
        Table::new()
            .id_salt(id_salt)
            .num_rows(num_rows)
            .columns(vec![Column::default(); num_columns])
            .num_sticky_cols(1)
            .headers([
                HeaderRow {
                    height: height,
                    groups: once(0..1)
                        .chain((0..self.behavior.data.entries.len()).map(|index| {
                            let start = index * RANGE + 1;
                            start..start + RANGE
                        }))
                        .collect(),
                    // groups: vec![0..1, 1..4, 4..7, 7..10],
                    // groups: vec![0..3, 3..6, 6..9],
                    // groups: vec![0..1, 1..4, 4..8],
                },
                HeaderRow::new(height),
            ])
            .auto_size_mode(AutoSizeMode::OnParentResize)
            .show(ui, self);
    }
}

impl TableDelegate for TableDemo<'_, '_> {
    fn header_cell_ui(&mut self, ui: &mut Ui, cell: &HeaderCellInfo) {
        match (cell.row_nr, cell.group_index, &cell.col_range) {
            (0, 0, _) => {}
            (0, column, _) => {
                let entry = &self.behavior.data.entries[column - 1];
                ui.heading(&entry.name);
            }
            (1, 0, _) => {
                Sides::new().height(ui.available_height()).show(
                    ui,
                    |ui| {
                        ui.heading("Row");
                    },
                    |ui| {
                        ui.label("⬇");
                    },
                );
            }
            (row, column, range) => {
                ui.heading(format!("Cell {row}, {column}, {range:?}"));
            }
        }
    }

    fn cell_ui(&mut self, ui: &mut Ui, cell: &CellInfo) {
        ui.label(format!("Column {}", cell.row_nr));
    }
}

pub(in crate::app) mod settings;
