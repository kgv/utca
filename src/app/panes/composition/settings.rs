use crate::{
    acylglycerol::Stereospecificity,
    app::MAX_PRECISION,
    localization::{
        ADDUCT, ASCENDING, ASCENDING_DESCRIPTION, COMPOSITION, DESCENDING, DESCENDING_DESCRIPTION,
        GROUP, GUNSTONE, GUNSTONE_DESCRIPTION, KEY, KEY_DESCRIPTION, METHOD, ORDER, PERCENT,
        PRECISION, SORT, VALUE, VALUE_DESCRIPTION, VANDER_WAL, VANDER_WAL_DESCRIPTION,
    },
    r#const::relative_atomic_mass::{H, LI, NA, NH4},
};
use egui::{ComboBox, DragValue, Key, KeyboardShortcut, Modifiers, RichText, Slider, Ui};
use egui_tiles::UiResponse;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

/// Calculation settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) percent: bool,
    pub(crate) precision: usize,

    pub(crate) adduct: OrderedFloat<f64>,
    pub(crate) method: Method,
    pub(crate) group: Composition,
    pub(crate) sort: Sort,
    pub(crate) order: Order,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            percent: true,
            precision: 1,
            adduct: OrderedFloat(0.0),
            method: Method::VanderWal,
            group: Composition::default(),
            sort: Sort::Value,
            order: Order::Descending,
        }
    }
}

impl Settings {
    pub(crate) fn ui(&mut self, ui: &mut Ui) -> UiResponse {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(&COMPOSITION).heading(), |ui| {
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
                let adduct = &mut self.adduct;
                ui.label(&ADDUCT);
                ui.add(
                    DragValue::new(&mut adduct.0)
                        .range(0.0..=f64::MAX)
                        .speed(1.0 / 10f64.powi(self.precision as _)),
                )
                .on_hover_text(format!("{adduct}"));
                ComboBox::from_id_source("")
                    .selected_text(match adduct.0 {
                        adduct if adduct == H => "H",
                        adduct if adduct == NH4 => "NH4",
                        adduct if adduct == NA => "Na",
                        adduct if adduct == LI => "Li",
                        _ => "",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut adduct.0, H, "H");
                        ui.selectable_value(&mut adduct.0, NH4, "NH4");
                        ui.selectable_value(&mut adduct.0, NA, "Na");
                        ui.selectable_value(&mut adduct.0, LI, "Li");
                    });
            });
            ui.separator();
            // Method
            ui.horizontal(|ui| {
                if ui.input_mut(|input| {
                    input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::G))
                }) {
                    self.method = Method::Gunstone;
                }
                if ui.input_mut(|input| {
                    input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::W))
                }) {
                    self.method = Method::VanderWal;
                }
                ui.label(&METHOD);
                ComboBox::from_id_source("method")
                    .selected_text(self.method.text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.method,
                            Method::Gunstone,
                            Method::Gunstone.text(),
                        )
                        .on_hover_text(Method::Gunstone.hover_text());
                        ui.selectable_value(
                            &mut self.method,
                            Method::VanderWal,
                            Method::VanderWal.text(),
                        )
                        .on_hover_text(Method::VanderWal.hover_text());
                    })
                    .response
                    .on_hover_text(self.method.hover_text());
            });
            // Group
            ui.horizontal(|ui| {
                ui.label(&GROUP);
                ComboBox::from_id_source("group")
                    .selected_text(self.group.text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.group, NC, NC.text())
                            .on_hover_text(NC.hover_text());
                        ui.selectable_value(&mut self.group, PNC, PNC.text())
                            .on_hover_text(PNC.hover_text());
                        ui.selectable_value(&mut self.group, SNC, SNC.text())
                            .on_hover_text(SNC.hover_text());
                        ui.selectable_value(&mut self.group, MC, MC.text())
                            .on_hover_text(MC.hover_text());
                        ui.selectable_value(&mut self.group, PMC, PMC.text())
                            .on_hover_text(PMC.hover_text());
                        ui.selectable_value(&mut self.group, SMC, SMC.text())
                            .on_hover_text(SMC.hover_text());
                        ui.selectable_value(&mut self.group, SC, SC.text())
                            .on_hover_text(SC.hover_text());
                        ui.selectable_value(&mut self.group, PSC, PSC.text())
                            .on_hover_text(PSC.hover_text());
                        ui.selectable_value(&mut self.group, SSC, SSC.text())
                            .on_hover_text(SSC.hover_text());
                        ui.selectable_value(&mut self.group, TC, TC.text())
                            .on_hover_text(TC.hover_text());
                        ui.selectable_value(&mut self.group, PTC, PTC.text())
                            .on_hover_text(PTC.hover_text());
                        ui.selectable_value(&mut self.group, STC, STC.text())
                            .on_hover_text(STC.hover_text());
                    })
                    .response
                    .on_hover_text(self.group.hover_text());
            });
            // ui.menu_button(&GROUP, |ui| {
            //     let mut response = ui
            //         .selectable_value(&mut context.settings.composition.tree.leafs, SC, SC.text())
            //         .on_hover_text(SC.hover_text());
            //     response |= ui
            //         .selectable_value(
            //             &mut context.settings.composition.tree.leafs,
            //             PSC,
            //             PSC.text(),
            //         )
            //         .on_hover_text(PSC.hover_text());
            //     response |= ui
            //         .selectable_value(
            //             &mut context.settings.composition.tree.leafs,
            //             SSC,
            //             SSC.text(),
            //         )
            //         .on_hover_text(SSC.hover_text());
            // });

            // Sort
            ui.horizontal(|ui| {
                ui.label(&SORT);
                ComboBox::from_id_source("sort")
                    .selected_text(self.sort.text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.sort, Sort::Key, Sort::Key.text())
                            .on_hover_text(Sort::Key.hover_text());
                        ui.selectable_value(&mut self.sort, Sort::Value, Sort::Value.text())
                            .on_hover_text(Sort::Value.hover_text());
                    })
                    .response
                    .on_hover_text(self.sort.hover_text());
            });
            ui.horizontal(|ui| {
                ui.label(&ORDER);
                ComboBox::from_id_source("order")
                    .selected_text(self.order.text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.order,
                            Order::Ascending,
                            Order::Ascending.text(),
                        )
                        .on_hover_text(Order::Ascending.hover_text());
                        ui.selectable_value(
                            &mut self.order,
                            Order::Descending,
                            Order::Descending.text(),
                        )
                        .on_hover_text(Order::Descending.hover_text());
                    })
                    .response
                    .on_hover_text(self.order.hover_text());
            });
        });
        UiResponse::None
    }
}

/// Method
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Method {
    Gunstone,
    VanderWal,
}

impl Method {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::Gunstone => &GUNSTONE,
            Self::VanderWal => &VANDER_WAL,
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::Gunstone => &GUNSTONE_DESCRIPTION,
            Self::VanderWal => &VANDER_WAL_DESCRIPTION,
        }
    }
}

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Sort {
    Key,
    Value,
}

impl Sort {
    pub(crate) fn text(self) -> &'static str {
        match self {
            Self::Key => &KEY,
            Self::Value => &VALUE,
        }
    }

    pub(crate) fn hover_text(self) -> &'static str {
        match self {
            Self::Key => &KEY_DESCRIPTION,
            Self::Value => &VALUE_DESCRIPTION,
        }
    }
}

/// Order
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Order {
    Ascending,
    Descending,
}

impl Order {
    pub(crate) fn text(self) -> &'static str {
        match self {
            Self::Ascending => &ASCENDING,
            Self::Descending => &DESCENDING,
        }
    }

    pub(crate) fn hover_text(self) -> &'static str {
        match self {
            Self::Ascending => &ASCENDING_DESCRIPTION,
            Self::Descending => &DESCENDING_DESCRIPTION,
        }
    }
}

// pub(crate) static BRANCHES: LazyLock<IndexMap<Scope, Vec<Composition>>> = LazyLock::new(|| {
//     indexmap! {
//         Scope::EquivalentCarbonNumber => vec![NC, PNC, SNC],
//         Scope::Mass => vec![MC, PMC, SMC],
//         Scope::Type => vec![TC, PTC, STC],
//         Scope::Species => vec![SC, PSC],
//     }
// });

pub(crate) const NC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::EquivalentCarbonNumber,
};
pub(crate) const PNC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::EquivalentCarbonNumber,
};
pub(crate) const SNC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::EquivalentCarbonNumber,
};
pub(crate) const MC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Mass,
};
pub(crate) const PMC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Mass,
};
pub(crate) const SMC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Mass,
};
pub(crate) const SC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Species,
};
pub(crate) const PSC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Species,
};
pub(crate) const SSC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Species,
};
pub(crate) const TC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Type,
};
pub(crate) const PTC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Type,
};
pub(crate) const STC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Type,
};

/// Composition
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) struct Composition {
    pub(crate) scope: Scope,
    pub(crate) stereospecificity: Option<Stereospecificity>,
}

impl Composition {
    pub(crate) fn text(&self) -> &'static str {
        match *self {
            NC => "NC",
            PNC => "PNC",
            SNC => "SNC",

            MC => "MC",
            PMC => "PMC",
            SMC => "SMC",

            TC => "TC",
            PTC => "PTC",
            STC => "STC",

            SC => "SC",
            PSC => "PSC",
            SSC => "SSC",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match *self {
            NC => "Equivalent carbon number composition",
            PNC => "Positional equivalent carbon number composition",
            SNC => "Stereo equivalent carbon number composition",

            MC => "Mass composition",
            PMC => "Positional mass composition",
            SMC => "Stereo mass composition",

            TC => "Type composition",
            PTC => "Positional type composition",
            STC => "Stereo type composition",

            SC => "Species composition",
            PSC => "Positional species composition",
            SSC => "Stereo species composition",
        }
    }
}

impl Default for Composition {
    fn default() -> Self {
        Self {
            stereospecificity: Some(Stereospecificity::Positional),
            scope: Scope::Species,
        }
    }
}

/// Scope
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum Scope {
    EquivalentCarbonNumber,
    Mass,
    Type,
    Species,
}

impl Scope {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::EquivalentCarbonNumber => "Equivalent carbon number",
            Self::Mass => "Mass",
            Self::Species => "Species",
            Self::Type => "Type",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::EquivalentCarbonNumber => "ECN",
            Self::Mass => "M",
            Self::Species => "S",
            Self::Type => "T",
        }
    }
}
