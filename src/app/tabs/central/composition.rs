use crate::{
    app::{
        context::{
            settings::composition::Stereospecificity,
            state::composition::{
                Data,
                Group::{self, Ec, Mass, Pec, Psc, Ptc, Sc, Sec, Ssc, Stc, Tc},
                Meta,
            },
            Context,
        },
        view::View,
    },
    properties::density::Hammond,
    r#const::{
        atoms::C,
        polymorphism::{alpha, beta::K_X},
        C3H2,
    },
    tree::{Branch, Hierarchized, Hierarchy, Item, Leaf, Node, Tree},
};
use egui::{
    collapsing_header::CollapsingState, text::LayoutJob, Align, CollapsingHeader,
    CollapsingResponse, Color32, Direction, FontId, Grid, Id, InnerResponse, Label, Layout,
    Response, RichText, ScrollArea, Sense, Separator, Slider, TextFormat, TextStyle, Ui, Vec2,
    Visuals, WidgetText,
};
use egui_ext::{ClickedLabel, CollapsingButton, CollapsingStateExt, TableBodyExt};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use molecule::{
    Saturable,
    Saturation::{self, Saturated, Unsaturated},
};
use std::{
    cmp::{max, min},
    fmt::Display,
};
use toml_edit::ser::to_string;
use uom::{
    fmt::DisplayStyle::*,
    si::{
        dynamic_viscosity::pascal_second,
        f64::ThermodynamicTemperature,
        mass_density::{gram_per_cubic_centimeter, kilogram_per_cubic_meter},
        molar_volume::cubic_meter_per_mole,
        thermodynamic_temperature::{degree_celsius, kelvin},
    },
};

const COLUMNS: usize = 2;

/// Central composition tab
pub(super) struct Composition<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Composition<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Composition<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        let p = context.settings.composition.precision;
        let height = ui.spacing().interact_size.y;
        context.compose(ui);
        ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            let branch = context
                .state
                .entry()
                .data
                .composed
                .composition(context.settings.composition.method);
            branch.ui(ui, context, &mut vec![], None);
        });

        // let mut close = false;
        // let mut path = vec![];
        // for Hierarchized(Hierarchy { level, index }, item) in context
        //     .state
        //     .entry()
        //     .data
        //     .composed
        //     .composition(context.settings.composition.method)
        //     .hierarchy()
        // {
        //     match item {
        //         Item::Meta(meta) => {
        //             while path.len() > level {
        //                 path.pop();
        //             }
        //             if let Some(group) = meta.group {
        //                 path.push(group.to_string());
        //             }
        //             // if close {
        //             //     continue;
        //             // }
        //             body.row(height, |mut row| {
        //                 row.col(|ui| {
        //                     let indent = ui.spacing().indent;
        //                     StripBuilder::new(ui)
        //                         .sizes(Size::exact(indent), level)
        //                         .size(Size::remainder())
        //                         .horizontal(|mut strip| {
        //                             for _ in 0..level {
        //                                 strip.cell(|ui| {
        //                                     ui.separator();
        //                                 });
        //                             }
        //                             strip.cell(|ui| {
        //                                 let text = meta
        //                                     .group
        //                                     .map_or_else(Default::default, |group| {
        //                                         format!("{group:.p$}")
        //                                     });
        //                                 let id = Id::new(&path);
        //                                 let InnerResponse { inner, response } =
        //                                     CollapsingButton::new(text)
        //                                         .id_source(id)
        //                                         .open(open)
        //                                         .show(ui);
        //                                 let filtered = meta.count.filtered;
        //                                 let unfiltered = meta.count.unfiltered;
        //                                 let count = unfiltered - filtered;
        //                                 response.on_hover_ui(|ui| {
        //                                     if let Some(Mass(mass)) = meta.group {
        //                                         ui.label(mass.to_string());
        //                                     }
        //                                     ui.label(format!("Count: {filtered} = {unfiltered} - {count}"));
        //                                  }
        //                             );
        //                                 close = !inner;
        //                             });
        //                         });
        //                 });
        //                 row.col(|ui| {
        //                     let value = &meta.value;
        //                     let mut rounded = value.rounded;
        //                     let mut unrounded = value.unrounded;
        //                     if context.settings.comparison.percent {
        //                         rounded *= 100.0;
        //                         unrounded *= 100.0;
        //                     }
        //                     ui.label(format!("{rounded:.p$}"))
        //                         .on_hover_text(format!("Unrounded: {unrounded}"));
        //                 });
        //             });
        //         }
        //         Item::Data(data) => {
        //             if close {
        //                 continue;
        //             }
        //             body.row(height, |mut row| {
        //                 row.col(|ui| {
        //                     let species = context.species(data.tag);
        //                     ui.label(format!("{species:#}")).on_hover_ui(|ui| {
        //                         ui.label(format!("STC: {}", context.r#type(data.tag)));
        //                         let ecn = context.ecn(data.tag);
        //                         ui.label(format!("ECN: {ecn:#} ({})", ecn.sum()));
        //                         let mass = context.mass(data.tag);
        //                         let adduct = context.settings.composition.adduct;
        //                         ui.label(format!(
        //                             "Mass: {:.p$} = [{:.p$} + {:.p$} + {:.p$}] + {adduct:.p$}",
        //                             C3H2 + mass.sum() + adduct.0, mass[0], mass[1], mass[2]
        //                         ));
        //                     })
        //                     .on_hover_ui(|ui| {
        //                         let t = ThermodynamicTemperature::new::<degree_celsius>(20.0);
        //                         ui.heading("Properties");
        //                         ui.label(format!("Density: {}", context.formula(data.tag).map(|counter| counter.density(t).into_format_args(gram_per_cubic_centimeter, Abbreviation))));
        //                         // ui.label(format!("Molar volume: {}", properties.molar_volume.into_format_args(cubic_meter_per_mole, Abbreviation)));
        //                         // ui.label(format!("Dynamic viscosity: {} ({})", properties.dynamic_viscosity.into_format_args(pascal_second, Abbreviation),
        //                         //     properties.dynamic_viscosity.into_format_args(pascal_second, Abbreviation)));
        //                     });
        //                     // .on_hover_ui(|ui| {
        //                     //     ui.heading("Thermodynamic properties");
        //                     //     let thermodynamic = context.thermodynamic(data.tag);
        //                     //     for properties in [&thermodynamic.alpha, &thermodynamic.beta_prime, &thermodynamic.beta] {
        //                     //         ui.separator();
        //                     //         let k0 = properties.melting_points.0.into_format_args(kelvin, Abbreviation);
        //                     //         let c0 = properties.melting_points.0.into_format_args(degree_celsius, Abbreviation);
        //                     //         let k1 = properties.melting_points.1.into_format_args(kelvin, Abbreviation);
        //                     //         let c1 = properties.melting_points.1.into_format_args(degree_celsius, Abbreviation);
        //                     //         ui.label(format!("ΔH (enthalpy): {}", properties.enthalpy_of_fusion));
        //                     //         ui.label(format!("ΔS (entropy): {}", properties.entropy_of_fusion));
        //                     //         ui.label(format!("T: {c0} ({k0}) / {c1} ({k1})"));
        //                     //     }
        //                     // });
        //                 });
        //                 row.col(|ui| {
        //                     let mut value = data.value;
        //                     if context.settings.composition.percent {
        //                         value *= 100.0;
        //                     }
        //                     ui.label(format!("{value:.p$}"))
        //                         .on_hover_text(format!("Unrounded: {value}"));
        //                 });
        //             });
        //         }
        //     }
        // }
        // // Footer
        // let meta = &context
        //     .state
        //     .entry()
        //     .data
        //     .composed
        //     .composition(context.settings.composition.method)
        //     .meta;
        // body.separate(height / 2.0, COLUMNS);
        // body.row(height, |mut row| {
        //     row.col(|ui| {
        //         let filtered = meta.count.filtered;
        //         let unfiltered = meta.count.unfiltered;
        //         let count = unfiltered - filtered;
        //         ui.label(filtered.to_string()).on_hover_ui(|ui| {
        //             ui.label(format!("{filtered} = {unfiltered} - {count}"));
        //         });
        //     });
        //     row.col(|ui| {
        //         let mut rounded = meta.value.rounded;
        //         let mut unrounded = meta.value.unrounded;
        //         if context.settings.composition.percent {
        //             rounded *= 100.0;
        //             unrounded *= 100.0;
        //         }
        //         ui.label(format!("{rounded:.p$}"))
        //             .on_hover_text(unrounded.to_string());
        //     });
        // });
        // body.separate(height / 2.0, COLUMNS);
    }
}

impl Branch<Meta, Data> {
    pub fn ui(
        &self,
        ui: &mut Ui,
        context: &Context,
        path: &mut Vec<Option<Group>>,
        mut open: Option<bool>,
    ) {
        path.push(self.meta.group);
        let p = context.settings.composition.precision;
        let mut job = LayoutJob::default();
        let default_color = if ui.visuals().dark_mode {
            Visuals::dark().text_color()
        } else {
            Visuals::light().text_color()
        };

        if let Some(group) = self.meta.group {
            let heading_font_id = TextStyle::Heading.resolve(ui.style());
            let small_font_id = TextStyle::Small.resolve(ui.style());
            match group {
                Ec(ecn) => {
                    job.append(
                        &ecn.to_string(),
                        0.0,
                        TextFormat {
                            color: default_color,
                            font_id: heading_font_id,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "ECNC",
                        1.0,
                        TextFormat {
                            color: default_color,
                            font_id: small_font_id,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                }
                Pec(ecn) => {
                    job.append(
                        &format!("{ecn:#}"),
                        0.0,
                        TextFormat {
                            color: default_color,
                            font_id: heading_font_id,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "PECNC",
                        1.0,
                        TextFormat {
                            color: default_color,
                            font_id: small_font_id,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                }
                Sec(ecn) => {
                    job.append(
                        &format!("{ecn:#}"),
                        0.0,
                        TextFormat {
                            color: default_color,
                            font_id: heading_font_id,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "SECNC",
                        1.0,
                        TextFormat {
                            color: default_color,
                            font_id: small_font_id,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                }
                Mass(mass) => {
                    job.append(
                        &mass.to_string(),
                        0.0,
                        TextFormat {
                            color: default_color,
                            font_id: heading_font_id,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "M",
                        1.0,
                        TextFormat {
                            color: default_color,
                            font_id: small_font_id,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                }
                Tc(r#type) => {
                    job.append(
                        &r#type.to_string(),
                        0.0,
                        TextFormat {
                            color: default_color,
                            font_id: heading_font_id,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "TC",
                        1.0,
                        TextFormat {
                            color: default_color,
                            font_id: small_font_id,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                }
                Ptc(r#type) => {
                    job.append(
                        &r#type.to_string(),
                        0.0,
                        TextFormat {
                            color: default_color,
                            font_id: heading_font_id,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "PTC",
                        1.0,
                        TextFormat {
                            color: default_color,
                            font_id: small_font_id,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                }
                Stc(r#type) => {
                    job.append(
                        &r#type.to_string(),
                        0.0,
                        TextFormat {
                            color: default_color,
                            font_id: heading_font_id,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "STC",
                        1.0,
                        TextFormat {
                            color: default_color,
                            font_id: small_font_id,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                }
                Sc(tag) => {
                    job.append(
                        &format!("{:#}", context.species(tag)),
                        0.0,
                        TextFormat {
                            color: default_color,
                            font_id: heading_font_id,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "SC",
                        1.0,
                        TextFormat {
                            color: default_color,
                            font_id: small_font_id,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                }
                Psc(tag) => {
                    job.append(
                        &format!("{:#}", context.species(tag)),
                        0.0,
                        TextFormat {
                            color: default_color,
                            font_id: heading_font_id,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "PSC",
                        1.0,
                        TextFormat {
                            color: default_color,
                            font_id: small_font_id,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                }
                Ssc(tag) => {
                    job.append(
                        &format!("{:#}", context.species(tag)),
                        0.0,
                        TextFormat {
                            color: default_color,
                            font_id: heading_font_id,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "SSC",
                        1.0,
                        TextFormat {
                            color: default_color,
                            font_id: small_font_id,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                }
            }
        }
        let collapsing_state =
            CollapsingState::load_with_default_open(ui.ctx(), Id::new(&path), path.len() < 1)
                .open(open.take());
        let opened = collapsing_state.is_open();
        let branches = !self
            .children
            .iter()
            .any(|node| matches!(node, Node::Leaf(_)));
        collapsing_state
            .show_header(ui, |ui| {
                // let available = ui.available_size_before_wrap().x;
                // let width = ui.spacing().interact_size.x;
                let response = ui
                    .horizontal(|ui| {
                        let mut response = ui.label(job);
                        let value = &self.meta.value;
                        let mut rounded = value.rounded;
                        let mut unrounded = value.unrounded;
                        if context.settings.comparison.percent {
                            rounded *= 100.0;
                            unrounded *= 100.0;
                        }
                        response |= ui
                            .label(format!("{rounded:.p$}"))
                            .on_hover_text(format!("Unrounded: {unrounded}"));
                        response.sense |= Sense::click();
                        response
                    })
                    .inner;
                if opened && branches {
                    response.context_menu(|ui| {
                        if ui.button("Show all").clicked() {
                            open = Some(true);
                            ui.close_menu();
                        }
                        if ui.button("Hide all").clicked() {
                            open = Some(false);
                            ui.close_menu();
                        }
                    });
                }
            })
            .body(|ui| {
                for child in &self.children {
                    match child {
                        Node::Branch(branch) => branch.ui(ui, context, path, open),
                        Node::Leaf(leaf) => leaf.ui(ui, context),
                    }
                }
            });
        path.pop();
    }
}

impl Leaf<Data> {
    pub fn ui(&self, ui: &mut Ui, context: &Context) {
        let p = context.settings.composition.precision;
        let height = ui.spacing().interact_size.y;
        // TableBuilder::new(ui)
        //     .auto_shrink(false)
        //     .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
        //     .columns(Column::auto(), 2)
        //     .striped(true)
        //     .header(height, |mut row| {
        //         row.col(|ui| {
        //             ui.label("");
        //         });
        //     })
        //     .body(|mut body| {
        //         body.row(height, |mut row| {
        //             row.col(|ui| {
        //                 let tag = self.data.tag;
        //                 let species = context.species(tag);
        //                 ui.label(format!("{species:#}"))
        //                     .on_hover_ui(|ui| {
        //                         ui.label(format!("STC: {}", context.r#type(tag)));
        //                         let ecn = context.ecn(tag);
        //                         ui.label(format!("ECN: {ecn:#} ({})", ecn.sum()));
        //                         let mass = context.mass(tag);
        //                         let adduct = context.settings.composition.adduct;
        //                         ui.label(format!(
        //                             "Mass: {:.p$} = [{:.p$} + {:.p$} + {:.p$}] + {adduct:.p$}",
        //                             C3H2 + mass.sum() + adduct.0,
        //                             mass[0],
        //                             mass[1],
        //                             mass[2]
        //                         ));
        //                     })
        //                     .on_hover_ui(|ui| {
        //                         let t = ThermodynamicTemperature::new::<degree_celsius>(20.0);
        //                         ui.heading("Properties");
        //                         ui.label(format!(
        //                             "Density: {}",
        //                             context.formula(tag).map(|counter| counter
        //                                 .density(t)
        //                                 .into_format_args(gram_per_cubic_centimeter, Abbreviation))
        //                         ));
        //                         // ui.label(format!("Molar volume: {}", properties.molar_volume.into_format_args(cubic_meter_per_mole, Abbreviation)));
        //                         // ui.label(format!("Dynamic viscosity: {} ({})", properties.dynamic_viscosity.into_format_args(pascal_second, Abbreviation),
        //                         //     properties.dynamic_viscosity.into_format_args(pascal_second, Abbreviation)));
        //                     });
        //             });
        //             row.col(|ui| {
        //                 let mut value = self.data.value;
        //                 if context.settings.composition.percent {
        //                     value *= 100.0;
        //                 }
        //                 ui.label(format!("{value:.p$}"))
        //                     .on_hover_text(format!("Unrounded: {value}"));
        //             });
        //         })
        //     })

        // ui.columns(2, |ui| {
        //     let tag = self.data.tag;
        //     let species = context.species(tag);
        //     ui[0]
        //         .label(format!("{species:#}"))
        //         .on_hover_ui(|ui| {
        //             ui.label(format!("STC: {}", context.r#type(tag)));
        //             let ecn = context.ecn(tag);
        //             ui.label(format!("ECN: {ecn:#} ({})", ecn.sum()));
        //             let mass = context.mass(tag);
        //             let adduct = context.settings.composition.adduct;
        //             ui.label(format!(
        //                 "Mass: {:.p$} = [{:.p$} + {:.p$} + {:.p$}] + {adduct:.p$}",
        //                 C3H2 + mass.sum() + adduct.0,
        //                 mass[0],
        //                 mass[1],
        //                 mass[2]
        //             ));
        //         })
        //         .on_hover_ui(|ui| {
        //             let t = ThermodynamicTemperature::new::<degree_celsius>(20.0);
        //             ui.heading("Properties");
        //             ui.label(format!(
        //                 "Density: {}",
        //                 context.formula(tag).map(|counter| counter
        //                     .density(t)
        //                     .into_format_args(gram_per_cubic_centimeter, Abbreviation))
        //             ));
        //             // ui.label(format!("Molar volume: {}", properties.molar_volume.into_format_args(cubic_meter_per_mole, Abbreviation)));
        //             // ui.label(format!("Dynamic viscosity: {} ({})", properties.dynamic_viscosity.into_format_args(pascal_second, Abbreviation),
        //             //     properties.dynamic_viscosity.into_format_args(pascal_second, Abbreviation)));
        //         });
        //     let mut value = self.data.value;
        //     if context.settings.composition.percent {
        //         value *= 100.0;
        //     }
        //     ui[1]
        //         .label(format!("{value:.p$}"))
        //         .on_hover_text(format!("Unrounded: {value}"));
        // });

        ui.horizontal(|ui| {
            let tag = self.data.tag;
            let species = context.species(tag);
            let default_color = if ui.visuals().dark_mode {
                Visuals::dark().text_color()
            } else {
                Visuals::light().text_color()
            };
            let small_font_id = TextStyle::Small.resolve(ui.style());
            let mut job = LayoutJob::default();
            job.append(
                &format!("{species:#}"),
                ui.spacing().indent,
                TextFormat {
                    color: default_color,
                    ..Default::default()
                },
            );
            job.append(
                context.settings.composition.tree.leafs.text(),
                1.0,
                TextFormat {
                    color: default_color,
                    font_id: small_font_id,
                    valign: Align::BOTTOM,
                    ..Default::default()
                },
            );
            ui.label(job)
                .on_hover_ui(|ui| {
                    ui.label(format!("STC: {}", context.r#type(tag)));
                    let ecn = context.ecn(tag);
                    ui.label(format!("ECN: {ecn:#} ({})", ecn.sum()));
                    let mass = context.mass(tag);
                    let adduct = context.settings.composition.adduct;
                    ui.label(format!(
                        "Mass: {:.p$} = [{:.p$} + {:.p$} + {:.p$}] + {adduct:.p$}",
                        C3H2 + mass.sum() + adduct.0,
                        mass[0],
                        mass[1],
                        mass[2]
                    ));
                })
                .on_hover_ui(|ui| {
                    let t = ThermodynamicTemperature::new::<degree_celsius>(20.0);
                    ui.heading("Properties");
                    ui.label(format!(
                        "Density: {}",
                        context.formula(tag).map(|counter| counter
                            .density(t)
                            .into_format_args(gram_per_cubic_centimeter, Abbreviation))
                    ));
                    // ui.label(format!("Molar volume: {}", properties.molar_volume.into_format_args(cubic_meter_per_mole, Abbreviation)));
                    // ui.label(format!("Dynamic viscosity: {} ({})", properties.dynamic_viscosity.into_format_args(pascal_second, Abbreviation),
                    //     properties.dynamic_viscosity.into_format_args(pascal_second, Abbreviation)));
                });
            let mut value = self.data.value;
            if context.settings.composition.percent {
                value *= 100.0;
            }
            ui.label(format!("{value:.p$}"))
                .on_hover_text(format!("Unrounded: {value}"));
        });
    }
}
