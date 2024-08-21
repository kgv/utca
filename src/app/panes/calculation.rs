use crate::app::{
    computers::calculator::{Calculated, Key as CalculatorKey},
    panes::Settings as PanesSettings,
    MAX_PRECISION,
};
use anyhow::Result;
use egui::{Color32, CursorIcon, Direction, Id, Layout, Response, RichText, Sense, Slider, Ui};
use egui_ext::{ClickedLabel, TableBodyExt, TableRowExt};
use egui_extras::{Column, TableBuilder};
use egui_tiles::UiResponse;
use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};
use tracing::error;

pub(crate) const TITLE: &str = "Calculation";

/// Central calculation pane
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(crate) struct Pane {
    pub(crate) data_frame: DataFrame,
    pub(crate) settings: Settings,
}

impl Pane {
    pub(crate) fn ui(&mut self, ui: &mut Ui, settings: &PanesSettings) -> UiResponse {
        let response = ui.heading(TITLE).on_hover_cursor(CursorIcon::Grab);
        let dragged = response.dragged();
        if let Err(error) = || -> Result<()> {
            let Some(data_frame) = ui.data_mut(|data| data.get_temp(Id::new("Configuration")))
            else {
                ui.spinner();
                return Ok(());
            };
            let data_frame = ui.memory_mut(|memory| {
                memory.caches.cache::<Calculated>().get(CalculatorKey {
                    data_frame: &data_frame,
                })
            });
            Ok(())
        }() {
            error!(%error);
        }
        if dragged {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }

    fn try_ui(self, ui: &mut Ui, settings: &PanesSettings) {
        // context.calculate(ui);
        // let data_frame = ui.memory_mut(|memory| memory.caches.cache::<Calculated>().get(Key {}));
        // let p = context.settings.calculation.precision;
        // let height = ui.spacing().interact_size.y;
        // let width = ui.spacing().interact_size.x;
        // TableBuilder::new(ui)
        //     .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
        //     .column(Column::auto_with_initial_suggestion(width))
        //     .columns(Column::auto(), COLUMNS)
        //     .auto_shrink(false)
        //     .resizable(context.settings.calculation.resizable)
        //     .striped(true)
        //     .header(height, |mut row| {
        //         let mut response = row.col(|_| {}).1;
        //         response.sense = Sense::click();
        //         // 1,2,3-TAGs
        //         row.col(|ui| {
        //             response |= ui.heading("1,2,3").on_hover_text("1,2,3-TAGs");
        //         });
        //         // 1,2/2,3-DAGs
        //         row.col(|ui| {
        //             response |= ui.heading("1,2/2,3").on_hover_text("1,2/2,3-DAGs");
        //         });
        //         // 2-MAGs
        //         row.col(|ui| {
        //             response |= ui.heading("2").on_hover_text("2-MAGs");
        //         });
        //         // 1,3-DAGs
        //         row.col(|ui| {
        //             response |= ui.heading("1,3").on_hover_text("1,3-DAGs");
        //         });
        //         row.response().union(response).context_menu(|ui| {
        //             let id = Id::new("show");
        //             let mut current_value = ui
        //                 .data(|data| data.get_temp::<Show>(id))
        //                 .unwrap_or_default();
        //             let mut response = ui.selectable_value(
        //                 &mut current_value,
        //                 Show::ExperimentalValue,
        //                 "Experimental value",
        //             );
        //             response |= ui.selectable_value(
        //                 &mut current_value,
        //                 Show::EnrichmentFactor,
        //                 "Enrichment factor",
        //             );
        //             response |= ui.selectable_value(
        //                 &mut current_value,
        //                 Show::SelectivityFactor,
        //                 "Selectivity factor",
        //             );
        //             if response.changed() {
        //                 ui.data_mut(|data| data.insert_temp(id, current_value));
        //                 ui.close_menu();
        //             }
        //         });
        //     })
        //     .body(|mut body| {
        //         for (label, (mut tag123, mut dag1223, mut mag2, mut dag13)) in context
        //             .state
        //             .entry()
        //             .meta
        //             .labels
        //             .iter()
        //             .zip(context.state.entry().data.calculated.zip())
        //         {
        //             body.row(height, |mut row| {
        //                 row.left_align_col(|ui| {
        //                     ui.heading(label);
        //                 });
        //                 // 1,2,3-TAGs
        //                 row.right_align_col(|ui| {
        //                     if context.settings.calculation.percent {
        //                         tag123.experimental.normalized *= 100.0;
        //                         tag123.theoretical.normalized *= 100.0;
        //                     }
        //                     let response = ui
        //                         // TODO: theoretical
        //                         .label(format!("{:.p$}", tag123.experimental.normalized))
        //                         // .label(format!("{:.p$}", tag123.theoretical.normalized))
        //                         .on_hover_ui(|ui| {
        //                             ui.vertical(|ui| {
        //                                 if context.settings.calculation.theoretical {
        //                                     ui.heading("Experimental:");
        //                                 }
        //                                 ui.label(tag123.experimental.normalized.to_string());
        //                                 if context.settings.calculation.unnormalized {
        //                                     let mut unnormalized = tag123.experimental.unnormalized;
        //                                     if context.settings.calculation.pchelkin {
        //                                         unnormalized *= 10.0;
        //                                     }
        //                                     ui.label(format!("Unnormalized: {unnormalized}"));
        //                                 }
        //                             });
        //                         });
        //                     if context.settings.calculation.theoretical {
        //                         response.on_hover_ui(|ui| {
        //                             ui.heading("Theoretical:");
        //                             ui.label(tag123.theoretical.normalized.to_string());
        //                             if context.settings.calculation.unnormalized {
        //                                 let mut unnormalized = tag123.theoretical.unnormalized;
        //                                 if context.settings.calculation.pchelkin {
        //                                     unnormalized *= 10.0;
        //                                 }
        //                                 ui.label(format!("Unnormalized: {unnormalized}"));
        //                             }
        //                             if context.settings.calculation.selectivity {
        //                                 let selectivity = tag123.theoretical.normalized
        //                                     / tag123.experimental.unnormalized;
        //                                 ui.label(format!("Selectivity: {selectivity}"));
        //                             }
        //                         });
        //                     }
        //                 });
        //                 // 1,2/2,3-DAGs
        //                 row.right_align_col(|ui| {
        //                     if context.settings.calculation.percent {
        //                         dag1223.experimental.normalized *= 100.0;
        //                         dag1223.theoretical.normalized *= 100.0;
        //                     }
        //                     let response = ui
        //                         // TODO: theoretical
        //                         .label(format!("{:.p$}", dag1223.value().normalized))
        //                         // .label(format!("{:.p$}", dag1223.theoretical.normalized))
        //                         .on_hover_ui(|ui| {
        //                             if !dag1223.is_experimental() {
        //                                 ui.colored_label(
        //                                     Color32::YELLOW,
        //                                     "⚠ Warning: it's a theoretical value",
        //                                 );
        //                                 ui.label(dag1223.theoretical.normalized.to_string());
        //                             } else {
        //                                 if context.settings.calculation.theoretical {
        //                                     ui.heading("Experimental:");
        //                                 }
        //                                 ui.label(dag1223.experimental.normalized.to_string());
        //                                 if context.settings.calculation.unnormalized {
        //                                     let mut unnormalized =
        //                                         dag1223.experimental.unnormalized;
        //                                     if context.settings.calculation.pchelkin {
        //                                         unnormalized *= 10.0;
        //                                     }
        //                                     ui.label(format!("Unnormalized: {unnormalized}"));
        //                                 }
        //                             }
        //                         });
        //                     if context.settings.calculation.theoretical {
        //                         response.on_hover_ui(|ui| {
        //                             ui.heading("Theoretical:");
        //                             ui.label(dag1223.theoretical.normalized.to_string());
        //                             if context.settings.calculation.unnormalized {
        //                                 let mut unnormalized = dag1223.theoretical.unnormalized;
        //                                 if context.settings.calculation.pchelkin {
        //                                     unnormalized *= 10.0;
        //                                 }
        //                                 ui.label(format!("Unnormalized: {unnormalized}"));
        //                             }
        //                             if context.settings.calculation.selectivity {
        //                                 let selectivity = dag1223.theoretical.normalized
        //                                     / tag123.experimental.unnormalized;
        //                                 ui.label(format!("Selectivity: {selectivity}"));
        //                             }
        //                         });
        //                     }
        //                 });
        //                 // 2-MAGs
        //                 row.right_align_col(|ui| {
        //                     if context.settings.calculation.percent {
        //                         mag2.experimental.normalized *= 100.0;
        //                         mag2.theoretical.normalized *= 100.0;
        //                     }
        //                     let response = ui
        //                         // TODO: theoretical
        //                         .label(format!("{:.p$}", mag2.value().normalized))
        //                         // .label(format!("{:.p$}", mag2.theoretical.normalized))
        //                         .on_hover_ui(|ui| {
        //                             if !mag2.is_experimental() {
        //                                 ui.colored_label(
        //                                     Color32::YELLOW,
        //                                     "⚠ Warning: it's a theoretical value",
        //                                 );
        //                                 ui.label(mag2.theoretical.normalized.to_string());
        //                             } else {
        //                                 if context.settings.calculation.theoretical {
        //                                     ui.heading("Experimental:");
        //                                 }
        //                                 ui.label(mag2.experimental.normalized.to_string());
        //                                 if context.settings.calculation.unnormalized {
        //                                     let mut unnormalized = mag2.experimental.unnormalized;
        //                                     if context.settings.calculation.pchelkin {
        //                                         unnormalized *= 10.0;
        //                                     }
        //                                     ui.label(format!("Unnormalized: {unnormalized}"));
        //                                 }
        //                                 if context.settings.calculation.selectivity_factor {
        //                                     let selectivity_factor = mag2.experimental.normalized
        //                                         / tag123.experimental.normalized;
        //                                     ui.label(format!(
        //                                         "Selectivity factor: {selectivity_factor}"
        //                                     ));
        //                                 }
        //                             }
        //                         });
        //                     if context.settings.calculation.theoretical {
        //                         response.on_hover_ui(|ui| {
        //                             ui.heading("Theoretical:");
        //                             ui.label(mag2.theoretical.normalized.to_string());
        //                             if context.settings.calculation.unnormalized {
        //                                 let mut unnormalized = mag2.theoretical.unnormalized;
        //                                 if context.settings.calculation.pchelkin {
        //                                     unnormalized *= 10.0;
        //                                 }
        //                                 ui.label(format!("Unnormalized: {unnormalized}"));
        //                             }
        //                             if context.settings.calculation.selectivity {
        //                                 let selectivity = mag2.theoretical.normalized
        //                                     / tag123.experimental.unnormalized;
        //                                 ui.label(format!("Selectivity: {selectivity}"));
        //                             }
        //                         });
        //                     }
        //                 });
        //                 // 1,3-DAGs
        //                 row.right_align_col(|ui| {
        //                     let show = ui
        //                         .data(|data| data.get_temp::<Show>(Id::new("show")))
        //                         .unwrap_or_default();
        //                     let mut normalized =
        //                         dag13.value(context.settings.calculation.from).normalized;
        //                     if context.settings.calculation.percent {
        //                         normalized *= 100.0;
        //                     }
        //                     match show {
        //                         Show::ExperimentalValue => {
        //                             ui.label(format!("{normalized:.p$}"));
        //                         }
        //                         Show::EnrichmentFactor => {
        //                             let enrichment_factor =
        //                                 normalized / tag123.experimental.normalized;
        //                             ui.label(format!("{enrichment_factor:.p$}"));
        //                         }
        //                         Show::SelectivityFactor => {
        //                             let selectivity_factor =
        //                                 normalized / tag123.experimental.normalized;
        //                             ui.label(format!("{selectivity_factor:.p$}"));
        //                         }
        //                     }

        //                     // let Normalizable {
        //                     //     mut unnormalized,
        //                     //     mut normalized,
        //                     // } = dag13.value(context.settings.calculation.from);
        //                     // if context.settings.calculation.percent {
        //                     //     normalized *= 100.0;
        //                     // }
        //                     // ui.label(format!("{normalized:.p$}")).on_hover_ui(|ui| {
        //                     //     ui.label(normalized.to_string());
        //                     //     if context.settings.calculation.unnormalized {
        //                     //         if context.settings.calculation.pchelkin {
        //                     //             unnormalized *= 10.0;
        //                     //         }
        //                     //         ui.label(format!("Unnormalized: {unnormalized}"));
        //                     //     }
        //                     //     if context.settings.calculation.selectivity {
        //                     //         let selectivity = normalized / tag123.experimental.unnormalized;
        //                     //         ui.label(format!("Selectivity: {selectivity}"));
        //                     //     }
        //                     //     if context.settings.calculation.selectivity_factor {
        //                     //         let selectivity_factor =
        //                     //             normalized / tag123.experimental.normalized;
        //                     //         ui.label(format!("Selectivity factor: {selectivity_factor}"));
        //                     //     }
        //                     //     if context.settings.calculation.enrichment_factor {
        //                     //         let selectivity_factor =
        //                     //             normalized / tag123.experimental.normalized;
        //                     //         let mut u123 = 0.0;
        //                     //         let mut u2 = 0.0;
        //                     //         let (u123, u2) = context.unsaturated().fold(
        //                     //             (0.0, 0.0),
        //                     //             |(u123, u2), index| {
        //                     //                 (
        //                     //                     u123 + context
        //                     //                         .state
        //                     //                         .entry()
        //                     //                         .data
        //                     //                         .calculated
        //                     //                         .tags123
        //                     //                         .experimental
        //                     //                         .normalized[index],
        //                     //                     u2 + context
        //                     //                         .state
        //                     //                         .entry()
        //                     //                         .data
        //                     //                         .calculated
        //                     //                         .tags123
        //                     //                         .experimental
        //                     //                         .normalized[index],
        //                     //                 )
        //                     //             },
        //                     //         );
        //                     //         // ui.label(format!("Enrichment factor: {enrichment_factor}"));
        //                     //     }
        //                     // });
        //                 });
        //             });
        //         }
        //         // Footer
        //         let calculated = &context.state.entry().data.calculated;
        //         body.separate(height / 2.0, 5);
        //         body.row(height, |mut row| {
        //             row.col(|_| {});
        //             // 1,2,3-TAGs
        //             row.right_align_col(|ui| {
        //                 let mut sum: f64 = calculated.tags123.experimental.normalized.iter().sum();
        //                 if context.settings.calculation.percent {
        //                     sum *= 100.0;
        //                 }
        //                 ui.label(format!("{sum:.p$}")).on_hover_ui(|ui| {
        //                     ui.label(sum.to_string());
        //                 });
        //             });
        //             // 1,2/2,3-DAGs
        //             row.right_align_col(|ui| {
        //                 let mut sum: f64 = calculated.dags1223.experimental.normalized.iter().sum();
        //                 if context.settings.calculation.percent {
        //                     sum *= 100.0;
        //                 }
        //                 ui.label(format!("{sum:.p$}")).on_hover_ui(|ui| {
        //                     ui.label(sum.to_string());
        //                 });
        //             });
        //             // 2-MAGs
        //             row.right_align_col(|ui| {
        //                 let mut sum: f64 = calculated.mags2.experimental.normalized.iter().sum();
        //                 if context.settings.calculation.percent {
        //                     sum *= 100.0;
        //                 }
        //                 ui.label(format!("{sum:.p$}")).on_hover_ui(|ui| {
        //                     ui.label(sum.to_string());
        //                 });
        //             });
        //             // 1,3-DAGs
        //             row.right_align_col(|ui| {
        //                 let mut sum: f64 = calculated
        //                     .dags13
        //                     .value(context.settings.calculation.from)
        //                     .normalized
        //                     .iter()
        //                     .sum();
        //                 if context.settings.calculation.percent {
        //                     sum *= 100.0;
        //                 }
        //                 ui.label(format!("{sum:.p$}")).on_hover_ui(|ui| {
        //                     ui.label(sum.to_string());
        //                 });
        //             });
        //         });
        //     });
    }
}

/// Calculation settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) percent: bool,
    pub(crate) precision: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            percent: true,
            precision: 1,
        }
    }
}

impl Settings {
    pub(crate) fn ui(&mut self, ui: &mut Ui) -> UiResponse {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(TITLE).heading(), |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Precision:");
                ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
            });
            ui.separator();
        });
        UiResponse::None
    }
}

// /// Column show
// #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
// enum Show {
//     #[default]
//     ExperimentalValue,
//     EnrichmentFactor,
//     SelectivityFactor,
// }
