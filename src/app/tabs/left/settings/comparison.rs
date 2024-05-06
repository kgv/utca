use crate::app::{
    context::{
        settings::{Order, Sort},
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
            RichText::new(CentralTab::Comparison.title()).heading(),
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
                    ui.label("Sort:");
                    ComboBox::from_id_source("sort")
                        .selected_text(
                            context
                                .settings
                                .comparison
                                .sort
                                .map_or("None", |sort| sort.text()),
                        )
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.comparison.sort,
                                None,
                                "None",
                            )
                            .on_hover_text(Sort::Key.hover_text());
                            ui.selectable_value(
                                &mut context.settings.comparison.sort,
                                Some(Sort::Key),
                                Sort::Key.text(),
                            )
                            .on_hover_text(Sort::Key.hover_text());
                            ui.selectable_value(
                                &mut context.settings.comparison.sort,
                                Some(Sort::Value),
                                Sort::Value.text(),
                            )
                            .on_hover_text(Sort::Value.hover_text());
                        })
                        .response
                        .on_hover_text(
                            context
                                .settings
                                .comparison
                                .sort
                                .map_or("None", |sort| sort.hover_text()),
                        );
                    if context.settings.comparison.sort == Some(Sort::Value) {
                        // ui.label("Mode:");
                        // ComboBox::from_id_source("mode")
                        //     .selected_text(context.settings.comparison.mode.text())
                        //     .show_ui(ui, |ui| {
                        //         ui.selectable_value(
                        //             &mut context.settings.comparison.mode,
                        //             Mode::MinMax,
                        //             Mode::MinMax.text(),
                        //         )
                        //         .on_hover_text(Mode::MinMax.hover_text());
                        //         ui.selectable_value(
                        //             &mut context.settings.comparison.mode,
                        //             Mode::Sum,
                        //             Mode::Sum.text(),
                        //         )
                        //         .on_hover_text(Mode::Sum.hover_text());
                        //     })
                        //     .response
                        //     .on_hover_text(context.settings.comparison.mode.hover_text());
                        ui.spacing_mut().combo_width /= 2.0;
                        ui.label("Column:");
                        ComboBox::from_id_source("column")
                            .selected_text(context.settings.comparison.column.to_string())
                            .show_ui(ui, |ui| {
                                for index in 0..context.state.entries.len() {
                                    ui.selectable_value(
                                        &mut context.settings.comparison.column,
                                        index,
                                        index.to_string(),
                                    )
                                    .on_hover_text(format!("Sort by {index} column values"));
                                }
                            })
                            .response
                            .on_hover_text(format!(
                                r#"Sort by "{}" column values"#,
                                context.state.entries[context.settings.comparison.column]
                                    .meta
                                    .name,
                            ));
                    }
                });
                if context.settings.comparison.sort.is_some() {
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
                }
            },
        );
    }
}
