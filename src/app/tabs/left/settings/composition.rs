use crate::{
    acylglycerol::Sn,
    app::{
        context::{
            settings::composition::{Filter, Order, Positional, Sort},
            Context,
        },
        view::View,
        MAX_PRECISION,
    },
};
use egui::{ComboBox, Id, RichText, Slider, Ui};

// macro filter_combobox($ui:ident, $context:expr, $id:ident) {{
//     let id_source = stringify!($id).trim_start_matches("sn");
//     let selected_text = id_source.chars().join(", ");
//     let mut changed = false;
//     ComboBox::from_id_source(id_source)
//         .selected_text(selected_text)
//         .show_ui($ui, |ui| {
//             for (index, label) in $context.state.meta.labels.iter().enumerate() {
//                 let mut checked = !$context.settings.composition.filter.$id.contains(&index);
//                 if ui.checkbox(&mut checked, label).changed() {
//                     changed |= true;
//                     if !checked {
//                         $context.settings.composition.filter.$id.insert(index);
//                     } else {
//                         $context.settings.composition.filter.$id.remove(&index);
//                     }
//                 }
//             }
//         })
//         .response
//         .context_menu(|ui| {
//             if ui.button("Check all").clicked() {
//                 $context.settings.composition.filter.$id.clear();
//                 ui.close_menu();
//             } else if ui.button("Uncheck all").clicked() {
//                 $context.settings.composition.filter.$id =
//                     (0..$context.state.meta.labels.len()).collect();
//                 ui.close_menu();
//             }
//         });
//     if changed {
//         let popup_id = $ui.make_persistent_id(Id::new(id_source)).with("popup");
//         $ui.memory_mut(|memory| memory.open_popup(popup_id));
//     }
// }}

/// Filter combobox
trait FilterCombobox {
    fn filter_combobox(&mut self, context: &mut Context, sn: Sn);
}

impl FilterCombobox for Ui {
    fn filter_combobox(&mut self, context: &mut Context, sn: Sn) {
        let Filter { sn1, sn2, sn3, .. } = &mut context.settings.composition.filter;
        let mut changed = false;
        ComboBox::from_id_source(sn)
            .selected_text(sn.to_string())
            .show_ui(self, |ui| {
                for (index, label) in context.state.meta.labels.iter().enumerate() {
                    let mut checked = match sn {
                        Sn::One => !sn1.contains(&index),
                        Sn::Two => !sn2.contains(&index),
                        Sn::Three => !sn3.contains(&index),
                    };
                    if ui.checkbox(&mut checked, label).changed() {
                        changed |= true;
                        if !checked {
                            match sn {
                                Sn::One | Sn::Three if !context.settings.composition.mirror => {
                                    sn1.insert(index);
                                    sn3.insert(index);
                                }
                                Sn::One => {
                                    sn1.insert(index);
                                }
                                Sn::Two => {
                                    sn2.insert(index);
                                }
                                Sn::Three => {
                                    sn3.insert(index);
                                }
                            }
                        } else {
                            match sn {
                                Sn::One | Sn::Three if !context.settings.composition.mirror => {
                                    sn1.remove(&index);
                                    sn3.remove(&index);
                                }
                                Sn::One => {
                                    sn1.remove(&index);
                                }
                                Sn::Two => {
                                    sn2.remove(&index);
                                }
                                Sn::Three => {
                                    sn3.remove(&index);
                                }
                            }
                        }
                    }
                }
            })
            .response
            .context_menu(|ui| {
                if ui.button("Check all").clicked() {
                    match sn {
                        Sn::One | Sn::Three if !context.settings.composition.mirror => {
                            sn1.clear();
                            sn3.clear();
                        }
                        Sn::One => {
                            sn1.clear();
                        }
                        Sn::Two => {
                            sn2.clear();
                        }
                        Sn::Three => {
                            sn3.clear();
                        }
                    }
                    ui.close_menu();
                } else if ui.button("Uncheck all").clicked() {
                    let all = (0..context.state.meta.labels.len()).collect();
                    match sn {
                        Sn::One | Sn::Three if !context.settings.composition.mirror => {
                            *sn1 = all;
                            *sn3 = sn1.clone();
                        }
                        Sn::One => {
                            *sn1 = all;
                        }
                        Sn::Two => {
                            *sn2 = all;
                        }
                        Sn::Three => {
                            *sn3 = all;
                        }
                    }
                    ui.close_menu();
                }
            });
        if changed {
            let popup_id = self.make_persistent_id(Id::new(sn)).with("popup");
            self.memory_mut(|memory| memory.open_popup(popup_id));
        }
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

impl View for Composition<'_> {
    fn view(self, ui: &mut Ui) {
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
                ui.label("Columns:");
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
                if ui
                    .checkbox(&mut self.context.settings.composition.mirror, "Mirror")
                    .changed()
                    && !self.context.settings.composition.mirror
                {
                    self.context.settings.composition.filter.sn1.clear();
                    self.context.settings.composition.filter.sn3.clear();
                }
                ui.toggle_value(&mut self.context.settings.composition.ecn, "ECN")
                    .on_hover_text("ECN (equivalent carbon number)");
                ui.toggle_value(&mut self.context.settings.composition.mass, "Mass");
            });
            ui.collapsing(RichText::new("🔎 Filter").heading(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("SN:").on_hover_text("Stereochemical number");
                    ui.filter_combobox(self.context, Sn::One);
                    ui.filter_combobox(self.context, Sn::Two);
                    ui.filter_combobox(self.context, Sn::Three);
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
                                Sort::Tag,
                                "Tag",
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
