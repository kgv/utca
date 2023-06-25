use eframe::{App, Frame};
use egui::Context;

/// Extension methods for [`Ui`]
pub(crate) trait AppExt: App {
    fn pre_update(&mut self, ctx: &Context, frame: &mut Frame) {}

    fn post_update(&mut self, ctx: &Context, frame: &mut Frame) {}
}
