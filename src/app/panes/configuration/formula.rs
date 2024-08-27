use crate::fatty_acid::FattyAcid;
use egui::{style::Widgets, DragValue, Response, ScrollArea, Ui, Widget};

/// Formula
pub(crate) struct Formula<'a> {
    pub(crate) fatty_acid: &'a mut FattyAcid,
}

impl<'a> Formula<'a> {
    pub(crate) fn new(fatty_acid: &'a mut FattyAcid) -> Self {
        Self { fatty_acid }
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
        ScrollArea::horizontal()
            .show(ui, |ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("C:");
                        let mut c = fatty_acid.c();
                        let mut response = ui.add(DragValue::new(&mut c).range(1..=u8::MAX));
                        if response.changed() {
                            let bounds = c - 1;
                            let len = fatty_acid.bounds.len();
                            for _ in 0..bounds.abs_diff(len) {
                                if bounds > len {
                                    fatty_acid.bounds.push(0);
                                } else {
                                    fatty_acid.bounds.pop();
                                }
                            }
                        }
                        if !fatty_acid.bounds.is_empty() {
                            ui.label("Bounds:");
                            for (index, bound) in fatty_acid.bounds.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}", index + 1));
                                    response |= ui.add(
                                        DragValue::new(bound)
                                            .range(0..=2)
                                            .custom_formatter(|bound, _| (bound + 1.0).to_string()),
                                    );
                                });
                            }
                        }
                        response
                    })
                    .inner
                })
                .inner
            })
            .inner
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
