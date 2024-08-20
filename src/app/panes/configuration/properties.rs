use crate::{
    fatty_acid::FattyAcid,
    localization::Localization,
    r#const::relative_atomic_mass::{CH2, O2},
};
use egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use fluent_content::Content;

/// Properties
pub(crate) struct Properties<'a> {
    pub(crate) fatty_acid: &'a FattyAcid,
    pub(crate) localization: &'a Localization,
}

impl<'a> Properties<'a> {
    pub(crate) fn new(fatty_acid: &'a mut FattyAcid, localization: &'a Localization) -> Self {
        Self {
            fatty_acid,
            localization,
        }
    }
}

impl Widget for Properties<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.heading(self.localization.content("properties").unwrap_or_default());
        let height = ui.spacing().interact_size.y;
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto())
            .column(Column::remainder())
            .body(|mut body| {
                let mass = self.fatty_acid.mass();
                body.row(height, |mut row| {
                    row.col(|ui| {
                        ui.label(
                            self.localization
                                .content("fatty_acid_mass")
                                .unwrap_or_default(),
                        );
                    });
                    let value = mass;
                    row.col(|ui| {
                        ui.label(value.to_string());
                    });
                });
                body.row(height, |mut row| {
                    row.col(|ui| {
                        ui.label(
                            self.localization
                                .content("methyl_ester_mass")
                                .unwrap_or_default(),
                        );
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
