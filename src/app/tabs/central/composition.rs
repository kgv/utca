use crate::{
    app::{
        context::{
            state::composition::{
                Count, Data,
                Group::{self, Mc, Nc, Pmc, Pnc, Psc, Ptc, Sc, Smc, Snc, Ssc, Stc, Tc},
                Meta,
            },
            Context,
        },
        view::View,
    },
    properties::density::Hammond,
    r#const::{
        polymorphism::{alpha, beta::K_X},
        relative_atomic_mass::C3H2,
    },
    tree::{Branch, Leaf, Node},
    utils::ui::UiExt,
};
use egui::{collapsing_header::CollapsingState, text::LayoutJob, Id, ScrollArea, Sense, Ui};
use egui_ext::CollapsingStateExt;
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
        context.compose(ui);
        ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            let composed = context.state.entry().data.composed.clone();
            composed
                .composition(context.settings.composition.method)
                .ui(ui, context, &mut vec![], None);
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
        context: &mut Context,
        path: &mut Vec<Option<Group>>,
        mut open: Option<bool>,
    ) {
        if self.meta.count.filtered == 0 && !context.settings.composition.empty {
            return;
        }
        path.push(self.meta.group);
        let p = context.settings.composition.precision;
        let text = if let Some(group) = self.meta.group {
            let text = &match group {
                Nc(ecn) => ecn.to_string(),
                Pnc(ecn) | Snc(ecn) => format!("{ecn:#}"),
                Mc(mass) => format!("{mass:#.p$}"),
                Pmc((c3h2, mass, adduct)) | Smc((c3h2, mass, adduct)) => {
                    let mut buffer = format!("{c3h2:.p$}{mass:#.p$}");
                    if adduct.value > 0 {
                        buffer += &format!("{adduct:.p$}");
                    }
                    buffer
                }
                Tc(r#type) | Ptc(r#type) | Stc(r#type) => r#type.to_string(),
                Sc(tag) | Psc(tag) | Ssc(tag) => format!("{:#}", context.species(tag)),
            };
            let subscription = group.composition().text();
            ui.subscripted_text(text, subscription, Default::default())
        } else {
            LayoutJob::single_section("∑".to_owned(), Default::default())
        };
        let collapsing_state =
            CollapsingState::load_with_default_open(ui.ctx(), Id::new(&path), path.len() < 1)
                .open(open.take());
        // let opened = collapsing_state.is_open();
        // let branches = !self
        //     .children
        //     .iter()
        //     .any(|node| matches!(node, Node::Leaf(_)));
        collapsing_state
            .show_header(ui, |ui| {
                // let available = ui.available_size_before_wrap().x;
                // let width = ui.spacing().interact_size.x;
                let response = ui
                    .horizontal(|ui| {
                        let mut response = ui.label(text);
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
                        let Count {
                            filtered,
                            unfiltered,
                        } = self.meta.count;
                        response |=
                            ui.label(format!("{filtered}/{unfiltered}"))
                                .on_hover_ui(|ui| {
                                    ui.label(format!("Filtered count: {filtered}"));
                                    ui.label(format!("Unfiltered count: {unfiltered}"));
                                });
                        response.sense |= Sense::click();
                        response
                    })
                    .inner;
                response.context_menu(|ui| {
                    if ui.button("Expand all children").clicked() {
                        open = Some(true);
                        ui.close_menu();
                    }
                    if ui.button("Collapse all children").clicked() {
                        open = Some(false);
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Add to compare").clicked() {
                        for leaf in self.leaves() {
                            let tag = context.species(leaf.data.tag).map(ToOwned::to_owned);
                            context.settings.comparison.set.insert(tag);
                        }
                        ui.close_menu();
                    }
                    if ui.button("Remove from compare").clicked() {
                        for leaf in self.leaves() {
                            let tag = context.species(leaf.data.tag).map(ToOwned::to_owned);
                            context.settings.comparison.set.shift_remove(&tag);
                        }
                        ui.close_menu();
                    }
                });
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
    pub fn ui(&self, ui: &mut Ui, context: &mut Context) {
        let p = context.settings.composition.precision;
        ui.horizontal(|ui| {
            let tag = self.data.tag;
            let species = context.species(tag);
            let text = ui.subscripted_text(
                &format!("{species:#}"),
                context.settings.composition.tree.leafs.text(),
                Default::default(),
            );
            let mut response = ui
                .label(text)
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
            response |= ui
                .label(format!("{value:.p$}"))
                .on_hover_text(format!("Unrounded: {value}"));
            response.context_menu(|ui| {
                let species = context.species(tag).map(ToOwned::to_owned);
                let contains = context.settings.comparison.set.contains(&species);
                if contains && ui.button("Remove from compare").clicked() {
                    context.settings.comparison.set.shift_remove(&species);
                    ui.close_menu();
                }
                if !contains && ui.button("Add to compare").clicked() {
                    context.settings.comparison.set.insert(species);
                    ui.close_menu();
                }
            });
        });
    }
}
