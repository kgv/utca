use crate::app::{
    context::{
        settings::calculation::{Fraction, From, Signedness},
        Context,
    },
    tabs::CentralTab,
    view::View,
    MAX_PRECISION,
};
use egui::{ComboBox, Key, KeyboardShortcut, Modifiers, RichText, Slider, Ui};

/// Left calculation tab
pub(super) struct Calculation<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Calculation<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Calculation<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        ui.collapsing(
            RichText::new(CentralTab::Calculation.title()).heading(),
            |ui| {
                ui.horizontal(|ui| {
                    ui.toggle_value(&mut context.settings.calculation.resizable, "↔ Resizable")
                        .on_hover_text("Resize table columns")
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    let precision = &mut context.settings.calculation.precision;
                    if ui.add(Slider::new(precision, 0..=MAX_PRECISION)).changed()
                        && context.settings.link
                    {
                        context.settings.configuration.precision = *precision;
                        context.settings.composition.precision = *precision;
                        context.settings.visualization.precision = *precision;
                        context.settings.comparison.precision = *precision;
                    }
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.horizontal(|ui| {
                    ui.label("Percent:");
                    ui.checkbox(&mut context.settings.calculation.percent, "");
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    let fraction = &mut context.settings.calculation.fraction;
                    ui.label("Fraction:");
                    ComboBox::from_id_source("fraction")
                        .selected_text(fraction.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(fraction, Fraction::Mass, Fraction::Mass.text())
                                .on_hover_text(Fraction::Mass.hover_text());
                            ui.selectable_value(
                                fraction,
                                Fraction::Molar { mixture: false },
                                Fraction::Molar { mixture: false }.text(),
                            )
                            .on_hover_text(Fraction::Molar { mixture: false }.hover_text());
                            ui.selectable_value(
                                fraction,
                                Fraction::Molar { mixture: true },
                                Fraction::Molar { mixture: true }.text(),
                            )
                            .on_hover_text(Fraction::Molar { mixture: true }.hover_text());
                        })
                        .response
                        .on_hover_text(fraction.hover_text());
                });
                ui.horizontal(|ui| {
                    if ui.input_mut(|input| {
                        input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::Num1))
                    }) {
                        context.settings.calculation.from = From::Dag1223;
                    }
                    if ui.input_mut(|input| {
                        input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::Num2))
                    }) {
                        context.settings.calculation.from = From::Mag2;
                    }
                    ui.label("1,3-DAG:");
                    ComboBox::from_id_source("1,3")
                        .selected_text(context.settings.calculation.from.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.calculation.from,
                                From::Dag1223,
                                From::Dag1223.text(),
                            )
                            .on_hover_text(From::Dag1223.hover_text());
                            ui.selectable_value(
                                &mut context.settings.calculation.from,
                                From::Mag2,
                                From::Mag2.text(),
                            )
                            .on_hover_text(From::Mag2.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.calculation.from.hover_text());
                });
                ui.collapsing("Hover", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Unnormalized:");
                        ui.checkbox(&mut context.settings.calculation.unnormalized, "")
                            .on_hover_text("Experimental unnormalized");
                        if context.settings.calculation.unnormalized {
                            ui.checkbox(&mut context.settings.calculation.pchelkin, "Pchelkin")
                                .on_hover_text("Pchelkin");
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Theoretical:");
                        ui.checkbox(&mut context.settings.calculation.theoretical, "")
                            .on_hover_text("Theoretical normalized");
                        if context.settings.calculation.theoretical {
                            ComboBox::from_id_source("signedness")
                                .selected_text(context.settings.calculation.signedness.text())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut context.settings.calculation.signedness,
                                        Signedness::Signed,
                                        Signedness::Signed.text(),
                                    )
                                    .on_hover_text(Signedness::Signed.hover_text());
                                    ui.selectable_value(
                                        &mut context.settings.calculation.signedness,
                                        Signedness::Unsigned,
                                        Signedness::Unsigned.text(),
                                    )
                                    .on_hover_text(Signedness::Unsigned.hover_text());
                                })
                                .response
                                .on_hover_text(
                                    context.settings.calculation.signedness.hover_text(),
                                );
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Selectivity:");
                        ui.checkbox(&mut context.settings.calculation.selectivity, "")
                            .on_hover_text("Selectivity");
                    });
                });
            },
        );
    }
}
