use super::Event;
use crate::fatty_acid::FattyAcid;
use egui::{style::Widgets, DragValue, Response, ScrollArea, TextEdit, Ui, Widget};
use egui_phosphor::regular::{ARROWS_CLOCKWISE, CHECK, MINUS, PENCIL, PLUS};

/// Formula
pub(crate) struct Formula<'a> {
    pub(crate) label: &'a mut String,
    pub(crate) fatty_acid: &'a mut FattyAcid,
    pub(crate) event: &'a mut Option<Event>,
}

impl<'a> Formula<'a> {
    pub(crate) fn new(
        label: &'a mut String,
        fatty_acid: &'a mut FattyAcid,
        event: &'a mut Option<Event>,
    ) -> Self {
        Self {
            label,
            fatty_acid,
            event,
        }
    }
}

impl Widget for Formula<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let fatty_acid = self.fatty_acid;
        ui.visuals_mut().widgets = if ui.style().visuals.dark_mode {
            Widgets::dark()
        } else {
            Widgets::light()
        };
        // Label
        let mut response = ui
            .horizontal(|ui| {
                ui.label("Label");
                TextEdit::singleline(self.label)
                    .hint_text("C")
                    .desired_width(ui.available_width())
                    .show(ui)
                    .response
            })
            .inner;
        if response.changed() {
            // self.event = 
        }
        // Structure
        ui.horizontal(|ui| {
            ui.label("Carbons");
            response |= ui.add(DragValue::new(&mut fatty_acid.carbons));
            if response.changed() {
                // let bounds = fatty_acid.carbons - 1;
                // let len = fatty_acid.bounds.len();
                // for _ in 0..bounds.abs_diff(len) {
                //     if bounds > len {
                //         fatty_acid.bounds.push(0);
                //     } else {
                //         fatty_acid.bounds.pop();
                //     }
                // }
            }
            // if fatty_acid.carbons > fatty_acid.u() {}

            // if !fatty_acid.triples.is_empty() {
            //     ui.label("Triples:");
            //     for bound in &mut fatty_acid.triples {
            //         response |=
            //             ui.add(DragValue::new(bound).range(1..=fatty_acid.carbons));
            //     }
            // }
        });
        ui.horizontal(|ui| {
            ui.label("Doubles");
            if !fatty_acid.doubles.is_empty() {
                if ui.button(MINUS).clicked() {
                    // while fatty_acid.c() <= fatty_acid.u() + 1 {
                    fatty_acid.doubles.pop();
                    response.mark_changed();
                    // }
                }
            }
            for bound in &mut fatty_acid.doubles {
                response |= ui.add(DragValue::new(bound).range(1..=fatty_acid.carbons));
            }
            if ui.button(PLUS).clicked() {
                // if fatty_acid.c() > fatty_acid.u() + 1 {
                fatty_acid.doubles.push(0);
                response.mark_changed();
                // }
            }
        });
        response
        // ui.group(|ui| {
        //     ui.horizontal(|ui| {
        //         ui.label("C:");
        //         let mut c = fatty_acid.c();
        //         let mut response = ui.add(DragValue::new(&mut c).range(1..=u8::MAX));
        //         if response.changed() {
        //             let bounds = c - 1;
        //             let len = fatty_acid.bounds.len();
        //             for _ in 0..bounds.abs_diff(len) {
        //                 if bounds > len {
        //                     fatty_acid.bounds.push(1);
        //                 } else {
        //                     fatty_acid.bounds.pop();
        //                 }
        //             }
        //         }
        //         if !fatty_acid.bounds.is_empty() {
        //             ui.label("Bounds:");
        //             ScrollArea::horizontal().show(ui, |ui| {
        //                 for (index, bound) in fatty_acid.bounds.iter_mut().enumerate() {
        //                     ui.horizontal(|ui| {
        //                         ui.label(format!("{}", index + 1));
        //                         response |= ui.add(DragValue::new(bound).range(1..=3));
        //                     });
        //                 }
        //             });
        //         }
        //         response
        //     })
        //     .inner
        // })
        // .inner

        // if !fatty_acid.bounds.is_empty() {
        //     ui.horizontal(|ui| {
        //         ui.label("Bounds:");
        //         ui.group(|ui| {
        //             ScrollArea::horizontal().show(ui, |ui| {
        //                 for (index, bound) in fatty_acid.bounds.iter_mut().enumerate() {
        //                     ui.horizontal(|ui| {
        //                         ui.label(format!("{}", index + 1));
        //                         response |= ui.add(DragValue::new(bound).range(1..=3));
        //                     });
        //                 }
        //             });
        //         });
        //         // ComboBox::from_id_source(format!("FattyAcid"))
        //         //     .selected_text(format!("{fatty_acid:#}"))
        //         //     .show_ui(ui, |ui| {
        //         //         ScrollArea::horizontal().show(ui, |ui| {
        //         //             for (index, bound) in fatty_acid.bounds.iter_mut().enumerate() {
        //         //                 ui.horizontal(|ui| {
        //         //                     ui.label(format!("{}", index + 1));
        //         //                     ui.add(DragValue::new(bound).range(1..=3));
        //         //                 });
        //         //             }
        //         //         });
        //         //     });
        //         // ui.menu_button(format!("{fatty_acid:#}"), |ui| {
        //         //     for (index, bound) in fatty_acid.bounds.iter_mut().enumerate() {
        //         //         ui.vertical(|ui| {
        //         //             ui.label(format!("{}", index + 1));
        //         //             ui.add(DragValue::new(bound).range(1..=3));
        //         //         });
        //         //     }
        //         // });
        //     });
        // }
    }
}
