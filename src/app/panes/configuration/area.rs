use egui::{DragValue, Response, Ui, Widget};

/// Area
pub(in crate::app) struct Area<'a> {
    pub(in crate::app) value: &'a mut f64,
    pub(in crate::app) editable: bool,
    pub(in crate::app) precision: usize,
}

impl<'a> Area<'a> {
    pub(in crate::app) fn new(value: &'a mut f64, editable: bool, precision: usize) -> Self {
        Self {
            value,
            editable,
            precision,
        }
    }
}

impl Widget for Area<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.editable {
            ui.add(
                DragValue::new(self.value)
                    .range(0.0..=f64::MAX)
                    .custom_formatter(|value, _| format!("{value:.*}", self.precision)),
            )
        } else {
            ui.label(format!("{:.*}", self.precision, self.value))
        }
        .on_hover_text(self.value.to_string())
    }
}
