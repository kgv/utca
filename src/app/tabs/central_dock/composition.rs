use crate::{
    acylglycerol::Tag,
    app::{
        computers::composer::{Composed, Key},
        context::Context,
        settings::{Order, Positional, Sort},
        MAX_PRECISION,
    },
    utils::egui::Separate,
};
use egui::{ComboBox, Direction, Id, Layout, RichText, Slider, Ui};
use egui_extras::{Column, TableBuilder};
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashSet, default::default, mem::take};

macro filter_combobox($ui:ident, $context:ident, $state:ident, $id:ident) {
    let id_source = stringify!($id).trim_start_matches("sn");
    let selected_text = id_source.chars().join(", ");
    let mut changed = false;
    ComboBox::from_id_source(id_source)
        .selected_text(selected_text)
        .show_ui($ui, |ui| {
            for (index, label) in $context.labels.iter().enumerate() {
                let mut checked = !$state.filter.$id.contains(&index);
                if ui.checkbox(&mut checked, label).changed() {
                    changed |= true;
                    if !checked {
                        $state.filter.$id.insert(index);
                    } else {
                        $state.filter.$id.remove(&index);
                    }
                }
            }
        });
    if changed {
        let popup_id = $ui.make_persistent_id(Id::new(id_source)).with("popup");
        $ui.memory_mut(|memory| memory.open_popup(popup_id));
    }
}

const COLUMNS: usize = 2;

/// Composition tab
pub(super) struct Composition<'a> {
    ui: &'a mut Ui,
    context: &'a mut Context,
    composed: IndexMap<Tag<usize>, f64>,
    state: State,
}

impl<'a> Composition<'a> {
    pub(super) fn view(ui: &'a mut Ui, context: &'a mut Context) {
        let state = State::load(ui);
        let composed = ui.memory_mut(|memory| {
            memory.caches.cache::<Composed>().get(Key {
                labels: &context.labels,
                dags13: &context.normalized.dags13,
                mags2: &context.normalized.mags2,
                composition: state.composition,
                sort: state.sort,
            })
        });
        context.composed = composed
            .iter()
            .filter(|(tag, &part)| {
                !state.filter.sn13.contains(&tag[0])
                    && !state.filter.sn2.contains(&tag[1])
                    && !state.filter.sn13.contains(&tag[2])
                    && part >= state.filter.part
            })
            .map(|(tag, part)| (*tag, *part))
            .collect();
        Self {
            ui,
            context,
            composed,
            state,
        }
        .ui()
    }
}

impl Composition<'_> {
    fn ui(&mut self) {
        self.control();
        self.content();
    }

    fn control(&mut self) {
        let Self {
            ui, context, state, ..
        } = self;
        ui.collapsing(RichText::new("Control").heading(), |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(&mut state.resizable, "↔ Resizable")
                    .on_hover_text("Resize table columns")
            });
            ui.collapsing(RichText::new("🛠 Control").heading(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    ui.add(Slider::new(&mut state.precision, 0..=MAX_PRECISION));
                });
                ui.horizontal(|ui| {
                    ui.label("Percent:");
                    ui.checkbox(&mut state.percent, "");
                });
                ui.horizontal(|ui| {
                    ui.label("Composition:");
                    let selected_text = match state.composition {
                        Some(positional) => Cow::Owned(positional.to_string()),
                        None => Cow::Borrowed("None"),
                    };
                    ComboBox::from_id_source("composition")
                        .width(ui.available_width())
                        .selected_text(selected_text)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut state.composition, None, "None");
                            ui.selectable_value(
                                &mut state.composition,
                                Some(Positional::Species),
                                "PSC",
                            )
                            .on_hover_text("Positional-species composition");
                            ui.selectable_value(
                                &mut state.composition,
                                Some(Positional::Type),
                                "PTC",
                            )
                            .on_hover_text("Positional-type composition");
                        });
                });
            });
            ui.collapsing(RichText::new("🔎 Filter").heading(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Key:");
                    filter_combobox!(ui, context, state, sn13);
                    filter_combobox!(ui, context, state, sn2);
                });
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ui.add(Slider::new(&mut state.filter.part, 0.0..=1.0).logarithmic(true));
                });
            });
            ui.collapsing(RichText::new("🔤 Sort").heading(), |ui| {
                let selected_text = match state.sort {
                    Sort::Key(order) => (format!("{order:?}"), default()),
                    Sort::Value(order) => (default(), format!("{order:?}")),
                };
                ui.horizontal(|ui| {
                    ui.label("Key:");
                    ComboBox::from_id_source("key")
                        .width(ui.available_width())
                        .selected_text(selected_text.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut state.sort,
                                Sort::Key(Order::Ascending),
                                "⬊ Ascending",
                            );
                            ui.selectable_value(
                                &mut state.sort,
                                Sort::Key(Order::Descending),
                                "⬈ Descending",
                            );
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ComboBox::from_id_source("value")
                        .width(ui.available_width())
                        .selected_text(selected_text.1)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut state.sort,
                                Sort::Value(Order::Ascending),
                                "⬊ Ascending",
                            );
                            ui.selectable_value(
                                &mut state.sort,
                                Sort::Value(Order::Descending),
                                "⬈ Descending",
                            );
                        });
                });
                // let mut keep = false;
                // ComboBox::from_id_source("sort")
                //     .width(ui.available_width())
                //     .selected_text(self.sort.to_string())
                //     .show_ui(ui, |ui| {
                //         keep |= ui
                //             .collapsing("By key", |ui| {
                //                 ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                //                     ui.selectable_value(
                //                         &mut self.sort,
                //                         Sort::Key(Order::Ascending),
                //                         "⬊ Ascending",
                //                     );
                //                     ui.selectable_value(
                //                         &mut self.sort,
                //                         Sort::Key(Order::Descending),
                //                         "⬈ Descending",
                //                     );
                //                 });
                //             })
                //             .header_response
                //             .changed();
                //         keep |= ui
                //             .collapsing("By value", |ui| {
                //                 ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                //                     ui.selectable_value(
                //                         &mut self.sort,
                //                         Sort::Value(Order::Ascending),
                //                         "⬊ Ascending",
                //                     );
                //                     ui.selectable_value(
                //                         &mut self.sort,
                //                         Sort::Value(Order::Descending),
                //                         "⬈ Descending",
                //                     );
                //                 });
                //             })
                //             .header_response
                //             .changed();
                //     });
                // if keep {
                //     let popup_id = ui.make_persistent_id(Id::new("sort")).with("popup");
                //     ui.memory_mut(|memory| memory.open_popup(popup_id));
                // }
            });
        });
    }

    fn content(&mut self) {
        let Self {
            ui,
            context,
            composed,
            state,
        } = self;
        let height = ui.spacing().interact_size.y;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), COLUMNS)
            .auto_shrink([false; 2])
            .resizable(state.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.heading("TAG");
                });
                row.col(|ui| {
                    ui.heading("Part");
                });
            })
            .body(|mut body| {
                for (tag, &(mut part)) in &context.composed {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            let tag = tag.map(|index| &context.labels[index]);
                            ui.label(format!("{tag}"));
                        });
                        row.col(|ui| {
                            if state.percent {
                                part *= 100.0;
                            }
                            ui.label(format!("{part:.*}", state.precision))
                                .on_hover_text(part.to_string());
                        });
                    });
                }
                // Footer
                body.separate(height / 2.0, COLUMNS);
                body.row(height, |mut row| {
                    row.col(|ui| {
                        let count = composed.len();
                        let filtered = context.composed.len();
                        let unfiltered = count - filtered;
                        ui.label(filtered.to_string())
                            .on_hover_text(format!("{count} - {unfiltered} = {filtered}"));
                    });
                    row.col(|ui| {
                        let mut sum: f64 = composed.values().sum();
                        let mut filtered: f64 = context.composed.values().sum();
                        if state.percent {
                            sum *= 100.0;
                            filtered *= 100.0;
                        }
                        let unfiltered = sum - filtered;
                        ui.label(format!("{filtered:.*}", state.precision))
                            .on_hover_text(format!(
                                "{sum:.0$} - {unfiltered:.0$} = {filtered:.0$}",
                                state.precision
                            ));
                    });
                });
            });
    }
}

impl Drop for Composition<'_> {
    fn drop(&mut self) {
        take(&mut self.state).save(self.ui);
    }
}

/// State
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct State {
    composition: Option<Positional>,
    filter: Filter,
    percent: bool,
    precision: usize,
    resizable: bool,
    sort: Sort,
}

impl State {
    fn load(ui: &Ui) -> Self {
        ui.data_mut(|data| {
            data.get_persisted(ui.id().with("state"))
                .unwrap_or_default()
        })
    }

    fn save(self, ui: &Ui) {
        let id = ui.id().with("state");
        ui.data_mut(|data| {
            if Some(&self) != data.get_persisted(id).as_ref() {
                data.insert_persisted(id, self);
            }
        });
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            composition: None,
            filter: default(),
            percent: false,
            precision: 5,
            resizable: false,
            sort: Sort::Key(Order::Ascending),
        }
    }
}

/// Filter
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Filter {
    part: f64,
    sn13: HashSet<usize>,
    sn2: HashSet<usize>,
}
