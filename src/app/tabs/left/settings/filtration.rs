use crate::{
    acylglycerol::Sn,
    app::{
        context::{settings::composition::Filter, Context},
        view::View,
    },
};
use egui::{ComboBox, Id, RichText, Slider, Ui};

/// Left filtration tab
pub(super) struct Filtration<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Filtration<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Filtration<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        ui.collapsing(RichText::new("🔎 Filtration").heading(), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().combo_width = 0.75 * ui.spacing().combo_width;
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
                let response = ui.checkbox(&mut context.settings.composition.mirror, "Mirror");
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
