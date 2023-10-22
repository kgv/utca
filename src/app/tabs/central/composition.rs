use crate::app::{
    context::{settings::composition::Group, Context},
    view::View,
};
use egui::{Align, Direction, InnerResponse, Layout, Ui};
use egui_ext::{ClickedLabel, CollapsingButton, TableBodyExt};
use egui_extras::{Column, TableBuilder};
use indexmap::IndexMap;
use itertools::Itertools;
use std::convert::identity;

const COLUMNS: usize = 4;

/// Central composition tab
pub(super) struct Composition<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Composition<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Composition<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        context.compose(ui);
        let height = ui.spacing().interact_size.y;
        let mut columns = COLUMNS;
        if !context.settings.composition.ecn {
            columns -= 1;
        }
        if !context.settings.composition.mass {
            columns -= 1;
        }
        let mut open = None;
        TableBuilder::new(ui)
            .auto_shrink([false; 2])
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), columns)
            .max_scroll_height(f32::NAN)
            .resizable(context.settings.composition.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.clicked_heading("TAG")
                        .context_menu(|ui| {
                            if context.settings.composition.group.is_some() {
                                if ui
                                    .button("Expand")
                                    .on_hover_text("Expand all groups")
                                    .clicked()
                                {
                                    open = Some(true);
                                    ui.close_menu();
                                }
                                if ui
                                    .button("Collapse")
                                    .on_hover_text("Collapse all groups")
                                    .clicked()
                                {
                                    open = Some(false);
                                    ui.close_menu();
                                }
                                ui.separator();
                            }
                            ui.menu_button("Copy", |ui| {
                                if ui.button("All").clicked() {
                                    ui.close_menu();
                                }
                                if ui.button("Groups").clicked() {
                                    ui.output_mut(|output| {
                                        output.copied_text = context
                                            .state
                                            .entry()
                                            .data
                                            .composed
                                            .filtered
                                            .keys()
                                            .copied()
                                            .filter_map(identity)
                                            .join("\n");
                                    });
                                    ui.close_menu();
                                }
                                if ui.button("Items").clicked() {
                                    ui.output_mut(|output| {
                                        output.copied_text = context
                                            .state
                                            .entry()
                                            .data
                                            .composed
                                            .filtered
                                            .values()
                                            .flat_map(|values| {
                                                values.keys().map(|&tag| context.species(tag))
                                            })
                                            .join("\n");
                                    });
                                    ui.close_menu();
                                }
                            });
                        })
                        .on_hover_text("Triacylglycerol");
                });
                row.col(|ui| {
                    ui.clicked_heading("Value").context_menu(|ui| {
                        ui.menu_button("Copy", |ui| {
                            if ui.button("Values").clicked() {
                                ui.close_menu();
                            }
                            if ui.button("Group values").clicked() {
                                ui.output_mut(|output| {
                                    output.copied_text = context
                                        .state
                                        .entry()
                                        .data
                                        .composed
                                        .filtered
                                        .values()
                                        .map(|values| values.values().sum())
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
                            if ui.button("Species values").clicked() {
                                ui.output_mut(|output| {
                                    output.copied_text = context
                                        .state
                                        .entry()
                                        .data
                                        .composed
                                        .filtered
                                        .values()
                                        .flat_map(IndexMap::values)
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
                        });
                    });
                });
                if context.settings.composition.ecn {
                    row.col(|ui| {
                        ui.clicked_heading("ECN")
                            .context_menu(|ui| {
                                //         if ui.button("Copy ECN list").clicked() {
                                //             ui.output_mut(|output| {
                                //                 output.copied_text = context
                                //                     .state
                                //                     .entry()
                                //                     .data
                                //                     .composed
                                //                     .filtered
                                //                     .keys()
                                //                     .map(|&tag| context.ecn(tag).sum())
                                //                     .join("\n");
                                //             });
                                //             ui.close_menu();
                                //         }
                            })
                            .on_hover_text("Equivalent carbon number");
                    });
                }
                if context.settings.composition.mass {
                    row.col(|ui| {
                        ui.heading("Mass");
                    });
                }
            })
            .body(|mut body| {
                for (group, values) in &context.state.entry().data.composed.filtered {
                    let mut close = false;
                    if let Some(group) = group {
                        body.row(height, |mut row| {
                            row.col(|ui| {
                                let InnerResponse { inner, response } = CollapsingButton::new()
                                    .text(group.to_string())
                                    .open(open)
                                    .show(ui);
                                response.on_hover_text(values.len().to_string());
                                close = !inner;
                            });
                            row.col(|ui| {
                                let mut sum: f64 = values.values().sum();
                                if context.settings.composition.percent {
                                    sum *= 100.0;
                                }
                                ui.label(format!(
                                    "{sum:.*}",
                                    context.settings.composition.precision,
                                ))
                                .on_hover_text(sum.to_string());
                            });
                            if context.settings.composition.ecn {
                                row.col(|ui| {
                                    if let Some((min, max)) = values
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
                                    if let Some((min, max)) = values
                                        .keys()
                                        .map(|&tag| context.mass(tag).sum())
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
                    if !close {
                        for (&tag, &(mut value)) in values {
                            body.row(height, |mut row| {
                                row.col(|ui| {
                                    let species = context.species(tag);
                                    let response = ui.label(species.to_string());
                                    if let Some(group) = context.settings.composition.group {
                                        response.on_hover_text(match group {
                                            Group::Ecn => format!("{:#}", context.ecn(tag)),
                                            Group::Ptc => context.r#type(tag).to_string(),
                                        });
                                    }
                                });
                                row.col(|ui| {
                                    if context.settings.composition.percent {
                                        value *= 100.0;
                                    }
                                    ui.label(format!(
                                        "{value:.*}",
                                        context.settings.composition.precision
                                    ))
                                    .on_hover_text(value.to_string());
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
                                                let mass = context.mass(tag);
                                                ui.label(format!(
                                                    "{:.*}",
                                                    context.settings.composition.precision,
                                                    mass.sum(),
                                                ))
                                                .on_hover_text(format!(
                                                    "{mass:#.*}",
                                                    context.settings.composition.precision
                                                ));
                                            },
                                        );
                                    });
                                }
                            });
                        }
                    }
                }
                // Footer
                body.separate(height / 2.0, columns);
                body.row(height, |mut row| {
                    row.col(|ui| {
                        let composed = &context.state.entry().data.composed;
                        let unfiltered = composed
                            .unfiltered
                            .values()
                            .flat_map(IndexMap::values)
                            .count();
                        let filtered = composed
                            .filtered
                            .values()
                            .flat_map(IndexMap::values)
                            .count();
                        let count = unfiltered - filtered;
                        ui.label(filtered.to_string()).on_hover_ui(|ui| {
                            if context.settings.composition.group.is_some() {
                                let unfiltered = composed.unfiltered.len();
                                let filtered = composed.filtered.len();
                                let count = unfiltered - filtered;
                                ui.label(format!("{unfiltered} - {count} = {filtered}"));
                            }
                            ui.label(format!("{unfiltered} - {count} = {filtered}"));
                        });
                    });
                    row.col(|ui| {
                        let mut unfiltered: f64 = context
                            .state
                            .entry()
                            .data
                            .composed
                            .unfiltered
                            .values()
                            .flat_map(IndexMap::values)
                            .sum();
                        let mut filtered: f64 = context
                            .state
                            .entry()
                            .data
                            .composed
                            .filtered
                            .values()
                            .flat_map(IndexMap::values)
                            .sum();
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
                    });
                    if context.settings.composition.ecn {
                        row.col(|ui| {
                            if let Some((min, max)) = context
                                .state
                                .entry()
                                .data
                                .composed
                                .filtered
                                .values()
                                .flat_map(|values| values.keys().map(|&tag| context.ecn(tag).sum()))
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
                                .entry()
                                .data
                                .composed
                                .filtered
                                .values()
                                .flat_map(|values| {
                                    values.keys().map(|&tag| context.mass(tag).sum())
                                })
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
                body.separate(height / 2.0, columns);
            });
    }
}
