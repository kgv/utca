use crate::{
    fatty_acid::FattyAcid,
    localization::{FATTY_ACID_MASS, METHYL_ESTER_MASS, PROPERTIES},
    r#const::relative_atomic_mass::{CH2, O2},
};
use egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};

/// Properties
pub(crate) struct Properties<'a> {
    pub(crate) fatty_acid: &'a FattyAcid,
}

impl<'a> Properties<'a> {
    pub(crate) fn new(fatty_acid: &'a mut FattyAcid) -> Self {
        Self { fatty_acid }
    }
}

impl Widget for Properties<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.heading(&PROPERTIES);
        let height = ui.spacing().interact_size.y;
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto())
            .column(Column::remainder())
            .body(|mut body| {
                let mass = self.fatty_acid.mass();
                body.row(height, |mut row| {
                    row.col(|ui| {
                        ui.label(&FATTY_ACID_MASS);
                    });
                    let value = mass;
                    row.col(|ui| {
                        ui.label(value.to_string());
                    });
                });
                body.row(height, |mut row| {
                    row.col(|ui| {
                        ui.label(&METHYL_ESTER_MASS);
                    });
                    let value = mass + CH2;
                    row.col(|ui| {
                        ui.label(value.to_string());
                    });
                });
            });
        response
    }
}
