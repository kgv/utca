use crate::{fatty_acid::FattyAcid, localization::localize, r#const::relative_atomic_mass::CH2};
use egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};

/// Properties
pub(in crate::app) struct Properties<'a> {
    pub(in crate::app) fatty_acid: &'a FattyAcid,
}

impl<'a> Properties<'a> {
    pub(in crate::app) fn new(fatty_acid: &'a mut FattyAcid) -> Self {
        Self { fatty_acid }
    }
}

impl Widget for Properties<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.heading(localize!("properties"));
        let height = ui.spacing().interact_size.y;
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto())
            .column(Column::remainder())
            .body(|mut body| {
                let mass = self.fatty_acid.mass();
                body.row(height, |mut row| {
                    row.col(|ui| {
                        ui.label(localize!("fatty_acid_mass"));
                    });
                    let value = mass;
                    row.col(|ui| {
                        ui.label(value.to_string());
                    });
                });
                body.row(height, |mut row| {
                    row.col(|ui| {
                        ui.label(localize!("methyl_ester_mass"));
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
