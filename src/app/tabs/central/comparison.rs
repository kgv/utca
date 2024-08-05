use crate::{
    app::{
        context::{
            state::{
                comparison::Meta,
                composition::{Count, Value},
            },
            Context,
        },
        view::View,
    },
    tree::{Hierarchized, Hierarchy, Item},
    utils::ui::UiExt,
};
use egui::{Direction, Id, InnerResponse, Label, Layout, RichText, Ui};
use egui_ext::{ClickedLabel, CollapsingButton, TableBodyExt, TableRowExt};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::iter::{once, zip};
use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Panel, Style},
    Table,
};

/// Central comparison tab
pub(super) struct Comparison<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Comparison<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Comparison<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        let p = context.settings.comparison.precision;
        context.compare(ui);
        let height = ui.spacing().interact_size.y;
        TableBuilder::new(ui)
            .auto_shrink(false)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), context.state.entries.len() + 1)
            .max_scroll_height(f32::NAN)
            .resizable(context.settings.comparison.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.clicked_heading("TAG").on_hover_text("Triacylglycerol");
                });
                for entry in &context.state.entries {
                    row.col(|ui| {
                        ui.add(
                            Label::new(RichText::new(&entry.meta.name).heading()).truncate(true),
                        );
                    });
                }
            })
            .body(|mut body| {
                for (tag, values) in &context.state.compared.data {
                    body.row(height, |mut row| {
                        row.left_align_col(|ui| {
                            let text = ui.subscripted_text(
                                &format!("{tag:#}"),
                                context.settings.composition.tree.leafs.text(),
                                Default::default(),
                            );
                            ui.label(text)
                                .on_hover_ui(|ui| {
                                    // if let Some(tag) = context.indices(tag) {
                                    // let cmn = context.cmn(indices);
                                    // ui.label(format!(
                                    //     "CMN: {cmn:00$b}",
                                    //     context.state.entries.len(),
                                    // ));
                                    // }
                                    // ui.label(format!("PTC: {}", context.r#type(tag)));
                                    // let ecn = context.ecn(data.tag);
                                    // ui.label(format!("ECN: {ecn:#} ({})", ecn.sum()));
                                    // let mass = context.mass(data.tag);
                                    // ui.label(format!("Mass: {mass:#.p$} ({:.p$})", mass.sum()));
                                })
                                .context_menu(|ui| {
                                    let contains = context.settings.comparison.set.contains(tag);
                                    if contains && ui.button("Remove from compare").clicked() {
                                        context.settings.comparison.set.shift_remove(tag);
                                        ui.close_menu();
                                    }
                                    if contains && ui.button("Clear compare").clicked() {
                                        context.settings.comparison.set.clear();
                                        ui.close_menu();
                                    }
                                });
                        });
                        for value in values {
                            row.col(|ui| {
                                if let Some(mut value) = value {
                                    if context.settings.comparison.percent {
                                        value *= 100.0;
                                    }
                                    ui.label(format!("{value:.p$}"))
                                        .on_hover_text(format!("Unrounded: {value}"));
                                } else {
                                    ui.label("-");
                                }
                            });
                        }
                    });
                }
                // Footer
                body.separate(height / 2.0, context.state.entries.len() + 1);
                body.row(height, |mut row| {
                    row.col(|_| {});
                    for meta in &context.state.compared.meta {
                        row.col(|ui| {
                            if let Some(mut value) = meta.sum {
                                if context.settings.comparison.percent {
                                    value *= 100.0;
                                }
                                let Count {
                                    filtered,
                                    unfiltered,
                                } = meta.count;
                                ui.label(format!("{value:.p$}")).on_hover_ui(|ui| {
                                    ui.label(format!("Unrounded: {value}"));
                                    ui.label(format!("Filtered count: {filtered}"));
                                    ui.label(format!("Unfiltered count: {unfiltered}"));
                                });
                            } else {
                                ui.label("-");
                            }
                        });
                    }
                });
                body.separate(height / 2.0, context.state.entries.len() + 1);
            });
    }
}
