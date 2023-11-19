use crate::{
    app::{
        context::{state::composition::Value, Context},
        view::View,
    },
    tree::{Hierarchized, Hierarchy, Item},
};
use egui::{Direction, Id, InnerResponse, Label, Layout, RichText, Ui};
use egui_ext::{ClickedLabel, CollapsingButton};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use std::iter::{once, zip};
use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style},
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
        let mut open = None;
        TableBuilder::new(ui)
            .auto_shrink([false; 2])
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), context.state.entries.len() + 1)
            .max_scroll_height(f32::NAN)
            .resizable(context.settings.comparison.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.clicked_heading("TAG")
                        .context_menu(|ui| {
                            if !context.settings.comparison.groups.is_empty() {
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
                            if ui.button("Copy").clicked() {
                                let mut builder = Builder::default();
                                let header = once("TAG").chain(
                                    context.state.entries.iter().map(|entry| &*entry.meta.name),
                                );
                                builder.set_header(header);
                                for Hierarchized(Hierarchy { level, index }, item) in
                                    context.state.compared.hierarchy()
                                {
                                    match item {
                                        Item::Meta(meta) => {
                                            let row = meta
                                                .group
                                                .iter()
                                                .map(|group| group.to_string())
                                                .chain(meta.values.iter().map(|&value| {
                                                    value
                                                        .map(|Value { mut rounded, .. }| {
                                                            if context.settings.comparison.percent {
                                                                rounded *= 100.0;
                                                            }
                                                            format!("{rounded:.p$}")
                                                        })
                                                        .unwrap_or_default()
                                                }));
                                            builder.push_record(row);
                                        }
                                        Item::Data(data) => {
                                            let row = once(context.species(data.tag).to_string())
                                                .chain(data.values.iter().map(|value| {
                                                    value
                                                        .map(|mut value| {
                                                            if context.settings.comparison.percent {
                                                                value *= 100.0;
                                                            }
                                                            format!("{value:.p$}")
                                                        })
                                                        .unwrap_or_default()
                                                }));
                                            builder.push_record(row);
                                        }
                                    }
                                }
                                let table = builder
                                    .build()
                                    .with(Style::markdown())
                                    .with(Modify::new(Rows::first()).with(Alignment::center()))
                                    .to_string();
                                ui.output_mut(|output| {
                                    output.copied_text = table;
                                });
                                ui.close_menu();
                            }
                        })
                        .on_hover_text("Triacylglycerol");
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
                let mut close = false;
                let mut path = vec![];
                for Hierarchized(Hierarchy { level, index }, item) in
                    context.state.compared.hierarchy()
                {
                    match item {
                        Item::Meta(meta) => {
                            while path.len() > level {
                                path.pop();
                            }
                            if let Some(group) = meta.group {
                                path.push(group.to_string());
                            }
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
                                                let count = meta.count;
                                                response.on_hover_text(format!(
                                                    "Count: {count}\n{path:?}"
                                                ));
                                                close = !inner;
                                            });
                                        });
                                });
                                for (count, value) in zip(&meta.counts, &meta.values) {
                                    row.col(|ui| {
                                        if let Some(value) = value {
                                            let mut rounded = value.rounded;
                                            let mut unrounded = value.unrounded;
                                            if context.settings.comparison.percent {
                                                rounded *= 100.0;
                                                unrounded *= 100.0;
                                            }
                                            ui.label(format!("{rounded:.p$}")).on_hover_ui(|ui| {
                                                ui.label(format!("Unrounded: {unrounded}"));
                                                ui.label(format!("Count: {count}"));
                                            });
                                        } else {
                                            ui.label("-");
                                        }
                                    });
                                }
                            });
                        }
                        Item::Data(data) => {
                            if close {
                                continue;
                            }
                            body.row(height, |mut row| {
                                let species = context.species(data.tag);
                                row.col(|ui| {
                                    ui.label(species.to_string()).on_hover_ui(|ui| {
                                        ui.label(format!(
                                            "CMN: {:01$b}",
                                            context.cmn(data.tag),
                                            context.state.entries.len(),
                                        ));
                                        ui.label(format!("PTC: {}", context.ptc(data.tag)));
                                        let ecn = context.ecn(data.tag);
                                        ui.label(format!("ECN: {ecn:#} ({})", ecn.sum()));
                                        let mass = context.mass(data.tag);
                                        ui.label(format!("Mass: {mass:#.p$} ({:.p$})", mass.sum()));
                                    });
                                });
                                for (index, &value) in data.values.iter().enumerate() {
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
                                        // if let Some(mut value) = value.or_else(|| {
                                        //     context.state.entries[index]
                                        //         .data
                                        //         .composed
                                        //         .unfiltered(&data.tag)
                                        // }) {
                                        //     response.on_hover_text({
                                        //         if context.settings.comparison.percent {
                                        //             value *= 100.0;
                                        //         }
                                        //         value.to_string()
                                        //     });
                                        // }
                                    });
                                }
                            });
                        }
                    }
                }
            });
    }
}
