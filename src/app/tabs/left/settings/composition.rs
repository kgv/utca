use crate::{
    acylglycerol::Sn,
    app::{
        context::{
            settings::{
                composition::{Filter, Group},
                Order, Sort,
            },
            Context,
        },
        tabs::CentralTab,
        view::View,
        MAX_PRECISION,
    },
};
use egui::{ComboBox, Id, RichText, Slider, Ui};

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
        let Self { context } = self;
        ui.collapsing(
            RichText::new(CentralTab::Composition.to_string()).heading(),
            |ui| {
                ui.horizontal(|ui| {
                    ui.toggle_value(&mut context.settings.composition.resizable, "↔ Resizable")
                        .on_hover_text("Resize table columns")
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    let precision = &mut context.settings.composition.precision;
                    if ui.add(Slider::new(precision, 0..=MAX_PRECISION)).changed()
                        && context.settings.link
                    {
                        context.settings.configuration.precision = *precision;
                        context.settings.calculation.precision = *precision;
                        context.settings.visualization.precision = *precision;
                        context.settings.comparison.precision = *precision;
                    }
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.horizontal(|ui| {
                    ui.label("Percent:");
                    ui.checkbox(&mut context.settings.composition.percent, "");
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Columns:");
                    ui.toggle_value(&mut context.settings.composition.ecn, "ECN")
                        .on_hover_text("ECN (equivalent carbon number)");
                    ui.toggle_value(&mut context.settings.composition.mass, "Mass");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Group:");
                    let response = ComboBox::from_id_source("group")
                        .selected_text(context.settings.composition.group.map_or("", Group::text))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.composition.group,
                                None,
                                "None",
                            )
                            .on_hover_text("Don't group");
                            ui.selectable_value(
                                &mut context.settings.composition.group,
                                Some(Group::Ecn),
                                Group::Ecn.text(),
                            )
                            .on_hover_text(Group::Ecn.hover_text());
                            ui.selectable_value(
                                &mut context.settings.composition.group,
                                Some(Group::Ptc),
                                Group::Ptc.text(),
                            )
                            .on_hover_text(Group::Ptc.hover_text());
                        })
                        .response;
                    if let Some(group) = context.settings.composition.group {
                        response.on_hover_text(group.hover_text());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Sort:");
                    ComboBox::from_id_source("sort")
                        .selected_text(context.settings.composition.sort.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.composition.sort,
                                Sort::Key,
                                Sort::Key.text(),
                            )
                            .on_hover_text(Sort::Key.hover_text());
                            ui.selectable_value(
                                &mut context.settings.composition.sort,
                                Sort::Value,
                                Sort::Value.text(),
                            )
                            .on_hover_text(Sort::Value.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.composition.sort.hover_text());
                });
                ui.horizontal(|ui| {
                    ui.label("Order:");
                    ComboBox::from_id_source("order")
                        .selected_text(context.settings.composition.order.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.composition.order,
                                Order::Ascending,
                                Order::Ascending.text(),
                            )
                            .on_hover_text(Order::Ascending.hover_text());
                            ui.selectable_value(
                                &mut context.settings.composition.order,
                                Order::Descending,
                                Order::Descending.text(),
                            )
                            .on_hover_text(Order::Descending.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.composition.order.hover_text());
                });
                ui.separator();
                ui.collapsing(RichText::new("🔎 Filter").heading(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("SN:").on_hover_text("Stereochemical number");
                        ui.filter_combobox(context, Sn::One);
                        ui.filter_combobox(context, Sn::Two);
                        ui.filter_combobox(context, Sn::Three);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Value:");
                        ui.add(
                            Slider::new(&mut context.settings.composition.filter.value, 0.0..=1.0)
                                .logarithmic(true)
                                .custom_formatter(|mut value, _| {
                                    let mut precision = 7;
                                    if context.settings.composition.percent {
                                        value *= 100.0;
                                        precision = 5;
                                    }
                                    format!("{value:.precision$}")
                                })
                                .custom_parser(|value| {
                                    let mut parsed = value.parse::<f64>().ok()?;
                                    if context.settings.composition.percent {
                                        parsed /= 100.0;
                                    }
                                    Some(parsed)
                                }),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Modifications:");
                        let response =
                            ui.checkbox(&mut context.settings.composition.mirror, "Mirror");
                        if response.changed() && context.settings.composition.mirror {
                            context.settings.composition.filter.sn1 = context
                                .settings
                                .composition
                                .filter
                                .sn1
                                .union(&context.settings.composition.filter.sn3)
                                .copied()
                                .collect();
                            context.settings.composition.filter.sn3 =
                                context.settings.composition.filter.sn1.clone();
                        }
                        ui.checkbox(&mut context.settings.composition.symmetrical, "Symmetrical");
                    });
                });
            },
        );
    }
}

/// Filter combobox
trait FilterCombobox {
    fn filter_combobox(&mut self, context: &mut Context, sn: Sn);
}

impl FilterCombobox for Ui {
    fn filter_combobox(&mut self, context: &mut Context, sn: Sn) {
        let Filter { sn1, sn2, sn3, .. } = &mut context.settings.composition.filter;
        let mut changed = false;
        ComboBox::from_id_source(sn)
            .selected_text(sn.text())
            .show_ui(self, |ui| {
                for (index, label) in context.state.entry().meta.labels.iter().enumerate() {
                    let mut checked = match sn {
                        Sn::One => !sn1.contains(&index),
                        Sn::Two => !sn2.contains(&index),
                        Sn::Three => !sn3.contains(&index),
                    };
                    if ui.checkbox(&mut checked, label).changed() {
                        changed |= true;
                        if !checked {
                            match sn {
                                Sn::One | Sn::Three if context.settings.composition.mirror => {
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
                                Sn::One | Sn::Three if context.settings.composition.mirror => {
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
                        Sn::One | Sn::Three if context.settings.composition.mirror => {
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
                    let all = (0..context.state.entry().meta.labels.len()).collect();
                    match sn {
                        Sn::One | Sn::Three if context.settings.composition.mirror => {
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
