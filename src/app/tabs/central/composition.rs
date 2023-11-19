use crate::{
    app::{context::Context, view::View},
    tree::{Hierarchized, Hierarchy, Item},
};
use egui::{Direction, Id, InnerResponse, Layout, Ui, WidgetText};
use egui_ext::{ClickedLabel, CollapsingButton, TableBodyExt};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

const COLUMNS: usize = 2;

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
        let p = context.settings.composition.precision;
        context.compose(ui);
        let height = ui.spacing().interact_size.y;
        let mut open = None;
        TableBuilder::new(ui)
            .auto_shrink([false; 2])
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), COLUMNS)
            .max_scroll_height(f32::NAN)
            .resizable(context.settings.composition.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.clicked_heading("TAG")
                        .context_menu(|ui| {
                            if !context.settings.composition.groups.is_empty() {
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
                                    // ui.output_mut(|output| {
                                    //     output.copied_text = context
                                    //         .state
                                    //         .entry()
                                    //         .data
                                    //         .composed
                                    //         .filtered
                                    //         .keys()
                                    //         .copied()
                                    //         .filter_map(identity)
                                    //         .join("\n");
                                    // });
                                    ui.close_menu();
                                }
                                if ui.button("Items").clicked() {
                                    // ui.output_mut(|output| {
                                    //     output.copied_text = context
                                    //         .state
                                    //         .entry()
                                    //         .data
                                    //         .composed
                                    //         .filtered
                                    //         .values()
                                    //         .flat_map(|values| {
                                    //             values.keys().map(|&tag| context.species(tag))
                                    //         })
                                    //         .join("\n");
                                    // });
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
                                // ui.output_mut(|output| {
                                //     output.copied_text = context
                                //         .state
                                //         .entry()
                                //         .data
                                //         .composed
                                //         .filtered
                                //         .values()
                                //         .map(|values| values.values().sum())
                                //         .format_with("\n", |mut value: f64, f| {
                                //             if context.settings.composition.percent {
                                //                 value *= 100.0;
                                //             }
                                //             f(&format_args!("{value:.p$}"))
                                //         })
                                //         .to_string();
                                // });
                                ui.close_menu();
                            };
                            if ui.button("Species values").clicked() {
                                // ui.output_mut(|output| {
                                //     output.copied_text = context
                                //         .state
                                //         .entry()
                                //         .data
                                //         .composed
                                //         .filtered
                                //         .values()
                                //         .flat_map(IndexMap::values)
                                //         .format_with("\n", |&(mut value), f| {
                                //             if context.settings.composition.percent {
                                //                 value *= 100.0;
                                //             }
                                //             f(&format_args!("{value:.p$}"))
                                //         })
                                //         .to_string();
                                // });
                                ui.close_menu();
                            }
                        });
                    });
                });
            })
            .body(|mut body| {
                let mut close = false;
                let mut path = vec![];
                for Hierarchized(Hierarchy { level, index }, item) in
                    context.state.entry().data.composed.hierarchy()
                {
                    match item {
                        Item::Meta(meta) => {
                            while path.len() > level {
                                path.pop();
                            }
                            if let Some(group) = meta.group {
                                path.push(group.to_string());
                            }
                            // if close {
                            //     continue;
                            // }
                            body.row(height, |mut row| {
                                row.col(|ui| {
                                    let indent = ui.spacing().indent;
                                    StripBuilder::new(ui)
                                        .sizes(Size::exact(indent), level)
                                        .size(Size::remainder())
                                        .horizontal(|mut strip| {
                                            for _ in 0..level {
                                                strip.cell(|ui| {
                                                    ui.separator();
                                                });
                                            }
                                            strip.cell(|ui| {
                                                let text = meta
                                                    .group
                                                    .map_or_else(Default::default, |group| {
                                                        group.to_string()
                                                    });
                                                let id = Id::new(&path);
                                                let InnerResponse { inner, response } =
                                                    CollapsingButton::new(text)
                                                        .id_source(id)
                                                        .open(open)
                                                        .show(ui);
                                                let filtered = meta.count.filtered;
                                                let unfiltered = meta.count.unfiltered;
                                                let count = unfiltered - filtered;
                                                response.on_hover_text(format!(
                                                    "Count: {filtered} = {unfiltered} - {count}",
                                                ));
                                                close = !inner;
                                            });
                                        });
                                });
                                row.col(|ui| {
                                    let value = &meta.value;
                                    let mut rounded = value.rounded;
                                    let mut unrounded = value.unrounded;
                                    if context.settings.comparison.percent {
                                        rounded *= 100.0;
                                        unrounded *= 100.0;
                                    }
                                    ui.label(format!("{rounded:.p$}"))
                                        .on_hover_text(format!("Unrounded: {unrounded}"));
                                });
                            });
                        }
                        Item::Data(data) => {
                            if close {
                                continue;
                            }
                            body.row(height, |mut row| {
                                row.col(|ui| {
                                    let species = context.species(data.tag);
                                    ui.label(species.to_string()).on_hover_ui(|ui| {
                                        ui.label(format!("PTC: {}", context.ptc(data.tag)));
                                        let ecn = context.ecn(data.tag);
                                        ui.label(format!("ECN: {ecn:#} ({})", ecn.sum()));
                                        let mass = context.mass(data.tag);
                                        ui.label(format!("Mass: {mass:#.p$} ({:.p$})", mass.sum()));
                                    });
                                });
                                row.col(|ui| {
                                    let mut value = data.value;
                                    if context.settings.composition.percent {
                                        value *= 100.0;
                                    }
                                    ui.label(format!("{value:.p$}"))
                                        .on_hover_text(format!("Unrounded: {value}"));
                                });
                            });
                        }
                    }
                }
                // Footer
                let meta = &context.state.entry().data.composed.meta;
                body.separate(height / 2.0, COLUMNS);
                body.row(height, |mut row| {
                    row.col(|ui| {
                        let filtered = meta.count.filtered;
                        let unfiltered = meta.count.unfiltered;
                        let count = unfiltered - filtered;
                        ui.label(filtered.to_string()).on_hover_ui(|ui| {
                            ui.label(format!("{filtered} = {unfiltered} - {count}"));
                        });
                    });
                    row.col(|ui| {
                        let mut rounded = meta.value.rounded;
                        let mut unrounded = meta.value.unrounded;
                        if context.settings.composition.percent {
                            rounded *= 100.0;
                            unrounded *= 100.0;
                        }
                        ui.label(format!("{rounded:.p$}"))
                            .on_hover_text(unrounded.to_string());
                    });
                });
                body.separate(height / 2.0, COLUMNS);
            });
    }
}
