use crate::{app::context::Context, utils::higher_order_functions::with_index};
use egui::{DroppedFile, TextStyle, Ui};
use egui_ext::DroppedFileExt;
use egui_extras::{Size, StripBuilder};
use serde::{Deserialize, Serialize};

/// Files
#[derive(Debug, Default, Deserialize, Serialize)]
pub(super) struct Files;

impl Files {
    pub(super) fn view(ui: &mut Ui, context: &mut Context) {
        // self.files.retain(with_index(|index, file: &DroppedFile| {
        //     let height = ui.spacing().interact_size.y;
        //     // let width = ui.spacing().item_spacing.x;
        //     let mut keep = true;
        //     // ui.columns(3, |ui| {
        //     //     ui[0].radio_value(&mut self.index, index, "");
        //     //     ui[1].label(file.display().to_string());
        //     //     keep = !ui[2].button("🗙").clicked();
        //     // });

        //     StripBuilder::new(ui)
        //         .size(Size::exact(height))
        //         .size(Size::remainder())
        //         .size(Size::exact(height))
        //         .horizontal(|mut strip| {
        //             strip.cell(|ui| {
        //                 ui.radio_value(&mut self.index, index, "");
        //             });
        //             strip.cell(|ui| {
        //                 let size = ui.available_size_before_wrap().x;
        //                 let height = ui.text_style_height(&TextStyle::Body);
        //                 let mut text = file.display().to_string();
        //                 text.truncate((2.5 * size / height) as usize);
        //                 ui.label(text);
        //                 // ui.add_sized(Vec2 { y: 0.0, ..size }, Label::new(text).wrap(true));
        //             });
        //             strip.cell(|ui| {
        //                 keep = !ui.button("🗙").clicked();
        //             });
        //         });

        //     // TableBuilder::new(ui)
        //     //     // .cell_layout(Layout::centered_and_justified(Direction::TopDown))
        //     //     .column(Column::exact(width))
        //     //     .column(Column::remainder())
        //     //     .column(Column::exact(width))
        //     //     // .auto_shrink([false; 2])
        //     //     .striped(true)
        //     //     // .header(height, |mut row| {
        //     //     //     row.col(|_| {});
        //     //     // })
        //     //     .body(|mut body| {
        //     //         body.row(height, |mut row| {
        //     //             row.col(|ui| {
        //     //                 ui.radio_value(&mut self.index, index, "");
        //     //             });
        //     //             row.col(|ui| {
        //     //                 ui.label(file.display().to_string());
        //     //             });
        //     //             row.col(|ui| {
        //     //                 keep = !ui.button("🗙").clicked();
        //     //             });
        //     //         });
        //     //     });
        //     keep
        // }));
    }
}
