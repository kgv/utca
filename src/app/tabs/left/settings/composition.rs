use crate::app::{
    context::{
        settings::composition::{Order, Positional, Sort},
        Context,
    },
    MAX_PRECISION,
};
use egui::{ComboBox, Id, RichText, Slider, Ui};
use itertools::Itertools;
use std::borrow::Cow;

macro filter_combobox($ui:ident, $context:expr, $id:ident) {
    let id_source = stringify!($id).trim_start_matches("sn");
    let selected_text = id_source.chars().join(", ");
    let mut changed = false;
    ComboBox::from_id_source(id_source)
        .selected_text(selected_text)
        .show_ui($ui, |ui| {
            for (index, label) in $context.state.meta.labels.iter().enumerate() {
                let mut checked = !$context.settings.composition.filter.$id.contains(&index);
                if ui.checkbox(&mut checked, label).changed() {
                    changed |= true;
                    if !checked {
                        $context.settings.composition.filter.$id.insert(index);
                    } else {
                        $context.settings.composition.filter.$id.remove(&index);
                    }
                }
            }
        });
    if changed {
        let popup_id = $ui.make_persistent_id(Id::new(id_source)).with("popup");
        $ui.memory_mut(|memory| memory.open_popup(popup_id));
    }
}

/// Left composition tab
pub(super) struct Composition<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Composition<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl Composition<'_> {
    pub(super) fn view(self, ui: &mut Ui) {
        ui.collapsing(RichText::new("⛃ Composition").heading(), |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(
                    &mut self.context.settings.composition.resizable,
                    "↔ Resizable",
                )
                .on_hover_text("Resize table columns")
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Precision:");
                ui.add(Slider::new(
                    &mut self.context.settings.composition.precision,
                    0..=MAX_PRECISION,
                ));
            });
            ui.horizontal(|ui| {
                ui.label("Percent:");
                ui.checkbox(&mut self.context.settings.composition.percent, "");
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Composition:");
                let selected_text = match self.context.settings.composition.composition {
                    Some(positional) => Cow::Owned(format!("{positional:#}")),
                    None => Cow::Borrowed("None"),
                };
                ComboBox::from_id_source("composition")
                    .selected_text(selected_text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.context.settings.composition.composition,
                            None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut self.context.settings.composition.composition,
                            Some(Positional::Species),
                            "PSC",
                        )
                        .on_hover_text("Positional-species composition");
                        ui.selectable_value(
                            &mut self.context.settings.composition.composition,
                            Some(Positional::Type),
                            "PTC",
                        )
                        .on_hover_text("Positional-type composition");
                    });
            });
            ui.collapsing(RichText::new("🔎 Filter").heading(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Key:");
                    filter_combobox!(ui, self.context, sn13);
                    filter_combobox!(ui, self.context, sn2);
                });
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ui.add(
                        Slider::new(
                            &mut self.context.settings.composition.filter.part,
                            0.0..=1.0,
                        )
                        .logarithmic(true)
                        .custom_formatter(|mut value, _| {
                            if self.context.settings.composition.percent {
                                value *= 100.0;
                            }
                            format!("{value:.6}")
                        })
                        .custom_parser(|value| {
                            let mut parsed = value.parse::<f64>().ok()?;
                            if self.context.settings.composition.percent {
                                parsed /= 100.0;
                            }
                            Some(parsed)
                        }),
                    );
                });
            });
            ui.collapsing(RichText::new("🔤 Sort").heading(), |ui| {
                let selected_text = match self.context.settings.composition.sort {
                    Sort::Key(order) => (format!("{order:?}"), Default::default()),
                    Sort::Value(order) => (Default::default(), format!("{order:?}")),
                };
                ui.horizontal(|ui| {
                    ui.label("Key:");
                    ComboBox::from_id_source("key")
                        .selected_text(selected_text.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.context.settings.composition.sort,
                                Sort::Key(Order::Ascending),
                                "⬊ Ascending",
                            );
                            ui.selectable_value(
                                &mut self.context.settings.composition.sort,
                                Sort::Key(Order::Descending),
                                "⬈ Descending",
                            );
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ComboBox::from_id_source("value")
                        .selected_text(selected_text.1)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.context.settings.composition.sort,
                                Sort::Value(Order::Ascending),
                                "⬊ Ascending",
                            );
                            ui.selectable_value(
                                &mut self.context.settings.composition.sort,
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
}
