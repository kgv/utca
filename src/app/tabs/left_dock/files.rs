use crate::utils::{egui::Display, higher_order_functions::with_index};
use egui::{text::LayoutJob, Direction, DroppedFile, Label, Layout, RichText, TextStyle, Ui, Vec2};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Files
#[derive(Debug, Default, Deserialize, Serialize)]
pub(in crate::app) struct Files {
    pub(in crate::app) files: Vec<DroppedFile>,
    pub(in crate::app) index: usize,
}

impl Files {
    pub(super) fn content(&mut self, ui: &mut Ui) {
        self.files.retain(with_index(|index, file: &DroppedFile| {
            let height = ui.spacing().interact_size.y;
            // let width = ui.spacing().item_spacing.x;
            let mut keep = true;
            // ui.columns(3, |ui| {
            //     ui[0].radio_value(&mut self.index, index, "");
            //     ui[1].label(file.display().to_string());
            //     keep = !ui[2].button("🗙").clicked();
            // });

            StripBuilder::new(ui)
                .size(Size::exact(height))
                .size(Size::remainder())
                .size(Size::exact(height))
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.radio_value(&mut self.index, index, "");
                    });
                    strip.cell(|ui| {
                        let size = ui.available_size_before_wrap().x;
                        let height = ui.text_style_height(&TextStyle::Body);
                        let mut text = file.display().to_string();
                        text.truncate((2.5 * size / height) as usize);
                        ui.label(text);
                        // ui.add_sized(Vec2 { y: 0.0, ..size }, Label::new(text).wrap(true));
                    });
                    strip.cell(|ui| {
                        keep = !ui.button("🗙").clicked();
                    });
                });

            // TableBuilder::new(ui)
            //     // .cell_layout(Layout::centered_and_justified(Direction::TopDown))
            //     .column(Column::exact(width))
            //     .column(Column::remainder())
            //     .column(Column::exact(width))
            //     // .auto_shrink([false; 2])
            //     .striped(true)
            //     // .header(height, |mut row| {
            //     //     row.col(|_| {});
            //     // })
            //     .body(|mut body| {
            //         body.row(height, |mut row| {
            //             row.col(|ui| {
            //                 ui.radio_value(&mut self.index, index, "");
            //             });
            //             row.col(|ui| {
            //                 ui.label(file.display().to_string());
            //             });
            //             row.col(|ui| {
            //                 keep = !ui.button("🗙").clicked();
            //             });
            //         });
            //     });
            keep
        }));
    }
}

impl Deref for Files {
    type Target = Vec<DroppedFile>;

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}
