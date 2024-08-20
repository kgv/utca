use crate::{fatty_acid::FattyAcid, localization::Localization};
use egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use fluent_content::Content;

/// Names
pub(crate) struct Names<'a> {
    pub(crate) fatty_acid: &'a FattyAcid,
    pub(crate) localization: &'a Localization,
}

impl<'a> Names<'a> {
    pub(crate) fn new(fatty_acid: &'a mut FattyAcid, localization: &'a Localization) -> Self {
        Self {
            fatty_acid,
            localization,
        }
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
        let response = ui.heading(self.localization.content("names").unwrap_or_default());
        let height = ui.spacing().interact_size.y;
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto())
            .column(Column::remainder())
            .body(|mut body| {
                let id = self.fatty_acid.id();
                body.row(height, |mut row| {
                    if let Some(abbreviation) =
                        self.localization.content(&format!("fa_{id}.abbreviation"))
                    {
                        row.col(|ui| {
                            ui.label(
                                self.localization
                                    .content("abbreviation")
                                    .unwrap_or_default(),
                            );
                        });
                        row.col(|ui| {
                            ui.label(abbreviation);
                        });
                    }
                });
                body.row(height, |mut row| {
                    if let Some(common_name) =
                        self.localization.content(&format!("fa_{id}.common_name"))
                    {
                        row.col(|ui| {
                            ui.label(self.localization.content("common_name").unwrap_or_default());
                        });
                        row.col(|ui| {
                            ui.label(common_name);
                        });
                    }
                });
                body.row(height, |mut row| {
                    if let Some(systematic_name) = self
                        .localization
                        .content(&format!("fa_{id}.systematic_name"))
                    {
                        row.col(|ui| {
                            ui.label(
                                self.localization
                                    .content("systematic_name")
                                    .unwrap_or_default(),
                            );
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
