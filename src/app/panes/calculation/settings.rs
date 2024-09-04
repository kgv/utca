use crate::{
    app::MAX_PRECISION,
    localization::{
        AS_IS, CALCULATION, FRACTION, PERCENT, PRECISION, SIGN, SIGNED, SIGNED_DESCRIPTION,
        TO_MASS_FRACTION, TO_MOLE_FRACTION, UNSIGNED, UNSIGNED_DESCRIPTION,
    },
};
use egui::{ComboBox, Key, KeyboardShortcut, Modifiers, RichText, Slider, Ui};
use egui_tiles::UiResponse;
use serde::{Deserialize, Serialize};

/// Calculation settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    pub(crate) fraction: Fraction,
    pub(crate) from: From,
    pub(crate) signedness: Sign,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            percent: true,
            precision: 1,
            fraction: Fraction::ToMole,
            from: From::Mag2,
            signedness: Sign::Unsigned,
        }
    }
}

impl Settings {
    pub(crate) fn ui(&mut self, ui: &mut Ui) -> UiResponse {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(&CALCULATION).heading(), |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(&PRECISION);
                ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
            });
            ui.horizontal(|ui| {
                ui.label(&PERCENT);
                ui.checkbox(&mut self.percent, "");
            });
            ui.separator();
            ui.horizontal(|ui| {
                let fraction = &mut self.fraction;
                ui.label(&FRACTION);
                ComboBox::from_id_source("fraction")
                    .selected_text(fraction.text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(fraction, Fraction::AsIs, Fraction::AsIs.text())
                            .on_hover_text(Fraction::AsIs.hover_text());
                        ui.selectable_value(fraction, Fraction::ToMole, Fraction::ToMole.text())
                            .on_hover_text(Fraction::ToMole.hover_text());
                        ui.selectable_value(fraction, Fraction::ToMass, Fraction::ToMass.text())
                            .on_hover_text(Fraction::ToMass.hover_text());
                        ui.selectable_value(
                            fraction,
                            Fraction::Pchelkin,
                            Fraction::Pchelkin.text(),
                        )
                        .on_hover_text(Fraction::Pchelkin.hover_text());
                    })
                    .response
                    .on_hover_text(fraction.hover_text());
            });
            ui.horizontal(|ui| {
                ui.label(&SIGN);
                ComboBox::from_id_source("sign")
                    .selected_text(self.signedness.text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.signedness,
                            Sign::Signed,
                            Sign::Signed.text(),
                        )
                        .on_hover_text(Sign::Signed.hover_text());
                        ui.selectable_value(
                            &mut self.signedness,
                            Sign::Unsigned,
                            Sign::Unsigned.text(),
                        )
                        .on_hover_text(Sign::Unsigned.hover_text());
                    })
                    .response
                    .on_hover_text(self.signedness.hover_text());
            });
            ui.horizontal(|ui| {
                if ui.input_mut(|input| {
                    input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::Num1))
                }) {
                    self.from = From::Dag1223;
                }
                if ui.input_mut(|input| {
                    input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::Num2))
                }) {
                    self.from = From::Mag2;
                }
                ui.label("Calculate 1,3-DAG:");
                ComboBox::from_id_source("1,3")
                    .selected_text(self.from.text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.from, From::Dag1223, From::Dag1223.text())
                            .on_hover_text(From::Dag1223.hover_text());
                        ui.selectable_value(&mut self.from, From::Mag2, From::Mag2.text())
                            .on_hover_text(From::Mag2.hover_text());
                    })
                    .response
                    .on_hover_text(self.from.hover_text());
            });
        });
        UiResponse::None
    }
}

/// Fraction
///
/// [wikipedia.org](https://en.wikipedia.org/wiki/Mole_fraction)
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Fraction {
    AsIs,
    ToMole,
    ToMass,
    Pchelkin,
}

impl Fraction {
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::AsIs => &AS_IS,
            Self::ToMole => &TO_MOLE_FRACTION,
            Self::ToMass => &TO_MASS_FRACTION,
            Self::Pchelkin => "Pchelkin",
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::AsIs => "S / ∑ S",
            Self::ToMole => "S / M / ∑(S / M)",
            Self::ToMass => "S * M / ∑(S * M)",
            Self::Pchelkin => "Pchelkin",
        }
    }
}

/// From
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum From {
    Dag1223,
    Mag2,
}

impl From {
    pub(crate) const fn text(self) -> &'static str {
        match self {
            Self::Dag1223 => "1,2/2,3-DAGs",
            Self::Mag2 => "2-MAGs",
        }
    }

    pub(crate) const fn hover_text(self) -> &'static str {
        match self {
            Self::Dag1223 => "Calculate 1,3-DAGs from 1,2/2,3-DAGs",
            Self::Mag2 => "Calculate 1,3-DAGs from 2-MAGs",
        }
    }
}

/// Sign
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Sign {
    Signed,
    #[default]
    Unsigned,
}

impl Sign {
    pub(crate) fn text(self) -> &'static str {
        match self {
            Self::Signed => &SIGNED,
            Self::Unsigned => &UNSIGNED,
        }
    }

    pub(crate) fn hover_text(self) -> &'static str {
        match self {
            Self::Signed => &SIGNED_DESCRIPTION,
            Self::Unsigned => &UNSIGNED_DESCRIPTION,
        }
    }
}

// /// Column show
// #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
// enum Show {
//     #[default]
//     ExperimentalValue,
//     EnrichmentFactor,
//     SelectivityFactor,
// }
