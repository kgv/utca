use crate::{
    app::{
        context::{settings::configuration::U, Context},
        view::View,
    },
    properties::{density::Hammond, viscosity::Rabelo},
    r#const::CH2,
    utils::UiExt,
};
use egui::{Align, Direction, DragValue, Layout, RichText, Ui};
use egui_ext::{ClickedLabel, TableBodyExt, TableRowExt};
use egui_extras::{Column, TableBuilder};
use molecule::{
    atom::{isotopes::*, Isotope},
    Saturable,
};
use std::{num::NonZeroUsize, sync::LazyLock};
use toml_edit::DocumentMut;
use uom::{
    fmt::DisplayStyle::Abbreviation,
    si::{
        dynamic_viscosity::{centipoise, millipascal_second, pascal_second},
        f64::ThermodynamicTemperature,
        mass_density::gram_per_cubic_centimeter,
        molar_volume::cubic_centimeter_per_mole,
        thermodynamic_temperature::degree_celsius,
    },
};

const H: Isotope = Isotope::H(H::One);
const C: Isotope = Isotope::C(C::Twelve);

static FATTY_ACIDS: LazyLock<DocumentMut> = LazyLock::new(|| {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/fatty_acids.toml"))
        .parse::<DocumentMut>()
        .unwrap()
});

/// Central configuration tab
pub(super) struct Configuration<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Configuration<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Configuration<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        let p = context.settings.configuration.precision;
        let mut columns = 4;
        if context.settings.configuration.editable {
            columns += 2;
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("Name:");
            ui.clicked_label(&context.state.entry_mut().meta.name)
                .context_menu(|ui| {
                    ui.text_edit_singleline(&mut context.state.entry_mut().meta.name);
                });
        });
        let mut builder = TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight));
        if context.settings.configuration.editable {
            builder = builder.column(Column::exact(width));
        }
        builder = builder
            .column(Column::auto_with_initial_suggestion(width))
            .columns(Column::auto(), 3);
        if context.settings.configuration.editable {
            builder = builder.column(Column::exact(width));
        }
        builder
            .auto_shrink(false)
            .resizable(context.settings.configuration.resizable)
            .striped(true)
            .header(height, |mut row| {
                if context.settings.configuration.editable {
                    row.col(|_| {});
                }
                row.col(|ui| {
                    ui.heading("FA").on_hover_text("Fatty acid label");
                });
                row.col(|ui| {
                    ui.heading("1,2,3-TAG");
                });
                row.col(|ui| {
                    ui.heading("1,2/2,3-DAG");
                });
                row.col(|ui| {
                    ui.heading("2-MAG");
                });
            })
            .body(|mut body| {
                let mut up = None;
                // Content
                for index in 0..context.state.entry().len() {
                    let mut keep = true;
                    body.row(height, |mut row| {
                        // Drag and drop
                        if context.settings.configuration.editable {
                            row.col(|ui| {
                                if ui.button("⏶").clicked() {
                                    up = Some(index);
                                }
                            });
                        }
                        // Fatty acid
                        // row.col(|ui| {
                        //     ui.text_edit_singleline(
                        //         &mut context.state.entry_mut().meta.labels[index],
                        //     );
                        // });
                        // // C
                        // row.col(|ui| {
                        //     let formula = &mut context.state.entry_mut().meta.formulas[index];
                        //     let c = formula.count(C);
                        //     ComboBox::from_id_source(Id::new("c").with(index))
                        //         .selected_text(c.to_string())
                        //         .width(ui.available_width())
                        //         .show_ui(ui, |ui| {
                        //             for variant in context.settings.configuration.c {
                        //                 if ui
                        //                     .selectable_label(c == variant, variant.to_string())
                        //                     .clicked()
                        //                 {
                        //                     *formula = fatty_acid!(variant);
                        //                     ui.ctx().request_repaint();
                        //                 }
                        //             }
                        //         })
                        //         .response
                        //         .on_hover_ui(|ui| {
                        //             ui.label(formula.to_string());
                        //             ui.label(format!("Mass: {}", formula.weight()));
                        //         });
                        // });
                        // // U
                        // row.col(|ui| {
                        //     let formula = &mut context.state.entry_mut().meta.formulas[index];
                        //     let c = formula.count(C);
                        //     let u = formula.unsaturated();
                        //     ComboBox::from_id_source(Id::new("u").with(index))
                        //         .selected_text(u.to_string())
                        //         .width(ui.available_width())
                        //         .show_ui(ui, |ui| {
                        //             for u in 0..=U::max(c).min(context.settings.configuration.u) {
                        //                 ui.selectable_value(
                        //                     formula,
                        //                     fatty_acid!(c, u),
                        //                     u.to_string(),
                        //                 );
                        //             }
                        //         })
                        //         .response
                        //         .on_hover_ui(|ui| {
                        //             ui.label(formula.to_string());
                        //             ui.label(format!("Mass: {}", formula.weight()));
                        //         });
                        // });
                        // row.left_align_col(|ui| {
                        //     let entry = context.state.entry();
                        //     let formula = &entry.meta.formulas[index];
                        //     let c = formula.count(C);
                        //     let u = formula.unsaturated();
                        //     let mut response = ui
                        //         .clicked_heading(entry.meta.labels[index].to_string())
                        //         .on_hover_ui(|ui| {
                        //             ui.heading("Fatty acid");
                        //             let formula = &context.state.entry().meta.formulas[index];
                        //             ui.label(format!("Formula: {}", formula));
                        //             ui.label(format!("Mass: {}", formula.weight()));
                        //             ui.label(format!(
                        //                 "Methyl ester mass: {}",
                        //                 formula.weight() + CH2,
                        //             ));
                        //         });
                        //     ui.allocate_ui_at_rect(response.rect, |ui| {
                        //         ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        //             ui.label(RichText::new(format!("{c}:{u}")).small());
                        //         });
                        //     });
                        //     if context.settings.configuration.properties {
                        //         response = response.on_hover_ui(|ui| {
                        //             ui.heading("Properties");
                        //             let formula = &context.state.entry().meta.formulas[index];
                        //             let t = ThermodynamicTemperature::new::<degree_celsius>(40.0);
                        //             ui.label(format!(
                        //                 "Molar volume: {}",
                        //                 formula.molar_volume(t).into_format_args(
                        //                     cubic_centimeter_per_mole,
                        //                     Abbreviation
                        //                 ),
                        //             ));
                        //             ui.label(format!(
                        //                 "Density: {}",
                        //                 formula.density(t).into_format_args(
                        //                     gram_per_cubic_centimeter,
                        //                     Abbreviation
                        //                 ),
                        //             ));
                        //             ui.label(format!(
                        //                 "Dynamic viscosity: {}",
                        //                 formula
                        //                     .dynamic_viscosity(t)
                        //                     .into_format_args(millipascal_second, Abbreviation),
                        //             ));
                        //         });
                        //     }
                        //     if context.settings.configuration.names {
                        //         if let Some(item) = FATTY_ACIDS.get(&format!("{c}:{u}")) {
                        //             if let Some(array_of_tables) = item.as_array_of_tables() {
                        //                 response = response.on_hover_ui(|ui| {
                        //                     TableBuilder::new(ui)
                        //                         .striped(true)
                        //                         .column(Column::exact(3.0 * width))
                        //                         .column(Column::exact(6.0 * width))
                        //                         .column(Column::remainder())
                        //                         .header(height, |mut header| {
                        //                             header.col(|ui| {
                        //                                 ui.heading("Abbreviation");
                        //                             });
                        //                             header.col(|ui| {
                        //                                 ui.heading("Common name");
                        //                             });
                        //                             header.col(|ui| {
                        //                                 ui.heading("Systematic name");
                        //                             });
                        //                         })
                        //                         .body(|mut body| {
                        //                             for table in array_of_tables {
                        //                                 body.row(height, |mut row| {
                        //                                     if let Some(abbreviation) =
                        //                                         table.get("abbreviation")
                        //                                     {
                        //                                         row.col(|ui| {
                        //                                             ui.label(
                        //                                                 abbreviation.to_string(),
                        //                                             );
                        //                                         });
                        //                                     } else {
                        //                                         row.col(|_| {});
                        //                                     }
                        //                                     if let Some(common_name) =
                        //                                         table.get("common_name")
                        //                                     {
                        //                                         row.col(|ui| {
                        //                                             ui.label(
                        //                                                 common_name.to_string(),
                        //                                             );
                        //                                         });
                        //                                     } else {
                        //                                         row.col(|_| {});
                        //                                     }
                        //                                     if let Some(systematic_name) =
                        //                                         table.get("systematic_name")
                        //                                     {
                        //                                         row.col(|ui| {
                        //                                             ui.label(
                        //                                                 systematic_name.to_string(),
                        //                                             );
                        //                                         });
                        //                                     } else {
                        //                                         row.col(|_| {});
                        //                                     }
                        //                                 });
                        //                             }
                        //                         });
                        //                 });
                        //             }
                        //         }
                        //     }
                        //     response.context_menu(|ui| {
                        //         ui.text_edit_singleline(
                        //             &mut context.state.entry_mut().meta.labels[index],
                        //         );
                        //         let formula = &mut context.state.entry_mut().meta.formulas[index];
                        //         let mut c = formula.count(C);
                        //         let mut u = formula.unsaturated();
                        //         ui.horizontal(|ui| {
                        //             // C
                        //             ui.label("C:");
                        //             if ui
                        //                 .add(DragValue::new(&mut c).clamp_range(
                        //                     context.settings.configuration.c.start
                        //                         ..=context.settings.configuration.c.end,
                        //                 ))
                        //                 .changed()
                        //             {
                        //                 let formula =
                        //                     &mut context.state.entry_mut().meta.formulas[index];
                        //                 if let Some(c) = NonZeroUsize::new(c) {
                        //                     formula.insert(C, c);
                        //                     let h = 2 * (c.get() - u);
                        //                     if let Some(h) = NonZeroUsize::new(h) {
                        //                         formula.insert(H, h);
                        //                     }
                        //                 }
                        //             }
                        //             // U
                        //             ui.label("U:");
                        //             if ui
                        //                 .add(DragValue::new(&mut u).clamp_range(
                        //                     0..=U::max(c).min(context.settings.configuration.u),
                        //                 ))
                        //                 .changed()
                        //             {
                        //                 let formula =
                        //                     &mut context.state.entry_mut().meta.formulas[index];
                        //                 if let Some(h) = NonZeroUsize::new(2 * (c - u)) {
                        //                     formula.insert(H, h);
                        //                 }
                        //             }
                        //         });
                        //         ui.horizontal(|ui| {
                        //             ui.label("Correction factor:");
                        //             ui.add(
                        //                 DragValue::new(
                        //                     &mut context.settings.configuration.correction_factor,
                        //                 )
                        //                 .clamp_range(f64::MIN..=f64::MAX)
                        //                 .speed(0.01),
                        //             )
                        //             .on_hover_text(
                        //                 context
                        //                     .settings
                        //                     .configuration
                        //                     .correction_factor
                        //                     .to_string(),
                        //             );
                        //         });
                        //     });
                        // });
                        row.left_align_col(|ui| {
                            let entry = context.state.entry();
                            let formula = &entry.meta.formulas[index];
                            let c = formula.count(C);
                            let u = formula.unsaturated();
                            let title = ui
                                .subscripted_widget(&entry.meta.labels[index], &format!("{c}:{u}"));
                            let mut response = ui
                                .menu_button(title, |ui| {
                                    ui.text_edit_singleline(
                                        &mut context.state.entry_mut().meta.labels[index],
                                    );
                                    let formula =
                                        &mut context.state.entry_mut().meta.formulas[index];
                                    let mut c = formula.count(C);
                                    let mut u = formula.unsaturated();
                                    ui.horizontal(|ui| {
                                        // C
                                        ui.label("C:");
                                        if ui
                                            .add(DragValue::new(&mut c).clamp_range(
                                                context.settings.configuration.c.start
                                                    ..=context.settings.configuration.c.end,
                                            ))
                                            .changed()
                                        {
                                            let formula =
                                                &mut context.state.entry_mut().meta.formulas[index];
                                            if let Some(c) = NonZeroUsize::new(c) {
                                                formula.insert(C, c);
                                                let h = 2 * (c.get() - u);
                                                if let Some(h) = NonZeroUsize::new(h) {
                                                    formula.insert(H, h);
                                                }
                                            }
                                        }
                                        // U
                                        ui.label("U:");
                                        if ui
                                            .add(DragValue::new(&mut u).clamp_range(
                                                0..=U::max(c).min(context.settings.configuration.u),
                                            ))
                                            .changed()
                                        {
                                            let formula =
                                                &mut context.state.entry_mut().meta.formulas[index];
                                            if let Some(h) = NonZeroUsize::new(2 * (c - u)) {
                                                formula.insert(H, h);
                                            }
                                        }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("Correction factor:");
                                        ui.add(
                                            DragValue::new(
                                                &mut context
                                                    .settings
                                                    .configuration
                                                    .correction_factor,
                                            )
                                            .clamp_range(f64::MIN..=f64::MAX)
                                            .speed(0.01),
                                        )
                                        .on_hover_text(
                                            context
                                                .settings
                                                .configuration
                                                .correction_factor
                                                .to_string(),
                                        );
                                    });
                                })
                                .response
                                .on_hover_ui(|ui| {
                                    ui.heading("Fatty acid");
                                    let formula = &context.state.entry().meta.formulas[index];
                                    ui.label(format!("Formula: {}", formula));
                                    ui.label(format!("Mass: {}", formula.weight()));
                                    ui.label(format!(
                                        "Methyl ester mass: {}",
                                        formula.weight() + CH2,
                                    ));
                                });
                            // ui.allocate_ui_at_rect(response.rect, |ui| {
                            //     ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            //         ui.label(RichText::new(format!("{c}:{u}")).small());
                            //     });
                            // });
                            if context.settings.configuration.properties {
                                response = response.on_hover_ui(|ui| {
                                    ui.heading("Properties");
                                    let formula = &context.state.entry().meta.formulas[index];
                                    let t = ThermodynamicTemperature::new::<degree_celsius>(40.0);
                                    ui.label(format!(
                                        "Molar volume: {}",
                                        formula.molar_volume(t).into_format_args(
                                            cubic_centimeter_per_mole,
                                            Abbreviation
                                        ),
                                    ));
                                    ui.label(format!(
                                        "Density: {}",
                                        formula.density(t).into_format_args(
                                            gram_per_cubic_centimeter,
                                            Abbreviation
                                        ),
                                    ));
                                    ui.label(format!(
                                        "Dynamic viscosity: {}",
                                        formula
                                            .dynamic_viscosity(t)
                                            .into_format_args(millipascal_second, Abbreviation),
                                    ));
                                });
                            }
                            if context.settings.configuration.names {
                                if let Some(item) = FATTY_ACIDS.get(&format!("{c}:{u}")) {
                                    if let Some(array_of_tables) = item.as_array_of_tables() {
                                        response = response.on_hover_ui(|ui| {
                                            TableBuilder::new(ui)
                                                .striped(true)
                                                .column(Column::exact(3.0 * width))
                                                .column(Column::exact(6.0 * width))
                                                .column(Column::remainder())
                                                .header(height, |mut header| {
                                                    header.col(|ui| {
                                                        ui.heading("Abbreviation");
                                                    });
                                                    header.col(|ui| {
                                                        ui.heading("Common name");
                                                    });
                                                    header.col(|ui| {
                                                        ui.heading("Systematic name");
                                                    });
                                                })
                                                .body(|mut body| {
                                                    for table in array_of_tables {
                                                        body.row(height, |mut row| {
                                                            if let Some(abbreviation) =
                                                                table.get("abbreviation")
                                                            {
                                                                row.col(|ui| {
                                                                    ui.label(
                                                                        abbreviation.to_string(),
                                                                    );
                                                                });
                                                            } else {
                                                                row.col(|_| {});
                                                            }
                                                            if let Some(common_name) =
                                                                table.get("common_name")
                                                            {
                                                                row.col(|ui| {
                                                                    ui.label(
                                                                        common_name.to_string(),
                                                                    );
                                                                });
                                                            } else {
                                                                row.col(|_| {});
                                                            }
                                                            if let Some(systematic_name) =
                                                                table.get("systematic_name")
                                                            {
                                                                row.col(|ui| {
                                                                    ui.label(
                                                                        systematic_name.to_string(),
                                                                    );
                                                                });
                                                            } else {
                                                                row.col(|_| {});
                                                            }
                                                        });
                                                    }
                                                });
                                        });
                                    }
                                }
                            }
                        });
                        let data = &mut context.state.entry_mut().data.configured[index];
                        // Tag123
                        row.right_align_col(|ui| {
                            ui.add(
                                DragValue::new(&mut data.tag123)
                                    .clamp_range(0.0..=f64::MAX)
                                    .custom_formatter(|tag123, _| format!("{tag123:.p$}")),
                            )
                            .on_hover_text(data.tag123.to_string());
                        });
                        // Dag1223
                        row.right_align_col(|ui| {
                            ui.add(
                                DragValue::new(&mut data.dag1223)
                                    .clamp_range(0.0..=f64::MAX)
                                    .custom_formatter(|dag1223, _| format!("{dag1223:.p$}")),
                            )
                            .on_hover_text(data.dag1223.to_string());
                        });
                        // Mag2
                        row.right_align_col(|ui| {
                            ui.add(
                                DragValue::new(&mut data.mag2)
                                    .clamp_range(0.0..=f64::MAX)
                                    .custom_formatter(|mag2, _| format!("{mag2:.p$}")),
                            )
                            .on_hover_text(data.mag2.to_string());
                        });
                        // Delete row
                        if context.settings.configuration.editable {
                            row.col(|ui| {
                                keep = !ui
                                    .button(RichText::new("-").monospace())
                                    .on_hover_text("Delete row")
                                    .clicked();
                            });
                        }
                    });
                    if !keep {
                        context.state.entry_mut().del(index);
                        break;
                    }
                }
                if let Some(index) = up {
                    context
                        .state
                        .entry_mut()
                        .swap(index, index.saturating_sub(1));
                }
                // Footer
                body.separate(height / 2.0, columns);
                body.row(height, |mut row| {
                    if context.settings.configuration.editable {
                        row.col(|_| {});
                    }
                    row.cols(1, |_| {});
                    // ∑
                    row.right_align_col(|ui| {
                        let sum: f64 = context.state.entry().data.configured.tags123().sum();
                        ui.label(format!("{sum:.p$}"))
                            .on_hover_text(sum.to_string());
                    });
                    row.right_align_col(|ui| {
                        let sum: f64 = context.state.entry().data.configured.dags1223().sum();
                        ui.label(format!("{sum:.p$}"))
                            .on_hover_text(sum.to_string());
                    });
                    row.right_align_col(|ui| {
                        let sum: f64 = context.state.entry().data.configured.mags2().sum();
                        ui.label(format!("{sum:.p$}"))
                            .on_hover_text(sum.to_string());
                    });
                    // Add row
                    if context.settings.configuration.editable {
                        row.col(|ui| {
                            if ui
                                .button(RichText::new("+").monospace())
                                .on_hover_text("Add row")
                                .clicked()
                            {
                                context.state.entry_mut().add();
                            }
                        });
                    }
                });
            });
    }
}

// /// Fatty acids TOML file
// trait FattyAcidsToml {
//     fn abbreviation(&self, key: &str) -> Option<String>;

//     fn common_name(&self, key: &str) -> Option<String>;

//     fn systematic_name(&self, key: &str) -> Option<String>;
// }

// impl FattyAcidsToml for Document {
//     fn abbreviation(&self, key: &str) -> Option<String> {
//         self.as_array_of_tables()? {

//         }
//             .and_then(|array_of_tables| array_of_tables)
//     }

//     fn common_name(&self, key: &str) -> Option<String> {
//         todo!()
//     }

//     fn systematic_name(&self, key: &str) -> Option<String> {
//         todo!()
//     }
// }
