use crate::{
    app::{
        context::{
            settings::composition::Stereospecificity,
            state::composition::{
                Data,
                Group::{Ecn, Mass, Psc, Ptc, Sc, Ssc, Stc, Tc},
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
    collapsing_header::CollapsingState, Align, CollapsingHeader, CollapsingResponse, Direction, Id,
    InnerResponse, Label, Layout, Response, Ui, WidgetText,
};
use egui_ext::{ClickedLabel, CollapsingButton, TableBodyExt};
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
        context.compose(ui);
        let height = ui.spacing().interact_size.y;
        let mut open = None;
        TableBuilder::new(ui)
            .auto_shrink(false)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), COLUMNS)
            .max_scroll_height(f32::NAN)
            .resizable(context.settings.composition.resizable)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.clicked_heading("TAG")
                        .on_hover_text("Triacylglycerol")
                        .context_menu(|ui| {
                            if !context.settings.composition.groups.is_empty() {
                                if ui
                                    .button("Expand")
                                    .on_hover_text("Expand all groups")
                                    .clicked()
                                {
                                    open = Some(true);
                                    ui.close_menu();
                                }
                                if ui
                                    .button("Collapse")
                                    .on_hover_text("Collapse all groups")
                                    .clicked()
                                {
                                    open = Some(false);
                                    ui.close_menu();
                                }
                                ui.separator();
                            }
                            ui.menu_button("Copy", |ui| {
                                if ui.button("All").clicked() {
                                    ui.close_menu();
                                }
                                if ui.button("Groups").clicked() {
                                    // ui.output_mut(|output| {
                                    //     output.copied_text = context
                                    //         .state
                                    //         .entry()
                                    //         .data
                                    //         .composed
                                    //         .filtered
                                    //         .keys()
                                    //         .copied()
                                    //         .filter_map(identity)
                                    //         .join("\n");
                                    // });
                                    ui.close_menu();
                                }
                                if ui.button("Items").clicked() {
                                    // ui.output_mut(|output| {
                                    //     output.copied_text = context
                                    //         .state
                                    //         .entry()
                                    //         .data
                                    //         .composed
                                    //         .filtered
                                    //         .values()
                                    //         .flat_map(|values| {
                                    //             values.keys().map(|&tag| context.species(tag))
                                    //         })
                                    //         .join("\n");
                                    // });
                                    ui.close_menu();
                                }
                            });
                        });
                });
                row.col(|ui| {
                    ui.clicked_heading("Value").context_menu(|ui| {
                        ui.menu_button("Copy", |ui| {
                            if ui.button("Values").clicked() {
                                ui.close_menu();
                            }
                            if ui.button("Group values").clicked() {
                                // ui.output_mut(|output| {
                                //     output.copied_text = context
                                //         .state
                                //         .entry()
                                //         .data
                                //         .composed
                                //         .filtered
                                //         .values()
                                //         .map(|values| values.values().sum())
                                //         .format_with("\n", |mut value: f64, f| {
                                //             if context.settings.composition.percent {
                                //                 value *= 100.0;
                                //             }
                                //             f(&format_args!("{value:.p$}"))
                                //         })
                                //         .to_string();
                                // });
                                ui.close_menu();
                            };
                            if ui.button("Species values").clicked() {
                                // ui.output_mut(|output| {
                                //     output.copied_text = context
                                //         .state
                                //         .entry()
                                //         .data
                                //         .composed
                                //         .filtered
                                //         .values()
                                //         .flat_map(IndexMap::values)
                                //         .format_with("\n", |&(mut value), f| {
                                //             if context.settings.composition.percent {
                                //                 value *= 100.0;
                                //             }
                                //             f(&format_args!("{value:.p$}"))
                                //         })
                                //         .to_string();
                                // });
                                ui.close_menu();
                            }
                        });
                    });
                });
            })
            .body(|mut body| {
                context
                    .state
                    .entry()
                    .data
                    .composed
                    .composition(context.settings.composition.method)
                    .ui(body.ui_mut(), context, 0);

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
            });
    }
}

impl Branch<Meta, Data> {
    pub fn ui(&self, ui: &mut Ui, context: &Context, depth: usize) {
        let p = context.settings.composition.precision;
        let group = &self
            .meta
            .group
            .map(|group| match group {
                Ecn(ecn) => ecn.to_string(),
                Mass(mass) => mass.to_string(),
                Tc(r#type) => r#type.to_string(),
                Ptc(r#type) => r#type.to_string(),
                Stc(r#type) => r#type.to_string(),
                Sc(tag) => format!("{:#}", context.species(tag)),
                Psc(tag) => format!("{:#}", context.species(tag)),
                Ssc(tag) => format!("{:#}", context.species(tag)),
            })
            .unwrap_or_default();
        CollapsingState::load_with_default_open(ui.ctx(), Id::new(group).with(depth), depth < 1)
            .show_header(ui, |ui| {
                // ui.add_sized(max_size, widget)
                // ui.horizontal(|ui| {
                //     // ui.add_sized([255.0, 0.0], |ui| {
                //     //     ui.with_layout(Layout::left_to_right(Align::Center), Label::new(group))
                //     //         .inner
                //     // });
                //     ui.label(group);
                //     ui.separator();
                //     let value = &self.meta.value;
                //     let mut rounded = value.rounded;
                //     let mut unrounded = value.unrounded;
                //     if context.settings.comparison.percent {
                //         rounded *= 100.0;
                //         unrounded *= 100.0;
                //     }
                //     ui.label(format!("{rounded:.p$}"))
                //         .on_hover_text(format!("Unrounded: {unrounded}"));
                // });
                let width = ui.spacing().interact_size.x;
                let available = ui.available_size_before_wrap().x;
                StripBuilder::new(ui)
                    // .size(Size::initial(available - 500.0).at_least(width))
                    .sizes(Size::remainder().at_least(width), 2)
                    .horizontal(|mut strip| {
                        strip.cell(move |ui| {
                            ui.label(group);
                        });
                        strip.cell(|ui| {
                            let value = &self.meta.value;
                            let mut rounded = value.rounded;
                            let mut unrounded = value.unrounded;
                            if context.settings.comparison.percent {
                                rounded *= 100.0;
                                unrounded *= 100.0;
                            }
                            ui.label(format!("{rounded:.p$}"))
                                .on_hover_text(format!("Unrounded: {unrounded}"));
                        });
                    });
            })
            .body(|ui| {
                for child in &self.children {
                    match child {
                        Node::Branch(branch) => branch.ui(ui, context, depth + 1),
                        Node::Leaf(leaf) => leaf.ui(ui, context),
                    }
                }
            });
        // CollapsingHeader::new(
        //     self.meta
        //         .group
        //         .map(|group| WidgetText::from(group.to_string()))
        //         .unwrap_or_default(),
        // )
        // .default_open(depth < 1)
        // .show(ui, |ui| {
        //     for child in &self.children {
        //         match child {
        //             Node::Branch(branch) => branch.ui(ui, context, depth + 1),
        //             Node::Leaf(leaf) => leaf.ui(ui, context),
        //         }
        //     }
        // });
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

        ui.columns(2, |ui| {
            let tag = self.data.tag;
            let species = context.species(tag);
            ui[0]
                .label(format!("{species:#}"))
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
            ui[1]
                .label(format!("{value:.p$}"))
                .on_hover_text(format!("Unrounded: {value}"));
        });

        // ui.horizontal(|ui| {
        //     let tag = self.data.tag;
        //     let species = context.species(tag);
        //     ui.label(format!("{species:#}"))
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
        //     ui.label(format!("{value:.p$}"))
        //         .on_hover_text(format!("Unrounded: {value}"));
        // });
    }
}
