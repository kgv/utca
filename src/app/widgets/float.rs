use egui::{Color32, Response, RichText, Ui, Widget};
use polars::prelude::AnyValue;

/// Float value
#[derive(Clone, Copy, Debug, Default)]
pub(in crate::app) struct FloatValue {
    pub(in crate::app) value: Option<f64>,
    pub(in crate::app) color: bool,
    pub(in crate::app) hover: bool,
    pub(in crate::app) percent: bool,
    pub(in crate::app) precision: Option<usize>,
}

impl FloatValue {
    pub(in crate::app) fn new(value: Option<f64>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    pub(in crate::app) fn color(self, color: bool) -> Self {
        Self { color, ..self }
    }

    pub(in crate::app) fn percent(self, percent: bool) -> Self {
        Self { percent, ..self }
    }

    pub(in crate::app) fn precision(self, precision: usize) -> Self {
        Self {
            precision: Some(precision),
            ..self
        }
    }
}

impl Widget for FloatValue {
    fn ui(self, ui: &mut Ui) -> Response {
        let any_value = AnyValue::from(self.value);
        let text = match self.value {
            Some(mut value) => {
                if self.percent {
                    value *= 100.0;
                }
                match self.precision {
                    Some(precision) => RichText::new(format!("{value:.precision$}")),
                    None => RichText::new(AnyValue::from(value).to_string()),
                }
            }
            None => {
                let mut text = RichText::new(AnyValue::Float64(0.0).to_string());
                if self.color {
                    text = text.color(Color32::RED);
                }
                text
            }
        };
        let response = ui.label(text);
        // if self.hover {
        //     response = response.on_hover_text(text);
        // }
        response
    }
}

// impl Widget for FloatValue {
//     fn ui(self, ui: &mut Ui) -> Response {
//         let text = match self.value {
//             Some(mut value) => {
//                 if self.percent {
//                     value *= 100.0;
//                 }
//                 match self.precision {
//                     Some(precision) => RichText::new(format!("{value:.precision$}")),
//                     None => RichText::new(AnyValue::from(value).to_string()),
//                 }
//             }
//             None => {
//                 let mut text = RichText::new(AnyValue::Null.to_string());
//                 if self.color {
//                     text = text.color(Color32::RED);
//                 }
//                 text
//             }
//         };
//         let response = ui.label(text);
//         // if self.hover {
//         //     response = response.on_hover_text(text);
//         // }
//         response
//     }
// }
