use crate::fatty_acid::FattyAcid;
use egui::{style::Widgets, DragValue, Response, ScrollArea, TextEdit, Ui, Widget};
use egui_phosphor::regular::{ARROWS_CLOCKWISE, CHECK, MINUS, PENCIL, PLUS};

/// Formula
pub(super) struct Formula<'a> {
    pub(super) label: &'a mut String,
    pub(super) fatty_acid: &'a mut FattyAcid,
}

impl<'a> Formula<'a> {
    pub(super) fn new(label: &'a mut String, fatty_acid: &'a mut FattyAcid) -> Self {
        Self { label, fatty_acid }
    }
}

impl Formula<'_> {
    pub(super) fn ui(self, ui: &mut Ui) -> Option<Change> {
        let mut change = None;
        ui.visuals_mut().widgets = if ui.style().visuals.dark_mode {
            Widgets::dark()
        } else {
            Widgets::light()
        };
        // Label
        ui.horizontal(|ui| {
            ui.label("Label");
            if TextEdit::singleline(self.label)
                .hint_text("C")
                .desired_width(ui.available_width())
                .show(ui)
                .response
                .changed()
            {
                change = Some(Change::Label);
            }
        });
        // Carbons
        ui.horizontal(|ui| {
            ui.label("Carbons");
            if ui
                .add(DragValue::new(&mut self.fatty_acid.carbons))
                .changed()
            {
                change = Some(Change::Carbons);
            }
        });
        // Doubles
        ui.horizontal(|ui| {
            let mut response = ui.label("Doubles");
            if !self.fatty_acid.doubles.is_empty() {
                if ui.button(MINUS).clicked() {
                    self.fatty_acid.doubles.pop();
                    response.mark_changed();
                }
            }
            let end = self.fatty_acid.b();
            for bound in &mut self.fatty_acid.doubles {
                response |= ui.add(DragValue::new(bound).range(0..=end));
            }
            if ui.button(PLUS).clicked() {
                if self.fatty_acid.b() > self.fatty_acid.u() {
                    self.fatty_acid.doubles.push(0);
                    response.mark_changed();
                }
            }
            if response.changed() {
                change = Some(Change::Doubles);
            }
        });
        change
    }
}

pub(super) enum Change {
    Label,
    Carbons,
    Doubles,
    Triples,
}
