use crate::{
    acylglycerol::Stereospecificity,
    app::MAX_PRECISION,
    localization::localize,
    r#const::relative_atomic_mass::{H, LI, NA, NH4},
};
use egui::{ComboBox, DragValue, Grid, Key, KeyboardShortcut, Modifiers, RichText, Slider, Ui};
use egui_phosphor::regular::{MINUS, PLUS};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

pub(in crate::app) const NC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Ecn,
};
pub(in crate::app) const PNC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Ecn,
};
pub(in crate::app) const SNC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Ecn,
};
pub(in crate::app) const MC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Mass,
};
pub(in crate::app) const PMC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Mass,
};
pub(in crate::app) const SMC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Mass,
};
pub(in crate::app) const SC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Species,
};
pub(in crate::app) const PSC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Species,
};
pub(in crate::app) const SSC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Species,
};
pub(in crate::app) const TC: Composition = Composition {
    stereospecificity: None,
    scope: Scope::Type,
};
pub(in crate::app) const PTC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    scope: Scope::Type,
};
pub(in crate::app) const STC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    scope: Scope::Type,
};

/// Composition settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,

    pub(in crate::app) adduct: OrderedFloat<f64>,
    pub(in crate::app) method: Method,
    pub(in crate::app) compositions: Vec<Composition>,
    pub(in crate::app) sort: Sort,
    pub(in crate::app) order: Order,
}

impl Settings {
    pub(in crate::app) const fn new() -> Self {
        Self {
            percent: true,
            precision: 1,
            adduct: OrderedFloat(0.0),
            method: Method::VanderWal,
            compositions: Vec::new(),
            sort: Sort::Value,
            order: Order::Descending,
        }
    }
}

impl Settings {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui) {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(localize!("composition")).heading(), |ui| {
            Grid::new("composition").show(ui, |ui| {
                ui.label(localize!("precision"));
                ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
                ui.end_row();

                ui.label(localize!("percent"));
                ui.checkbox(&mut self.percent, "");
                ui.end_row();

                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label(localize!("adduct"));
                ui.horizontal(|ui| {
                    let adduct = &mut self.adduct;
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
                ui.end_row();

                ui.separator();
                ui.separator();
                ui.end_row();

                // Method
                ui.label(localize!("method"));
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
                ui.end_row();

                // Compose
                // ui.label(localize!("composition"));
                ui.label(localize!("compose"));
                if ui.button(PLUS).clicked() {
                    self.compositions.push(Composition::new());
                }
                ui.end_row();
                self.compositions.retain_mut(|composition| {
                    // ui.add_space(ui);
                    ComboBox::from_id_source(ui.next_auto_id())
                        .selected_text(composition.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(composition, NC, NC.text())
                                .on_hover_text(NC.hover_text());
                            ui.selectable_value(composition, PNC, PNC.text())
                                .on_hover_text(PNC.hover_text());
                            ui.selectable_value(composition, SNC, SNC.text())
                                .on_hover_text(SNC.hover_text());
                            ui.selectable_value(composition, MC, MC.text())
                                .on_hover_text(MC.hover_text());
                            ui.selectable_value(composition, PMC, PMC.text())
                                .on_hover_text(PMC.hover_text());
                            ui.selectable_value(composition, SMC, SMC.text())
                                .on_hover_text(SMC.hover_text());
                            ui.selectable_value(composition, TC, TC.text())
                                .on_hover_text(TC.hover_text());
                            ui.selectable_value(composition, PTC, PTC.text())
                                .on_hover_text(PTC.hover_text());
                            ui.selectable_value(composition, STC, STC.text())
                                .on_hover_text(STC.hover_text());
                            ui.selectable_value(composition, SC, SC.text())
                                .on_hover_text(SC.hover_text());
                            ui.selectable_value(composition, PSC, PSC.text())
                                .on_hover_text(PSC.hover_text());
                            ui.selectable_value(composition, SSC, SSC.text())
                                .on_hover_text(SSC.hover_text());
                        })
                        .response
                        .on_hover_text(composition.hover_text());
                    let keep = !ui.button(MINUS).clicked();
                    ui.end_row();
                    keep
                });

                // Sort
                ui.label(localize!("sort"));
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
                ui.end_row();

                ui.label(localize!("order"));
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
                ui.end_row();
            });
            // ui.horizontal(|ui| {
            //     ui.label(localize!("precision"));
            //     ui.add(Slider::new(&mut self.precision, 0..=MAX_PRECISION));
            // });
            // ui.horizontal(|ui| {
            //     ui.label(localize!("percent"));
            //     ui.checkbox(&mut self.percent, "");
            // });
            // ui.separator();
            // ui.horizontal(|ui| {
            //     ui.label(localize!("adduct"));
            //     let adduct = &mut self.adduct;
            //     ui.add(
            //         DragValue::new(&mut adduct.0)
            //             .range(0.0..=f64::MAX)
            //             .speed(1.0 / 10f64.powi(self.precision as _)),
            //     )
            //     .on_hover_text(format!("{adduct}"));
            //     ComboBox::from_id_source("")
            //         .selected_text(match adduct.0 {
            //             adduct if adduct == H => "H",
            //             adduct if adduct == NH4 => "NH4",
            //             adduct if adduct == NA => "Na",
            //             adduct if adduct == LI => "Li",
            //             _ => "",
            //         })
            //         .show_ui(ui, |ui| {
            //             ui.selectable_value(&mut adduct.0, H, "H");
            //             ui.selectable_value(&mut adduct.0, NH4, "NH4");
            //             ui.selectable_value(&mut adduct.0, NA, "Na");
            //             ui.selectable_value(&mut adduct.0, LI, "Li");
            //         });
            // });
            // ui.separator();
            // // Method
            // ui.horizontal(|ui| {
            //     if ui.input_mut(|input| {
            //         input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::G))
            //     }) {
            //         self.method = Method::Gunstone;
            //     }
            //     if ui.input_mut(|input| {
            //         input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::W))
            //     }) {
            //         self.method = Method::VanderWal;
            //     }
            //     ui.label(localize!("method"));
            //     ComboBox::from_id_source("method")
            //         .selected_text(self.method.text())
            //         .show_ui(ui, |ui| {
            //             ui.selectable_value(
            //                 &mut self.method,
            //                 Method::Gunstone,
            //                 Method::Gunstone.text(),
            //             )
            //             .on_hover_text(Method::Gunstone.hover_text());
            //             ui.selectable_value(
            //                 &mut self.method,
            //                 Method::VanderWal,
            //                 Method::VanderWal.text(),
            //             )
            //             .on_hover_text(Method::VanderWal.hover_text());
            //         })
            //         .response
            //         .on_hover_text(self.method.hover_text());
            // });
            // // Group
            // ui.vertical(|ui| {
            //     ui.horizontal(|ui| {
            //         ui.label(localize!("group"));
            //         if ui.button(PLUS).clicked() {
            //             self.compositions.push(Composition::new());
            //         }
            //     });
            //     self.compositions.retain_mut(|composition| {
            //         ui.horizontal(|ui| {
            //             ComboBox::from_id_source(ui.next_auto_id())
            //                 .selected_text(composition.text())
            //                 .show_ui(ui, |ui| {
            //                     ui.selectable_value(composition, NC, NC.text())
            //                         .on_hover_text(NC.hover_text());
            //                     ui.selectable_value(composition, PNC, PNC.text())
            //                         .on_hover_text(PNC.hover_text());
            //                     ui.selectable_value(composition, SNC, SNC.text())
            //                         .on_hover_text(SNC.hover_text());
            //                     ui.selectable_value(composition, MC, MC.text())
            //                         .on_hover_text(MC.hover_text());
            //                     ui.selectable_value(composition, PMC, PMC.text())
            //                         .on_hover_text(PMC.hover_text());
            //                     ui.selectable_value(composition, SMC, SMC.text())
            //                         .on_hover_text(SMC.hover_text());
            //                     ui.selectable_value(composition, TC, TC.text())
            //                         .on_hover_text(TC.hover_text());
            //                     ui.selectable_value(composition, PTC, PTC.text())
            //                         .on_hover_text(PTC.hover_text());
            //                     ui.selectable_value(composition, STC, STC.text())
            //                         .on_hover_text(STC.hover_text());
            //                     ui.selectable_value(composition, SC, SC.text())
            //                         .on_hover_text(SC.hover_text());
            //                     ui.selectable_value(composition, PSC, PSC.text())
            //                         .on_hover_text(PSC.hover_text());
            //                     ui.selectable_value(composition, SSC, SSC.text())
            //                         .on_hover_text(SSC.hover_text());
            //                 })
            //                 .response
            //                 .on_hover_text(composition.hover_text());
            //             !ui.button(MINUS).clicked()
            //         })
            //         .inner
            //     });
            //     // for (index, composition) in self.compositions.iter_mut().enumerate() {
            //     //     ComboBox::from_id_source(index)
            //     //         .selected_text(composition.text())
            //     //         .show_ui(ui, |ui| {
            //     //             ui.selectable_value(composition, NC, NC.text())
            //     //                 .on_hover_text(NC.hover_text());
            //     //             ui.selectable_value(composition, PNC, PNC.text())
            //     //                 .on_hover_text(PNC.hover_text());
            //     //             ui.selectable_value(composition, SNC, SNC.text())
            //     //                 .on_hover_text(SNC.hover_text());
            //     //             ui.selectable_value(composition, MC, MC.text())
            //     //                 .on_hover_text(MC.hover_text());
            //     //             ui.selectable_value(composition, PMC, PMC.text())
            //     //                 .on_hover_text(PMC.hover_text());
            //     //             ui.selectable_value(composition, SMC, SMC.text())
            //     //                 .on_hover_text(SMC.hover_text());
            //     //             ui.selectable_value(composition, SC, SC.text())
            //     //                 .on_hover_text(SC.hover_text());
            //     //             ui.selectable_value(composition, PSC, PSC.text())
            //     //                 .on_hover_text(PSC.hover_text());
            //     //             ui.selectable_value(composition, SSC, SSC.text())
            //     //                 .on_hover_text(SSC.hover_text());
            //     //             ui.selectable_value(composition, TC, TC.text())
            //     //                 .on_hover_text(TC.hover_text());
            //     //             ui.selectable_value(composition, PTC, PTC.text())
            //     //                 .on_hover_text(PTC.hover_text());
            //     //             ui.selectable_value(composition, STC, STC.text())
            //     //                 .on_hover_text(STC.hover_text());
            //     //         })
            //     //         .response
            //     //         .on_hover_text(composition.hover_text());
            //     //     if ui.button(MINUS).clicked() {
            //     //         self.compositions.push(Composition::new());
            //     //     }
            //     // }
            // });
            // // Sort
            // ui.horizontal(|ui| {
            //     ui.label(localize!("sort"));
            //     ComboBox::from_id_source("sort")
            //         .selected_text(self.sort.text())
            //         .show_ui(ui, |ui| {
            //             ui.selectable_value(&mut self.sort, Sort::Key, Sort::Key.text())
            //                 .on_hover_text(Sort::Key.hover_text());
            //             ui.selectable_value(&mut self.sort, Sort::Value, Sort::Value.text())
            //                 .on_hover_text(Sort::Value.hover_text());
            //         })
            //         .response
            //         .on_hover_text(self.sort.hover_text());
            // });
            // ui.horizontal(|ui| {
            //     ui.label(localize!("order"));
            //     ComboBox::from_id_source("order")
            //         .selected_text(self.order.text())
            //         .show_ui(ui, |ui| {
            //             ui.selectable_value(
            //                 &mut self.order,
            //                 Order::Ascending,
            //                 Order::Ascending.text(),
            //             )
            //             .on_hover_text(Order::Ascending.hover_text());
            //             ui.selectable_value(
            //                 &mut self.order,
            //                 Order::Descending,
            //                 Order::Descending.text(),
            //             )
            //             .on_hover_text(Order::Descending.hover_text());
            //         })
            //         .response
            //         .on_hover_text(self.order.hover_text());
            // });
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

/// Method
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Method {
    Gunstone,
    VanderWal,
}

impl Method {
    pub(in crate::app) fn text(&self) -> String {
        match self {
            Self::Gunstone => localize!("gunstone"),
            Self::VanderWal => localize!("vander_wal"),
        }
    }

    pub(in crate::app) fn hover_text(&self) -> String {
        match self {
            Self::Gunstone => localize!("gunstone.description"),
            Self::VanderWal => localize!("vander_wal.description"),
        }
    }
}

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Sort {
    Key,
    Value,
}

impl Sort {
    pub(in crate::app) fn text(self) -> String {
        match self {
            Self::Key => localize!("key"),
            Self::Value => localize!("value"),
        }
    }

    pub(in crate::app) fn hover_text(self) -> String {
        match self {
            Self::Key => localize!("key.description"),
            Self::Value => localize!("value.description"),
        }
    }
}

/// Order
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Order {
    Ascending,
    Descending,
}

impl Order {
    pub(in crate::app) fn text(self) -> String {
        match self {
            Self::Ascending => localize!("ascending"),
            Self::Descending => localize!("descending"),
        }
    }

    pub(in crate::app) fn hover_text(self) -> String {
        match self {
            Self::Ascending => localize!("ascending.description"),
            Self::Descending => localize!("descending.description"),
        }
    }
}

/// Group
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Composition {
    pub(in crate::app) scope: Scope,
    pub(in crate::app) stereospecificity: Option<Stereospecificity>,
}

impl Composition {
    pub(in crate::app) const fn new() -> Self {
        Self {
            stereospecificity: Some(Stereospecificity::Positional),
            scope: Scope::Species,
        }
    }
}

impl Composition {
    pub(in crate::app) fn text(&self) -> &'static str {
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

    pub(in crate::app) fn hover_text(&self) -> &'static str {
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
        Self::new()
    }
}

/// Scope
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Scope {
    Ecn,
    Mass,
    Type,
    Species,
}

impl Scope {
    pub(in crate::app) fn text(&self) -> &'static str {
        match self {
            Self::Ecn => "Equivalent carbon number",
            Self::Mass => "Mass",
            Self::Species => "Species",
            Self::Type => "Type",
        }
    }

    pub(in crate::app) fn hover_text(&self) -> &'static str {
        match self {
            Self::Ecn => "ECN",
            Self::Mass => "M",
            Self::Species => "S",
            Self::Type => "T",
        }
    }
}
