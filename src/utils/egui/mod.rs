pub(crate) use self::{
    content::Content,
    display::Trait as Display,
    response::{InnerResponseExt, ResponseExt},
    table::{Separate, TableRowExt},
    ui::UiExt,
};

use egui::{Response, Ui};
use std::ops::BitOr;

pub trait SelectableValueFromIter<T> {
    fn selectable_value_from_iter(
        &mut self,
        current_value: &mut T,
        values: impl Iterator<Item = T>,
    ) -> Response;
}

impl<T> SelectableValueFromIter<T> for Ui
where
    T: PartialEq + std::fmt::Display + Copy,
{
    fn selectable_value_from_iter(
        &mut self,
        current_value: &mut T,
        values: impl Iterator<Item = T>,
    ) -> Response {
        values
            .map(|value| self.selectable_value(current_value, value, format!("{value}")))
            .reduce(BitOr::bitor)
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "⚠ No items")
            })
    }
}

mod app;
mod color;
mod content;
mod display;
mod response;
mod table;
mod ui;
