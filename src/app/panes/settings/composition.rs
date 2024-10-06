use crate::{
    acylglycerol::Stereospecificity,
    app::MAX_PRECISION,
    localization::localize,
    r#const::relative_atomic_mass::{H, LI, NA, NH4},
};
use egui::{
    emath::Float, ComboBox, DragValue, Grid, Key, KeyboardShortcut, Modifiers, RichText, Sides,
    Slider, SliderClamping, Ui,
};
use egui_phosphor::regular::{ARROWS_HORIZONTAL, EYE, EYE_SLASH, FUNNEL, FUNNEL_X, MINUS, PLUS};
use ordered_float::OrderedFloat;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

pub(in crate::app) const UC: Composition = Composition {
    stereospecificity: None,
    kind: Kind::Unsaturation,
};
pub(in crate::app) const PUC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    kind: Kind::Unsaturation,
};
pub(in crate::app) const SUC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    kind: Kind::Unsaturation,
};

pub(in crate::app) const NC: Composition = Composition {
    stereospecificity: None,
    kind: Kind::Ecn,
};
pub(in crate::app) const PNC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    kind: Kind::Ecn,
};
pub(in crate::app) const SNC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    kind: Kind::Ecn,
};
pub(in crate::app) const MC: Composition = Composition {
    stereospecificity: None,
    kind: Kind::Mass,
};
pub(in crate::app) const PMC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    kind: Kind::Mass,
};
pub(in crate::app) const SMC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    kind: Kind::Mass,
};
pub(in crate::app) const SC: Composition = Composition {
    stereospecificity: None,
    kind: Kind::Species,
};
pub(in crate::app) const PSC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    kind: Kind::Species,
};
pub(in crate::app) const SSC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    kind: Kind::Species,
};
pub(in crate::app) const TC: Composition = Composition {
    stereospecificity: None,
    kind: Kind::Type,
};
pub(in crate::app) const PTC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Positional),
    kind: Kind::Type,
};
pub(in crate::app) const STC: Composition = Composition {
    stereospecificity: Some(Stereospecificity::Stereo),
    kind: Kind::Type,
};

/// Composition settings
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Settings {
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: usize,
    // How many columns are sticky
    // Default is 0.
    pub(in crate::app) sticky_columns: usize,

    pub(in crate::app) adduct: OrderedFloat<f64>,
    pub(in crate::app) method: Method,
    pub(in crate::app) groups: Vec<Group>,
    pub(in crate::app) filter: Filter,
    pub(in crate::app) sort: Sort,
    pub(in crate::app) order: Order,
    pub(in crate::app) join: Join,
}

impl Settings {
    pub(in crate::app) const fn new() -> Self {
        Self {
            percent: true,
            precision: 1,
            sticky_columns: 0,
            adduct: OrderedFloat(0.0),
            method: Method::VanderWal,
            groups: Vec::new(),
            filter: Filter::new(),
            sort: Sort::Value,
            order: Order::Descending,
            join: Join::Left,
        }
    }
}

impl Settings {
    pub(in crate::app) fn ui(&mut self, ui: &mut Ui) {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new(localize!("composition")).heading(), |ui| {
            Grid::new("composition").show(ui, |ui| {
                ui.label(localize!("sticky"));
                ui.add(Slider::new(
                    &mut self.sticky_columns,
                    0..=self.groups.len() + 1,
                ));
                ui.end_row();

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
                    ComboBox::from_id_salt("")
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
                ComboBox::from_id_salt("method")
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

                // Filter
                ui.label(localize!("filter"));
                ui.add(
                    Slider::new(&mut self.filter.value, 0.0..=1.0)
                        .clamping(SliderClamping::Always)
                        .logarithmic(true)
                        .custom_formatter(|mut value, _| {
                            if self.percent {
                                value *= 100.0;
                            }
                            AnyValue::Float64(value).to_string()
                        })
                        .custom_parser(|value| {
                            let mut parsed = value.parse::<f64>().ok()?;
                            if self.percent {
                                parsed /= 100.0;
                            }
                            Some(parsed)
                        }),
                );
                ui.end_row();

                // Compose
                ui.label(localize!("compose"));
                if ui.button(PLUS).clicked() {
                    self.groups.push(Group::new());
                }
                ui.end_row();
                self.groups.retain_mut(|group| {
                    let mut keep = true;
                    ui.label("");
                    ui.horizontal(|ui| {
                        // Delete
                        keep = !ui.button(MINUS).clicked();
                        ComboBox::from_id_salt(ui.next_auto_id())
                            .selected_text(group.composition.text())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut group.composition, NC, NC.text())
                                    .on_hover_text(NC.hover_text());
                                ui.selectable_value(&mut group.composition, PNC, PNC.text())
                                    .on_hover_text(PNC.hover_text());
                                ui.selectable_value(&mut group.composition, SNC, SNC.text())
                                    .on_hover_text(SNC.hover_text());
                                ui.separator();
                                ui.selectable_value(&mut group.composition, MC, MC.text())
                                    .on_hover_text(MC.hover_text());
                                ui.selectable_value(&mut group.composition, PMC, PMC.text())
                                    .on_hover_text(PMC.hover_text());
                                ui.selectable_value(&mut group.composition, SMC, SMC.text())
                                    .on_hover_text(SMC.hover_text());
                                ui.separator();
                                ui.selectable_value(&mut group.composition, UC, UC.text())
                                    .on_hover_text(UC.hover_text());
                                ui.selectable_value(&mut group.composition, PUC, PUC.text())
                                    .on_hover_text(PUC.hover_text());
                                ui.selectable_value(&mut group.composition, SUC, SUC.text())
                                    .on_hover_text(SUC.hover_text());
                                ui.separator();
                                ui.selectable_value(&mut group.composition, TC, TC.text())
                                    .on_hover_text(TC.hover_text());
                                ui.selectable_value(&mut group.composition, PTC, PTC.text())
                                    .on_hover_text(PTC.hover_text());
                                ui.selectable_value(&mut group.composition, STC, STC.text())
                                    .on_hover_text(STC.hover_text());
                                ui.separator();
                                ui.selectable_value(&mut group.composition, SC, SC.text())
                                    .on_hover_text(SC.hover_text());
                                ui.selectable_value(&mut group.composition, PSC, PSC.text())
                                    .on_hover_text(PSC.hover_text());
                                ui.selectable_value(&mut group.composition, SSC, SSC.text())
                                    .on_hover_text(SSC.hover_text());
                            })
                            .response
                            .on_hover_text(group.composition.hover_text());
                        // Filter
                        let title = if group.filter == Default::default() {
                            FUNNEL
                        } else {
                            ui.visuals_mut().widgets.inactive = ui.visuals().widgets.active;
                            FUNNEL_X
                        };
                        let id = ui.id().with("filter");
                        let mut value =
                            ui.data_mut(|data| *data.get_temp_mut_or(id, group.filter.value));
                        ui.menu_button(title, |ui| {
                            ui.label(format!(
                                "{} {}",
                                group.composition.text(),
                                localize!("filter")
                            ));
                            let response = ui.add(
                                Slider::new(&mut value, 0.0..=1.0)
                                    .clamping(SliderClamping::Always)
                                    .logarithmic(true)
                                    .custom_formatter(|mut value, _| {
                                        if self.percent {
                                            value *= 100.0;
                                        }
                                        AnyValue::Float64(value).to_string()
                                    })
                                    .custom_parser(|value| {
                                        let mut parsed = value.parse::<f64>().ok()?;
                                        if self.percent {
                                            parsed /= 100.0;
                                        }
                                        Some(parsed)
                                    }),
                            );
                            if response.changed() {
                                ui.data_mut(|data| data.insert_temp(id, value));
                            }
                            if response.drag_stopped() || response.lost_focus() {
                                group.filter.value = value;
                            }
                            if response.clicked_elsewhere() {
                                ui.data_mut(|data| data.insert_temp(id, group.filter.value));
                            }
                        });
                        if ui.input(|input| input.key_pressed(Key::Escape)) {
                            ui.data_mut(|data| data.insert_temp(id, group.filter.value));
                        }
                    });
                    ui.end_row();
                    keep
                });
                ui.label("");
                ui.label("SSC");
                ui.end_row();

                // Join
                ui.label(localize!("join"));
                ComboBox::from_id_salt("join")
                    .selected_text(self.join.text())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.join, Join::Left, Join::Left.text())
                            .on_hover_text(Join::Left.hover_text());
                        ui.selectable_value(&mut self.join, Join::And, Join::And.text())
                            .on_hover_text(Join::And.hover_text());
                        ui.selectable_value(&mut self.join, Join::Or, Join::Or.text())
                            .on_hover_text(Join::Or.hover_text());
                    })
                    .response
                    .on_hover_text(self.join.hover_text());
                ui.end_row();

                // Sort
                ui.label(localize!("sort"));
                ComboBox::from_id_salt("sort")
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
                ComboBox::from_id_salt("order")
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
            //     ComboBox::from_id_salt("")
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
            //     ComboBox::from_id_salt("method")
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
            //             ComboBox::from_id_salt(ui.next_auto_id())
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
            //     //     ComboBox::from_id_salt(index)
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
            //     ComboBox::from_id_salt("sort")
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
            //     ComboBox::from_id_salt("order")
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

/// Join
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Join {
    Left,
    And,
    Or,
}

impl Join {
    pub(in crate::app) fn text(self) -> String {
        match self {
            Self::Left => localize!("left"),
            Self::And => localize!("and"),
            Self::Or => localize!("or"),
        }
    }

    pub(in crate::app) fn hover_text(self) -> String {
        match self {
            Self::Left => localize!("left.description"),
            Self::And => localize!("and.description"),
            Self::Or => localize!("or.description"),
        }
    }
}

impl From<Join> for JoinType {
    fn from(value: Join) -> Self {
        match value {
            Join::Left => JoinType::Left,
            Join::And => JoinType::Inner,
            Join::Or => JoinType::Full,
        }
    }
}

/// Method
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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

/// Filter
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub(in crate::app) struct Filter {
    pub(in crate::app) value: f64,
}

impl Filter {
    pub(in crate::app) const fn new() -> Self {
        Self { value: 0.0 }
        // Self { value: 0.005 }
    }
}

impl Eq for Filter {}

impl Hash for Filter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.ord().hash(state);
    }
}

impl PartialEq for Filter {
    fn eq(&self, other: &Self) -> bool {
        self.value.ord() == other.value.ord()
    }
}

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Group {
    pub(in crate::app) composition: Composition,
    pub(in crate::app) filter: Filter,
}

impl Group {
    pub(in crate::app) const fn new() -> Self {
        Self {
            composition: Composition::new(),
            filter: Filter::new(),
        }
    }
}

/// Composition
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) struct Composition {
    pub(in crate::app) kind: Kind,
    pub(in crate::app) stereospecificity: Option<Stereospecificity>,
}

impl Composition {
    pub(in crate::app) const fn new() -> Self {
        Self {
            stereospecificity: Some(Stereospecificity::Positional),
            kind: Kind::Species,
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

            UC => "UC",
            PUC => "PUC",
            SUC => "SUC",

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

            UC => "Unsaturation composition",
            PUC => "Positional unsaturation composition",
            SUC => "Stereo unsaturation composition",

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

/// Composition kind
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Kind {
    Ecn,
    Mass,
    Species,
    Type,
    Unsaturation,
}

impl Kind {
    pub(in crate::app) fn text(&self) -> &'static str {
        match self {
            Self::Ecn => "Equivalent carbon number",
            Self::Mass => "Mass",
            Self::Species => "Species",
            Self::Type => "Type",
            Self::Unsaturation => "Unsaturation",
        }
    }

    pub(in crate::app) fn hover_text(&self) -> &'static str {
        match self {
            Self::Ecn => "ECN",
            Self::Mass => "M",
            Self::Species => "S",
            Self::Type => "T",
            Self::Unsaturation => "U",
        }
    }
}
