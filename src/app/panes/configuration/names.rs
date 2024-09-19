use crate::{
    fatty_acid::FattyAcid,
    localization::{lowercase, localize},
};
use egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};

/// Names
pub(in crate::app) struct Names<'a> {
    pub(in crate::app) fatty_acid: &'a FattyAcid,
}

impl<'a> Names<'a> {
    pub(in crate::app) fn new(fatty_acid: &'a mut FattyAcid) -> Self {
        Self { fatty_acid }
    }
}

impl Widget for Names<'_> {
    // fn ui(self, ui: &mut Ui) -> Response {
    //     let response = ui.heading("Names");
    //     let id = self.fatty_acid.id();
    //     if let Some(abbreviation) = self.localization.content(&format!("fa_{id}.abbreviation")) {
    //         ui.horizontal(|ui| {
    //             ui.label("Abbreviation:");
    //             ui.label(abbreviation);
    //         });
    //     }
    //     if let Some(common_name) = self.localization.content(&format!("fa_{id}.common-name")) {
    //         ui.horizontal(|ui| {
    //             ui.label("Common name:");
    //             ui.label(common_name);
    //         });
    //     }
    //     if let Some(systematic_name) = self
    //         .localization
    //         .content(&format!("fa_{id}.systematic-name"))
    //     {
    //         ui.horizontal(|ui| {
    //             ui.label("Systematic name:");
    //             ui.label(systematic_name);
    //         });
    //     }
    //     response
    // }
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.heading(localize!("names"));
        let height = ui.spacing().interact_size.y;
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto())
            .column(Column::remainder())
            .body(|mut body| {
                let id = self.fatty_acid.id();
                body.row(height, |mut row| {
                    if let Some(abbreviation) = lowercase!(&format!("fa_{id}.abbreviation")) {
                        row.col(|ui| {
                            ui.label(localize!("abbreviation"));
                        });
                        row.col(|ui| {
                            ui.label(abbreviation);
                        });
                    }
                });
                body.row(height, |mut row| {
                    if let Some(common_name) = lowercase!(&format!("fa_{id}.common_name")) {
                        row.col(|ui| {
                            ui.label(localize!("common_name"));
                        });
                        row.col(|ui| {
                            ui.label(common_name);
                        });
                    }
                });
                body.row(height, |mut row| {
                    if let Some(systematic_name) = lowercase!(&format!("fa_{id}.systematic_name")) {
                        row.col(|ui| {
                            ui.label(localize!("systematic_name"));
                        });
                        row.col(|ui| {
                            ui.label(systematic_name);
                        });
                    }
                });
            });
        response
    }
}
