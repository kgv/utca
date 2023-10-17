use crate::app::{computers::composer::Composed, context::Context};
use egui::{Align, Direction, Layout, Ui};
use egui_ext::{ClickedLabel, TableBodyExt};
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;
use std::ops::Range;

const COLUMNS: usize = 5;

/// Central composition tab
pub(super) struct Composition;

impl Composition {
    pub(super) fn view(ui: &mut Ui, context: &mut Context) {
        context.state.data.composed =
            ui.memory_mut(|memory| memory.caches.cache::<Composed>().get(context));
        let height = ui.spacing().interact_size.y;
        let ptc = context.settings.composition.is_positional_type();
        let psc = context.settings.composition.is_positional_species();
        let mut count = COLUMNS;
        if !ptc || !psc {
            count -= 1;
        }
        if !context.settings.composition.ecn {
            count -= 1;
        }
        if !context.settings.composition.mass {
            count -= 1;
        }
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), count)
            .auto_shrink([false; 2])
            .resizable(context.settings.composition.resizable)
            .striped(true)
            .header(height, |mut row| {
                if ptc {
                    row.col(|ui| {
                        ui.clicked_heading("Type")
                            .context_menu(|ui| {
                                if ui.button("Copy types").clicked() {
                                    ui.output_mut(|output| {
                                        output.copied_text = context
                                            .state
                                            .data
                                            .composed
                                            .filtered
                                            .keys()
                                            .map(|&tag| context.r#type(tag))
                                            .unique()
                                            .join("\n");
                                    });
                                    ui.close_menu();
                                }
                            })
                            .on_hover_text("TAG's type");
                    });
                }
                if psc {
                    row.col(|ui| {
                        ui.clicked_heading("Species")
                            .context_menu(|ui| {
                                if ui.button("Copy species").clicked() {
                                    ui.output_mut(|output| {
                                        output.copied_text = context
                                            .state
                                            .data
                                            .composed
                                            .filtered
                                            .keys()
                                            .map(|&tag| context.species(tag))
                                            .join("\n");
                                    });
                                    ui.close_menu();
                                }
                            })
                            .on_hover_text("TAG's species");
                    });
                }
                row.col(|ui| {
                    ui.clicked_heading("Value").context_menu(|ui| {
                        if ptc {
                            let text = if psc {
                                "Copy type values"
                            } else {
                                "Copy values"
                            };
                            if ui.button(text).clicked() {
                                ui.output_mut(|output| {
                                    output.copied_text = context
                                        .state
                                        .data
                                        .composed
                                        .filtered
                                        .iter()
                                        .group_by(|(&tag, _)| context.r#type(tag))
                                        .into_iter()
                                        .map(|(_, group)| group.map(|(_, &value)| value).sum())
                                        .format_with("\n", |mut value: f64, f| {
                                            if context.settings.composition.percent {
                                                value *= 100.0;
                                            }
                                            f(&format_args!(
                                                "{value:.*}",
                                                context.settings.composition.precision
                                            ))
                                        })
                                        .to_string();
                                });
                                ui.close_menu();
                            };
                        }
                        if psc {
                            let text = if ptc {
                                "Copy species values"
                            } else {
                                "Copy values"
                            };
                            if ui.button(text).clicked() {
                                ui.output_mut(|output| {
                                    output.copied_text = context
                                        .state
                                        .data
                                        .composed
                                        .filtered
                                        .values()
                                        .format_with("\n", |&(mut value), f| {
                                            if context.settings.composition.percent {
                                                value *= 100.0;
                                            }
                                            f(&format_args!(
                                                "{value:.*}",
                                                context.settings.composition.precision
                                            ))
                                        })
                                        .to_string();
                                });
                                ui.close_menu();
                            }
                        }
                    });
                });
                if context.settings.composition.ecn {
                    row.col(|ui| {
                        ui.heading("ECN").on_hover_text("Equivalent carbon number");
                    });
                }
                if context.settings.composition.mass {
                    row.col(|ui| {
                        ui.heading("Mass").on_hover_text("Triacylglycerol mass");
                    });
                }
            })
            .body(|mut body| {
                let mut index = 0;
                for (&tag, &(mut value)) in &context.state.data.composed.filtered {
                    let r#type = &context.r#type(tag);
                    if ptc {
                        let Range { start, end } = context.state.data.composed.grouped[r#type];
                        if index == start {
                            index = end;
                            body.row(height, |mut row| {
                                row.col(|ui| {
                                    ui.label(r#type.to_string());
                                });
                                if psc {
                                    row.col(|ui| {
                                        let count = context.state.data.composed.filtered
                                            [start..end]
                                            .keys()
                                            .count();
                                        ui.label(count.to_string());
                                    });
                                }
                                row.col(|ui| {
                                    ui.with_layout(
                                        Layout::left_to_right(Align::Center)
                                            .with_main_align(Align::RIGHT)
                                            .with_main_justify(true),
                                        |ui| {
                                            let mut value: f64 =
                                                context.state.data.composed.filtered[start..end]
                                                    .values()
                                                    .sum();
                                            if context.settings.composition.percent {
                                                value *= 100.0;
                                            }
                                            ui.label(format!(
                                                "{value:.*}",
                                                context.settings.composition.precision
                                            ))
                                            .on_hover_text(value.to_string());
                                        },
                                    );
                                });
                                if context.settings.composition.ecn {
                                    row.col(|ui| {
                                        if let Some((min, max)) =
                                            context.state.data.composed.filtered[start..end]
                                                .keys()
                                                .map(|&tag| context.ecn(tag).sum())
                                                .minmax()
                                                .into_option()
                                        {
                                            ui.label(format!("[{min}, {max}]"));
                                        }
                                    });
                                }
                                if context.settings.composition.mass {
                                    row.col(|ui| {
                                        if let Some((min, max)) =
                                            context.state.data.composed.filtered[start..end]
                                                .keys()
                                                .map(|&tag| context.weight(tag).sum())
                                                .minmax()
                                                .into_option()
                                        {
                                            ui.label(format!(
                                                "[{min:.0$}, {max:.0$}]",
                                                context.settings.composition.precision
                                            ));
                                        }
                                    });
                                }
                            });
                        }
                    }
                    if psc {
                        body.row(height, |mut row| {
                            if ptc {
                                row.col(|_| {});
                            }
                            row.col(|ui| {
                                ui.label(context.species(tag).to_string())
                                    .on_hover_text(r#type.to_string());
                            });
                            row.col(|ui| {
                                ui.with_layout(
                                    Layout::left_to_right(Align::Center)
                                        .with_main_align(Align::RIGHT)
                                        .with_main_justify(true),
                                    |ui| {
                                        if context.settings.composition.percent {
                                            value *= 100.0;
                                        }
                                        ui.label(format!(
                                            "{value:.*}",
                                            context.settings.composition.precision
                                        ))
                                        .on_hover_text(value.to_string());
                                    },
                                );
                            });
                            if context.settings.composition.ecn {
                                row.col(|ui| {
                                    ui.with_layout(
                                        Layout::left_to_right(Align::Center)
                                            .with_main_align(Align::Center)
                                            .with_main_justify(true),
                                        |ui| {
                                            let ecn = context.ecn(tag);
                                            ui.label(ecn.sum().to_string())
                                                .on_hover_text(format!("{ecn:#}"));
                                        },
                                    );
                                });
                            }
                            if context.settings.composition.mass {
                                row.col(|ui| {
                                    ui.with_layout(
                                        Layout::left_to_right(Align::Center)
                                            .with_main_align(Align::Center)
                                            .with_main_justify(true),
                                        |ui| {
                                            let weight = context.weight(tag);
                                            ui.label(format!(
                                                "{:.*}",
                                                context.settings.composition.precision,
                                                weight.sum(),
                                            ))
                                            .on_hover_text(format!(
                                                "{weight:#.*}",
                                                context.settings.composition.precision
                                            ));
                                        },
                                    );
                                });
                            }
                        });
                    }
                }
                // Footer
                body.separate(height / 2.0, count);
                body.row(height, |mut row| {
                    if ptc {
                        row.col(|ui| {
                            let unfiltered = context.state.data.composed.grouped.len();
                            let filtered = context
                                .state
                                .data
                                .composed
                                .grouped
                                .values()
                                .filter(|&range| !range.is_empty())
                                .count();
                            let count = unfiltered - filtered;
                            ui.label(filtered.to_string())
                                .on_hover_text(format!("{unfiltered} - {count} = {filtered}"));
                        });
                    }
                    if psc {
                        row.col(|ui| {
                            let unfiltered = context.state.data.composed.unfiltered.len();
                            let filtered = context.state.data.composed.filtered.len();
                            let count = unfiltered - filtered;
                            ui.label(filtered.to_string())
                                .on_hover_text(format!("{unfiltered} - {count} = {filtered}"));
                        });
                    }
                    row.col(|ui| {
                        ui.with_layout(
                            Layout::left_to_right(Align::Center)
                                .with_main_align(Align::RIGHT)
                                .with_main_justify(true),
                            |ui| {
                                let mut unfiltered: f64 =
                                    context.state.data.composed.unfiltered.values().sum();
                                let mut filtered: f64 =
                                    context.state.data.composed.filtered.values().sum();

                                if context.settings.composition.percent {
                                    unfiltered *= 100.0;
                                    filtered *= 100.0;
                                }
                                let sum = unfiltered - filtered;
                                ui.label(format!(
                                    "{filtered:.*}",
                                    context.settings.composition.precision
                                ))
                                .on_hover_text(format!(
                                    "{unfiltered:.0$} - {sum:.0$} = {filtered:.0$}",
                                    context.settings.composition.precision
                                ));
                            },
                        );
                    });
                    if context.settings.composition.ecn {
                        row.col(|ui| {
                            if let Some((min, max)) = context
                                .state
                                .data
                                .composed
                                .filtered
                                .keys()
                                .map(|&tag| context.ecn(tag).sum())
                                .minmax()
                                .into_option()
                            {
                                ui.label(format!("[{min}, {max}]"));
                            }
                        });
                    }
                    if context.settings.composition.mass {
                        row.col(|ui| {
                            if let Some((min, max)) = context
                                .state
                                .data
                                .composed
                                .filtered
                                .keys()
                                .map(|&tag| context.weight(tag).sum())
                                .minmax()
                                .into_option()
                            {
                                ui.label(format!(
                                    "[{min:.0$}, {max:.0$}]",
                                    context.settings.composition.precision
                                ));
                            }
                        });
                    }
                });
                body.separate(height / 2.0, count);
            });
    }
}
