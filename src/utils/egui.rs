use egui::{Align, Layout, Rect, Response, Ui};
use egui_extras::TableRow;

/// Right align column
pub trait RightAlignColumn {
    fn right_align_col<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> (Rect, Response);

    fn left_align_col<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> (Rect, Response);
}

impl RightAlignColumn for TableRow<'_, '_> {
    fn right_align_col<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> (Rect, Response) {
        self.col(|ui| {
            ui.with_layout(
                Layout::left_to_right(Align::Center)
                    .with_main_align(Align::RIGHT)
                    .with_main_justify(true),
                content,
            );
        })
    }

    fn left_align_col<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> (Rect, Response) {
        self.col(|ui| {
            ui.with_layout(
                Layout::left_to_right(Align::Center)
                    .with_main_align(Align::LEFT)
                    .with_main_justify(true),
                content,
            );
        })
    }
}
