use eframe::{App, Frame};
use egui::Context;

/// Extension methods for [`Ui`]
pub(crate) trait AppExt: App {
    fn pre_update(&mut self, _ctx: &Context, _frame: &mut Frame) {}

    fn post_update(&mut self, _ctx: &Context, _frame: &mut Frame) {}
}
