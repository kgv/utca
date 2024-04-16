use egui::{text::LayoutJob, Align, Color32, Style, TextFormat, TextStyle, Ui, Visuals};
use std::sync::Arc;

// struct CompositeText {
//     color: Color32,
//     layout_job: LayoutJob,
//     style: Arc<Style>,
// }

// impl CompositeText {
//     pub fn new(ui: &Ui) -> Self {
//         let style = ui.style().clone();
//         let color = if style.visuals.dark_mode {
//             Visuals::dark().text_color()
//         } else {
//             Visuals::light().text_color()
//         };
//         let layout_job = LayoutJob::default();
//         Self {
//             color,
//             layout_job,
//             style,
//         }
//     }

//     pub fn widget(&mut self) -> &mut Self {
//         self.color = if self.style.visuals.dark_mode {
//             Visuals::dark().widgets.inactive.text_color()
//         } else {
//             Visuals::light().widgets.inactive.text_color()
//         };
//         self
//     }

//     pub fn text(&mut self, text: &str, text_style: TextStyle) -> &mut Self {
//         let font_id = text_style.resolve(&self.style);
//         self.layout_job.append(
//             text,
//             0.0,
//             TextFormat {
//                 color: self.color,
//                 font_id,
//                 ..Default::default()
//             },
//         );
//         self
//     }

//     pub fn subscript(&mut self, text: &str) -> &mut Self {
//         let font_id = TextStyle::Small.resolve(&self.style);
//         self.layout_job.append(
//             text,
//             1.0,
//             TextFormat {
//                 color: self.color,
//                 font_id,
//                 ..Default::default()
//             },
//         );
//         self
//     }
// }

/// Extension methods for [`Ui`]
pub trait UiExt {
    fn subscripted_text(&self, text: &str, subscripted_text: &str) -> LayoutJob;
    fn subscripted_widget(&self, text: &str, subscripted_text: &str) -> LayoutJob;
    fn subscripted(&self, text: &str, subscripted_text: &str, color: Color32) -> LayoutJob;
}

impl UiExt for Ui {
    fn subscripted_text(&self, text: &str, subscripted_text: &str) -> LayoutJob {
        let color = if self.visuals().dark_mode {
            Visuals::dark().text_color()
        } else {
            Visuals::light().text_color()
        };
        self.subscripted(text, subscripted_text, color)
    }

    fn subscripted_widget(&self, text: &str, subscripted_text: &str) -> LayoutJob {
        let color = if self.visuals().dark_mode {
            Visuals::dark().widgets.inactive.text_color()
        } else {
            Visuals::light().widgets.inactive.text_color()
        };
        self.subscripted(text, subscripted_text, color)
    }

    fn subscripted(&self, text: &str, subscripted_text: &str, color: Color32) -> LayoutJob {
        let body_font_id = TextStyle::Body.resolve(self.style());
        let small_font_id = TextStyle::Small.resolve(self.style());
        let mut layout_job = LayoutJob::default();
        layout_job.append(
            text,
            0.0,
            TextFormat {
                color,
                // font_id: body_font_id,
                ..Default::default()
            },
        );
        layout_job.append(
            subscripted_text,
            1.0,
            TextFormat {
                color,
                font_id: small_font_id,
                valign: Align::BOTTOM,
                ..Default::default()
            },
        );
        layout_job
    }
}
