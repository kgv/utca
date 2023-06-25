use super::{InnerResponseExt, ResponseExt};
use crate::utils::BoundExt;
use eframe::emath::Numeric;
use egui::{Align, Color32, DragValue, Layout, Response, Separator, Ui, Widget, WidgetText};
use egui_extras::TableBody;
use serde::{Deserialize, Serialize};
use std::{
    default::default,
    ops::{Bound, RangeInclusive},
};

/// Extension methods for [`Ui`]
pub trait UiExt {
    fn drag_bound<T>(
        &mut self,
        bound: &mut Bound<T>,
        f: impl FnMut(DragValue) -> DragValue,
    ) -> Response
    where
        for<'a> T: Numeric + Serialize + Deserialize<'a> + Send + Sync;

    fn drag_option<T: Numeric>(
        &mut self,
        value: &mut Option<T>,
        clamp_range: RangeInclusive<T>,
        speed: f64,
    ) -> Response;

    fn drag_percent<T: Numeric>(&mut self, value: &mut T) -> Response;

    // fn option_value<T: Default>(
    //     &mut self,
    //     option: &mut Option<T>,
    //     text: impl Into<WidgetText>,
    // ) -> Response;
}

impl UiExt for Ui {
    fn drag_bound<T>(
        &mut self,
        bound: &mut Bound<T>,
        mut f: impl FnMut(DragValue) -> DragValue,
    ) -> Response
    where
        for<'a> T: Numeric + Serialize + Deserialize<'a> + Send + Sync,
    {
        let id = self.id().with("value");
        let mut value = bound.value().copied().unwrap_or_else(|| {
            self.data_mut(|data| data.get_persisted(id).unwrap_or(T::from_f64(f64::NAN)))
        });
        match bound {
            Bound::Unbounded => self
                .add_enabled_ui(false, |ui| {
                    ui.add(f(
                        DragValue::new(&mut value).custom_formatter(|_, _| "∞".to_owned())
                    ))
                })
                .flatten(),
            Bound::Included(value) | Bound::Excluded(value) => self.add(f(DragValue::new(value))),
        }
        .on_hover_text(bound.variant_name())
        .context_menu(|ui| {
            let response = ui.selectable_value(bound, Bound::Included(value), "Included")
                | ui.selectable_value(bound, Bound::Excluded(value), "Excluded")
                | ui.selectable_value(bound, Bound::Unbounded, "Unbounded")
                    .with_clicked(|| {
                        self.data_mut(|data| data.insert_persisted(id, value));
                    });
            if response.clicked() {
                ui.close_menu();
            }
            // if ui.ui_contains_pointer() && ui.input(|input| input.pointer.any_click()) {
            //     ui.close_menu();
            // }
        })
    }

    fn drag_option<T: Numeric>(
        &mut self,
        value: &mut Option<T>,
        clamp_range: RangeInclusive<T>,
        speed: f64,
    ) -> Response {
        let mut default = T::from_f64(0.0);
        let enabled = value.is_some();
        self.add_enabled_ui(enabled, |ui| {
            ui.add(
                DragValue::new(value.as_mut().unwrap_or(&mut default))
                    .clamp_range(clamp_range)
                    .speed(speed)
                    .custom_formatter(|value, _| {
                        if enabled {
                            value.to_string()
                        } else {
                            "-".to_owned()
                        }
                    }),
            )
        })
        .flatten()
        .context_menu(|ui| {
            if enabled && ui.button("None").clicked() {
                *value = None;
                ui.close_menu();
            } else if !enabled && ui.button("Some").clicked() {
                *value = Some(default);
                ui.close_menu();
            }
        })
    }

    fn drag_percent<T: Numeric>(&mut self, value: &mut T) -> Response {
        DragValue::new(value)
            .clamp_range(0..=100)
            .speed(0.1)
            .suffix('%')
            .ui(self)
    }

    // fn option_value<T: Default>(
    //     &mut self,
    //     option: &mut Option<T>,
    //     text: impl Into<WidgetText>,
    // ) -> Response {
    //     let mut checked = option.is_some();
    //     let mut response = self.checkbox(&mut checked, text);
    //     if response.changed() {
    //         *option = option.map_or(Some(default()), |_| None);
    //         response.mark_changed();
    //     }
    //     response
    // }
}
