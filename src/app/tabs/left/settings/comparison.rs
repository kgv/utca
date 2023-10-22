use crate::app::{
    context::{
        settings::{
            comparison::Mode,
            composition::Group::{Ecn, Ptc},
            Group, Order, Sort,
        },
        Context,
    },
    tabs::CentralTab,
    view::View,
    MAX_PRECISION,
};
use egui::{ComboBox, RichText, Slider, Ui};

/// Left comparison tab
pub(super) struct Comparison<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Comparison<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Comparison<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        ui.collapsing(
            RichText::new(CentralTab::Comparison.to_string()).heading(),
            |ui| {
                ui.horizontal(|ui| {
                    ui.toggle_value(&mut context.settings.comparison.resizable, "↔ Resizable")
                        .on_hover_text("Resize table columns")
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    let precision = &mut context.settings.comparison.precision;
                    if ui.add(Slider::new(precision, 0..=MAX_PRECISION)).changed()
                        && context.settings.link
                    {
                        context.settings.configuration.precision = *precision;
                        context.settings.calculation.precision = *precision;
                        context.settings.composition.precision = *precision;
                        context.settings.visualization.precision = *precision;
                    }
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.horizontal(|ui| {
                    ui.label("Percent:");
                    ui.checkbox(&mut context.settings.comparison.percent, "");
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Group:");
                    let response = ComboBox::from_id_source("group")
                        .selected_text(context.settings.comparison.group.map_or("", Group::text))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.comparison.group,
                                None,
                                "None",
                            )
                            .on_hover_text("Don't group");
                            ui.selectable_value(
                                &mut context.settings.comparison.group,
                                Some(Group::Composition(Ecn)),
                                Group::Composition(Ecn).text(),
                            )
                            .on_hover_text(Group::Composition(Ecn).hover_text());
                            ui.selectable_value(
                                &mut context.settings.comparison.group,
                                Some(Group::Composition(Ptc)),
                                Group::Composition(Ptc).text(),
                            )
                            .on_hover_text(Group::Composition(Ptc).hover_text());
                            ui.selectable_value(
                                &mut context.settings.comparison.group,
                                Some(Group::Occurrence),
                                Group::Occurrence.text(),
                            )
                            .on_hover_text(Group::Occurrence.hover_text());
                        })
                        .response;
                    if let Some(group) = context.settings.comparison.group {
                        response.on_hover_text(group.hover_text());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Sort:");
                    ComboBox::from_id_source("sort")
                        .selected_text(context.settings.comparison.sort.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.comparison.sort,
                                Sort::Key,
                                Sort::Key.text(),
                            )
                            .on_hover_text(Sort::Key.hover_text());
                            ui.selectable_value(
                                &mut context.settings.comparison.sort,
                                Sort::Value,
                                Sort::Value.text(),
                            )
                            .on_hover_text(Sort::Value.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.comparison.sort.hover_text());
                    if context.settings.comparison.sort == Sort::Value {
                        ui.label("Mode:");
                        ComboBox::from_id_source("mode")
                            .selected_text(context.settings.comparison.mode.text())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut context.settings.comparison.mode,
                                    Mode::MinMax,
                                    Mode::MinMax.text(),
                                )
                                .on_hover_text(Mode::MinMax.hover_text());
                                ui.selectable_value(
                                    &mut context.settings.comparison.mode,
                                    Mode::Sum,
                                    Mode::Sum.text(),
                                )
                                .on_hover_text(Mode::Sum.hover_text());
                            })
                            .response
                            .on_hover_text(context.settings.comparison.mode.hover_text());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Order:");
                    ComboBox::from_id_source("order")
                        .selected_text(context.settings.comparison.order.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.comparison.order,
                                Order::Ascending,
                                Order::Ascending.text(),
                            )
                            .on_hover_text(Order::Ascending.hover_text());
                            ui.selectable_value(
                                &mut context.settings.comparison.order,
                                Order::Descending,
                                Order::Descending.text(),
                            )
                            .on_hover_text(Order::Descending.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.comparison.order.hover_text());
                });
            },
        );
    }
}
