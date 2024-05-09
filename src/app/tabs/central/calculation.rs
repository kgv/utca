use crate::app::{
    context::{settings::calculation::From, state::calculation::Normalizable, Context},
    view::View,
};
use egui::{Color32, Direction, Layout, Ui};
use egui_ext::{TableBodyExt, TableRowExt};
use egui_extras::{Column, TableBuilder};

const COLUMNS: usize = 4;

/// Central calculation tab
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
        context.calculate(ui);
        let p = context.settings.calculation.precision;
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .column(Column::auto_with_initial_suggestion(width))
            .columns(Column::auto(), COLUMNS)
            .auto_shrink(false)
            .resizable(context.settings.calculation.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|_| {});
                // 1,2,3-TAGs
                row.col(|ui| {
                    ui.heading("1,2,3").on_hover_text("1,2,3-TAGs");
                });
                // 1,2/2,3-DAGs
                row.col(|ui| {
                    ui.heading("1,2/2,3").on_hover_text("1,2/2,3-DAGs");
                });
                // 2-MAGs
                row.col(|ui| {
                    ui.heading("2").on_hover_text("2-MAGs");
                });
                // 1,3-DAGs
                row.col(|ui| {
                    ui.heading("1,3").on_hover_text("1,3-DAGs");
                });
            })
            .body(|mut body| {
                for (label, (mut tag123, mut dag1223, mut mag2, mut dag13)) in context
                    .state
                    .entry()
                    .meta
                    .labels
                    .iter()
                    .zip(context.state.entry().data.calculated.zip())
                {
                    body.row(height, |mut row| {
                        row.left_align_col(|ui| {
                            ui.heading(label);
                        });
                        // 1,2,3-TAGs
                        row.right_align_col(|ui| {
                            if context.settings.calculation.percent {
                                tag123.experimental.normalized *= 100.0;
                                tag123.theoretical.normalized *= 100.0;
                            }
                            let response = ui
                                // TODO: theoretical
                                .label(format!("{:.p$}", tag123.experimental.normalized))
                                // .label(format!("{:.p$}", tag123.theoretical.normalized))
                                .on_hover_ui(|ui| {
                                    ui.vertical(|ui| {
                                        if context.settings.calculation.theoretical {
                                            ui.heading("Experimental:");
                                        }
                                        ui.label(tag123.experimental.normalized.to_string());
                                        if context.settings.calculation.unnormalized {
                                            let mut unnormalized = tag123.experimental.unnormalized;
                                            if context.settings.calculation.pchelkin {
                                                unnormalized *= 10.0;
                                            }
                                            ui.label(format!("Unnormalized: {unnormalized}"));
                                        }
                                    });
                                });
                            if context.settings.calculation.theoretical {
                                response.on_hover_ui(|ui| {
                                    ui.heading("Theoretical:");
                                    ui.label(tag123.theoretical.normalized.to_string());
                                    if context.settings.calculation.unnormalized {
                                        let mut unnormalized = tag123.theoretical.unnormalized;
                                        if context.settings.calculation.pchelkin {
                                            unnormalized *= 10.0;
                                        }
                                        ui.label(format!("Unnormalized: {unnormalized}"));
                                    }
                                    if context.settings.calculation.selectivity {
                                        let selectivity = tag123.theoretical.normalized
                                            / tag123.experimental.unnormalized;
                                        ui.label(format!("Selectivity: {selectivity}"));
                                    }
                                });
                            }
                        });
                        // 1,2/2,3-DAGs
                        row.right_align_col(|ui| {
                            if context.settings.calculation.percent {
                                dag1223.experimental.normalized *= 100.0;
                                dag1223.theoretical.normalized *= 100.0;
                            }
                            let response = ui
                                // TODO: theoretical
                                .label(format!("{:.p$}", dag1223.value().normalized))
                                // .label(format!("{:.p$}", dag1223.theoretical.normalized))
                                .on_hover_ui(|ui| {
                                    if !dag1223.is_experimental() {
                                        ui.colored_label(
                                            Color32::YELLOW,
                                            "⚠ Warning: it's a theoretical value",
                                        );
                                        ui.label(dag1223.theoretical.normalized.to_string());
                                    } else {
                                        if context.settings.calculation.theoretical {
                                            ui.heading("Experimental:");
                                        }
                                        ui.label(dag1223.experimental.normalized.to_string());
                                        if context.settings.calculation.unnormalized {
                                            let mut unnormalized =
                                                dag1223.experimental.unnormalized;
                                            if context.settings.calculation.pchelkin {
                                                unnormalized *= 10.0;
                                            }
                                            ui.label(format!("Unnormalized: {unnormalized}"));
                                        }
                                    }
                                });
                            if context.settings.calculation.theoretical {
                                response.on_hover_ui(|ui| {
                                    ui.heading("Theoretical:");
                                    ui.label(dag1223.theoretical.normalized.to_string());
                                    if context.settings.calculation.unnormalized {
                                        let mut unnormalized = dag1223.theoretical.unnormalized;
                                        if context.settings.calculation.pchelkin {
                                            unnormalized *= 10.0;
                                        }
                                        ui.label(format!("Unnormalized: {unnormalized}"));
                                    }
                                    if context.settings.calculation.selectivity {
                                        let selectivity = dag1223.theoretical.normalized
                                            / tag123.experimental.unnormalized;
                                        ui.label(format!("Selectivity: {selectivity}"));
                                    }
                                });
                            }
                        });
                        // 2-MAGs
                        row.right_align_col(|ui| {
                            if context.settings.calculation.percent {
                                mag2.experimental.normalized *= 100.0;
                                mag2.theoretical.normalized *= 100.0;
                            }
                            let response = ui
                                // TODO: theoretical
                                .label(format!("{:.p$}", mag2.value().normalized))
                                // .label(format!("{:.p$}", mag2.theoretical.normalized))
                                .on_hover_ui(|ui| {
                                    if !mag2.is_experimental() {
                                        ui.colored_label(
                                            Color32::YELLOW,
                                            "⚠ Warning: it's a theoretical value",
                                        );
                                        ui.label(mag2.theoretical.normalized.to_string());
                                    } else {
                                        if context.settings.calculation.theoretical {
                                            ui.heading("Experimental:");
                                        }
                                        ui.label(mag2.experimental.normalized.to_string());
                                        if context.settings.calculation.unnormalized {
                                            let mut unnormalized = mag2.experimental.unnormalized;
                                            if context.settings.calculation.pchelkin {
                                                unnormalized *= 10.0;
                                            }
                                            ui.label(format!("Unnormalized: {unnormalized}"));
                                        }
                                    }
                                });
                            if context.settings.calculation.theoretical {
                                response.on_hover_ui(|ui| {
                                    ui.heading("Theoretical:");
                                    ui.label(mag2.theoretical.normalized.to_string());
                                    if context.settings.calculation.unnormalized {
                                        let mut unnormalized = mag2.theoretical.unnormalized;
                                        if context.settings.calculation.pchelkin {
                                            unnormalized *= 10.0;
                                        }
                                        ui.label(format!("Unnormalized: {unnormalized}"));
                                    }
                                    if context.settings.calculation.selectivity {
                                        let selectivity = mag2.theoretical.normalized
                                            / tag123.experimental.unnormalized;
                                        ui.label(format!("Selectivity: {selectivity}"));
                                    }
                                });
                            }
                        });
                        // 1,3-DAGs
                        row.right_align_col(|ui| {
                            let Normalizable {
                                mut unnormalized,
                                mut normalized,
                            } = *dag13.value(context.settings.calculation.from);
                            if context.settings.calculation.percent {
                                normalized *= 100.0;
                            }
                            ui.label(format!("{normalized:.p$}")).on_hover_ui(|ui| {
                                ui.label(normalized.to_string());
                                if context.settings.calculation.unnormalized {
                                    if context.settings.calculation.pchelkin {
                                        unnormalized *= 10.0;
                                    }
                                    ui.label(format!("Unnormalized: {unnormalized}"));
                                }
                                if context.settings.calculation.selectivity {
                                    let selectivity = normalized / tag123.experimental.unnormalized;
                                    ui.label(format!("Selectivity: {selectivity}"));
                                }
                                if context.settings.calculation.selectivity_factor {
                                    let selectivity_factor =
                                        normalized / tag123.experimental.normalized;
                                    ui.label(format!("Selectivity factor: {selectivity_factor}"));
                                }
                                if context.settings.calculation.enrichment_factor {
                                    let selectivity_factor =
                                        normalized / tag123.experimental.normalized;
                                    let mut u123 = 0.0;
                                    let mut u2 = 0.0;
                                    let (u123, u2) = context.unsaturated().fold(
                                        (0.0, 0.0),
                                        |(u123, u2), index| {
                                            (
                                                u123 + context
                                                    .state
                                                    .entry()
                                                    .data
                                                    .calculated
                                                    .tags123
                                                    .experimental
                                                    .normalized[index],
                                                u2 + context
                                                    .state
                                                    .entry()
                                                    .data
                                                    .calculated
                                                    .tags123
                                                    .experimental
                                                    .normalized[index],
                                            )
                                        },
                                    );
                                    // ui.label(format!("Enrichment factor: {enrichment_factor}"));
                                }
                            });
                        });
                    });
                }
                // Footer
                let calculated = &context.state.entry().data.calculated;
                body.separate(height / 2.0, 5);
                body.row(height, |mut row| {
                    row.col(|_| {});
                    // 1,2,3-TAGs
                    row.right_align_col(|ui| {
                        let mut sum: f64 = calculated.tags123.experimental.normalized.iter().sum();
                        if context.settings.calculation.percent {
                            sum *= 100.0;
                        }
                        ui.label(format!("{sum:.p$}")).on_hover_ui(|ui| {
                            ui.label(sum.to_string());
                        });
                    });
                    // 1,2/2,3-DAGs
                    row.right_align_col(|ui| {
                        let mut sum: f64 = calculated.dags1223.experimental.normalized.iter().sum();
                        if context.settings.calculation.percent {
                            sum *= 100.0;
                        }
                        ui.label(format!("{sum:.p$}")).on_hover_ui(|ui| {
                            ui.label(sum.to_string());
                        });
                    });
                    // 2-MAGs
                    row.right_align_col(|ui| {
                        let mut sum: f64 = calculated.mags2.experimental.normalized.iter().sum();
                        if context.settings.calculation.percent {
                            sum *= 100.0;
                        }
                        ui.label(format!("{sum:.p$}")).on_hover_ui(|ui| {
                            ui.label(sum.to_string());
                        });
                    });
                    // 1,3-DAGs
                    row.right_align_col(|ui| {
                        let mut sum: f64 = calculated
                            .dags13
                            .value(context.settings.calculation.from)
                            .normalized
                            .iter()
                            .sum();
                        if context.settings.calculation.percent {
                            sum *= 100.0;
                        }
                        ui.label(format!("{sum:.p$}")).on_hover_ui(|ui| {
                            ui.label(sum.to_string());
                        });
                    });
                });
            });
    }
}
