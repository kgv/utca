use crate::{
    app::MAX_PRECISION,
    localization::{
        ADDUCT, ASCENDING, ASCENDING_DESCRIPTION, COMPOSITION, DESCENDING, DESCENDING_DESCRIPTION,
        GUNSTONE, GUNSTONE_DESCRIPTION, KEY, KEY_DESCRIPTION, METHOD, ORDER, PERCENT, PRECISION,
        SORT, VALUE, VALUE_DESCRIPTION, VANDER_WAL, VANDER_WAL_DESCRIPTION,
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
        });
        UiResponse::None
    }
}

/// Method
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) enum Method {
    Gunstone,
    VanderWal,
}

impl Method {
    pub(in crate::app) fn text(&self) -> &'static str {
        match self {
            Self::Gunstone => &GUNSTONE,
            Self::VanderWal => &VANDER_WAL,
        }
    }

    pub(in crate::app) fn hover_text(&self) -> &'static str {
        match self {
            Self::Gunstone => &GUNSTONE_DESCRIPTION,
            Self::VanderWal => &VANDER_WAL_DESCRIPTION,
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
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Key => &KEY,
            Self::Value => &VALUE,
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Key => &KEY_DESCRIPTION,
            Self::Value => &VALUE_DESCRIPTION,
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
    pub(in crate::app) fn text(self) -> &'static str {
        match self {
            Self::Ascending => &ASCENDING,
            Self::Descending => &DESCENDING,
        }
    }

    pub(in crate::app) fn hover_text(self) -> &'static str {
        match self {
            Self::Ascending => &ASCENDING_DESCRIPTION,
            Self::Descending => &DESCENDING_DESCRIPTION,
        }
    }
}
