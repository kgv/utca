use crate::{app::MAX_PRECISION, localization::localize};
use egui::{ComboBox, Grid, Key, KeyboardShortcut, Modifiers, RichText, Slider, Ui};
use serde::{Deserialize, Serialize};

/// Calculation settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
    pub(in crate::app) fraction: Fraction,
    pub(in crate::app) from: From,
    pub(in crate::app) signedness: Sign,
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
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui) {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(localize!("calculation")).heading(), |ui| {
            Grid::new("calculation").show(ui, |ui| {
                ui.label(localize!("precision"));
                ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
                ui.end_row();

                ui.label(localize!("percent"));
                ui.checkbox(&mut self.percent, "");
                ui.end_row();

                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label(localize!("fraction"));
                let fraction = &mut self.fraction;
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
                ui.end_row();

                ui.label(localize!("sign"));
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
                ui.end_row();

                ui.label(localize!("from"))
                    .on_hover_text(localize!("from.description"));
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
                ui.end_row();
            });
        });
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
    pub(in crate::app) fn text(self) -> String {
        match self {
            Self::AsIs => localize!("as_is"),
            Self::ToMole => localize!("to_mole_fraction"),
            Self::ToMass => localize!("to_mass_fraction"),
            Self::Pchelkin => "Pchelkin".to_owned(),
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
pub(in crate::app) enum From {
    Dag1223,
    Mag2,
}

impl From {
    pub(in crate::app) fn text(self) -> String {
        match self {
            Self::Dag1223 => localize!("from_dag"),
            Self::Mag2 => localize!("from_mag"),
        }
    }

    pub(in crate::app) fn hover_text(self) -> String {
        match self {
            Self::Dag1223 => localize!("from_dag.description"),
            Self::Mag2 => localize!("from_mag.description"),
        }
    }
}

/// Sign
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Sign {
    Signed,
    #[default]
    Unsigned,
}

impl Sign {
    pub(in crate::app) fn text(self) -> String {
        match self {
            Self::Signed => localize!("signed"),
            Self::Unsigned => localize!("unsigned"),
        }
    }

    pub(in crate::app) fn hover_text(self) -> String {
        match self {
            Self::Signed => localize!("signed.description"),
            Self::Unsigned => localize!("unsigned.description"),
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
