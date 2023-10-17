use crate::app::{
    context::{
        settings::composition::{Order, Positional, Sort},
        Context,
    },
    MAX_PRECISION,
};
use egui::{ComboBox, Id, RichText, Slider, Ui};
use itertools::Itertools;

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
        })
        .response
        .context_menu(|ui| {
            if ui.button("Check all").clicked() {
                $context.settings.composition.filter.$id.clear();
                ui.close_menu();
            } else if ui.button("Uncheck all").clicked() {
                $context.settings.composition.filter.$id =
                    (0..$context.state.meta.labels.len()).collect();
                ui.close_menu();
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
                let mut ptc = self.context.settings.composition.is_positional_type();
                ptc ^= ui
                    .selectable_label(ptc, "PTC")
                    .on_hover_text("Positional-type composition")
                    .clicked();
                let mut psc = self.context.settings.composition.is_positional_species();
                psc ^= ui
                    .selectable_label(psc, "PSC")
                    .on_hover_text("Positional-species composition")
                    .clicked();
                self.context.settings.composition.positional = if ptc && psc {
                    None
                } else if ptc {
                    Some(Positional::Type)
                } else if psc {
                    Some(Positional::Species)
                } else {
                    self.context.settings.composition.positional.map(
                        |positional| match positional {
                            Positional::Type => Positional::Species,
                            Positional::Species => Positional::Type,
                        },
                    )
                };
                ui.checkbox(&mut self.context.settings.composition.mirror, "Mirror");
            });
            ui.horizontal(|ui| {
                ui.label("Columns:");
                ui.checkbox(&mut self.context.settings.composition.ecn, "Ecn")
                    .on_hover_text("ECN (equivalent carbon number) column");
                ui.checkbox(&mut self.context.settings.composition.mass, "Mass")
                    .on_hover_text("Mass column");
            });
            // ui.horizontal(|ui| {
            //     ui.label("Composition:");
            //     let text = match self.context.settings.composition.positional {
            //         None => "PTSC",
            //         Some(Positional::Type) => "PTC",
            //         Some(Positional::Species) => "PSC",
            //     };
            //     ComboBox::from_id_source("composition")
            //         .selected_text(text)
            //         .show_ui(ui, |ui| {
            //             ui.selectable_value(
            //                 &mut self.context.settings.composition.positional,
            //                 None,
            //                 "PTSC",
            //             )
            //             .on_hover_text("Positional-type-species composition");
            //             ui.selectable_value(
            //                 &mut self.context.settings.composition.positional,
            //                 Some(Positional::Type),
            //                 "PTC",
            //             )
            //             .on_hover_text("Positional-type composition");
            //             ui.selectable_value(
            //                 &mut self.context.settings.composition.positional,
            //                 Some(Positional::Species),
            //                 "PSC",
            //             )
            //             .on_hover_text("Positional-species composition");
            //         })
            //         .response
            //         .on_hover_text(format!(
            //             "{:#}",
            //             self.context.settings.calculation.normalization
            //         ));
            //     ui.checkbox(&mut self.context.settings.composition.mirror, "Mirror");
            // });
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
                            &mut self.context.settings.composition.filter.value,
                            0.0..=1.0,
                        )
                        .logarithmic(true)
                        .custom_formatter(|mut value, _| {
                            let mut precision = 7;
                            if self.context.settings.composition.percent {
                                value *= 100.0;
                                precision = 5;
                            }
                            format!("{value:.precision$}")
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
                ui.horizontal(|ui| {
                    ui.label("By:");
                    ComboBox::from_id_source("by")
                        .selected_text(self.context.settings.composition.sort.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.context.settings.composition.sort,
                                Sort::Key,
                                "Key",
                            )
                            .on_hover_text("Sort by type and species");
                            ui.selectable_value(
                                &mut self.context.settings.composition.sort,
                                Sort::Value,
                                "Value",
                            )
                            .on_hover_text("Sort by value");
                        });
                    ui.label("Order:");
                    ComboBox::from_id_source("order")
                        .selected_text(self.context.settings.composition.order.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.context.settings.composition.order,
                                Order::Ascending,
                                "⬊ Ascending",
                            );
                            ui.selectable_value(
                                &mut self.context.settings.composition.order,
                                Order::Descending,
                                "⬈ Descending",
                            );
                        });
                });
            });
        });
    }
}
