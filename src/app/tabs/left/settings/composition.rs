use crate::app::{
    context::{
        settings::{composition::Checkable, Order, Sort},
        Context,
    },
    tabs::CentralTab,
    view::View,
    MAX_PRECISION,
};
use egui::{ComboBox, Id, RichText, Slider, Ui};
use egui_dnd::dnd;

/// Left composition tab
pub(super) struct Composition<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Composition<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Composition<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        ui.collapsing(
            RichText::new(CentralTab::Composition.title()).heading(),
            |ui| {
                ui.horizontal(|ui| {
                    ui.toggle_value(&mut context.settings.composition.resizable, "↔ Resizable")
                        .on_hover_text("Resize table columns")
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    let precision = &mut context.settings.composition.precision;
                    if ui.add(Slider::new(precision, 0..=MAX_PRECISION)).changed()
                        && context.settings.link
                    {
                        context.settings.configuration.precision = *precision;
                        context.settings.calculation.precision = *precision;
                        context.settings.visualization.precision = *precision;
                        context.settings.comparison.precision = *precision;
                    }
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.horizontal(|ui| {
                    ui.label("Percent:");
                    ui.checkbox(&mut context.settings.composition.percent, "");
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.separator();
                ui.label("Group:");
                dnd(ui, Id::new("dnd").with("composition")).show_vec(
                    &mut context.settings.composition.groups,
                    |ui,
                     Checkable {
                         value: group,
                         checked,
                     },
                     handle,
                     state| {
                        ui.horizontal(|ui| {
                            handle.ui(ui, |ui| {
                                let _ = ui.button(if state.dragged { "👊" } else { "✋" });
                            });
                            ui.checkbox(checked, "");
                            ui.label(group.text()).on_hover_text(group.hover_text());
                        });
                    },
                );
                ui.horizontal(|ui| {
                    ui.label("Sort:");
                    ComboBox::from_id_source("sort")
                        .selected_text(context.settings.composition.sort.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.composition.sort,
                                Sort::Key,
                                Sort::Key.text(),
                            )
                            .on_hover_text(Sort::Key.hover_text());
                            ui.selectable_value(
                                &mut context.settings.composition.sort,
                                Sort::Value,
                                Sort::Value.text(),
                            )
                            .on_hover_text(Sort::Value.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.composition.sort.hover_text());
                });
                ui.horizontal(|ui| {
                    ui.label("Order:");
                    ComboBox::from_id_source("order")
                        .selected_text(context.settings.composition.order.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.composition.order,
                                Order::Ascending,
                                Order::Ascending.text(),
                            )
                            .on_hover_text(Order::Ascending.hover_text());
                            ui.selectable_value(
                                &mut context.settings.composition.order,
                                Order::Descending,
                                Order::Descending.text(),
                            )
                            .on_hover_text(Order::Descending.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.composition.order.hover_text());
                });
            },
        );
    }
}
