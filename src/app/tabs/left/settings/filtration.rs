use crate::{
    acylglycerol::Sn,
    app::{
        context::{
            settings::composition::{Filter, PSC, SSC},
            Context,
        },
        view::View,
    },
    utils::ui::{SubscriptedTextFormat, UiExt as _},
};
use egui::{RichText, ScrollArea, Slider, TextStyle, Ui};

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
                ui.label("Species composition:");
                ui.label(context.settings.composition.tree.leafs.text())
                    .on_hover_text(context.settings.composition.tree.leafs.hover_text());
            });
            ui.horizontal(|ui| {
                // ui.spacing_mut().combo_width = 0.75 * ui.spacing().combo_width;
                ui.horizontal(|ui| {
                    ui.label("Key:");
                    for sn in [Sn::One, Sn::Two, Sn::Three] {
                        ui.filter_menu(context, sn);
                    }
                });
                if ui.button("🗑").on_hover_text("Clear filter").clicked() {
                    context.settings.composition.filter = Default::default();
                    context.settings.composition.filter.symmetrical = false;
                }
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
                ui.label("Symmetrical:");
                ui.checkbox(&mut context.settings.composition.filter.symmetrical, "")
                    .on_hover_text("Show only symmetrical species");
            });
        });
    }
}

/// Extension methods for [`Ui`]
trait UiExt {
    fn filter_menu(&mut self, context: &mut Context, sn: Sn);
}

impl UiExt for Ui {
    fn filter_menu(&mut self, context: &mut Context, sn: Sn) {
        let psc = context.settings.composition.tree.leafs == PSC;
        let Filter { sn1, sn2, sn3, .. } = &mut context.settings.composition.filter;
        let mut changed = false;
        self.menu_button(
            self.subscripted_text(
                "SN",
                sn.text(),
                SubscriptedTextFormat {
                    widget: true,
                    ..Default::default()
                },
            ),
            |ui| {
                ScrollArea::vertical().show(ui, |ui| {
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
                                    Sn::One | Sn::Three if psc => {
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
                                    Sn::One | Sn::Three if psc => {
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
                });
            },
        )
        .response
        .on_hover_text(format!("Stereochemical number {}", sn.text()))
        .context_menu(|ui| {
            if ui.button("Check all").clicked() {
                match sn {
                    Sn::One | Sn::Three if psc => {
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
                    Sn::One | Sn::Three if psc => {
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
    }
}
