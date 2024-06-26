use crate::{
    acylglycerol::Sn,
    app::{
        context::{
            settings::{
                composition::{Discrimination, Method, BRANCHES, PSC, PTC, SC, SSC},
                Order, Sort,
            },
            Context,
        },
        tabs::CentralTab,
        view::View,
        MAX_PRECISION,
    },
    r#const::relative_atomic_mass::{H, LI, NA, NH4},
    utils::ui::{SubscriptedTextFormat, UiExt as _},
};
use egui::{
    epaint::util::FloatOrd, text::LayoutJob, CollapsingHeader, ComboBox, DragValue, Id, Key,
    KeyboardShortcut, Modifiers, RichText, ScrollArea, Slider, TextStyle, Ui, Window,
};
use egui_animation::animate_eased;
use egui_dnd::dnd;
use egui_ext::{color, ClickedLabel};
use egui_extras::{Column, TableBuilder};
use egui_plot::{Line, Plot, PlotBounds, PlotItem, PlotPoints};
use simple_easing::linear;
use std::hash::{Hash, Hasher};

/// Left composition tab
pub(super) struct Composition<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Composition<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }

    fn windows(self, ui: &mut Ui) {
        let Self { context } = self;
        Window::new("📊 Method")
            .open(&mut context.settings.composition.window)
            .show(ui.ctx(), |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.ctx().request_repaint();
                    let id = Id::new("method");
                    let mut u3: Parameters =
                        ui.data_mut(|data| *data.get_temp_mut_or_default(id.with("u3")));
                    let mut su2: [Parameters; 2] =
                        ui.data_mut(|data| *data.get_temp_mut_or_default(id.with("su2")));
                    let mut s2u: [Parameters; 2] =
                        ui.data_mut(|data| *data.get_temp_mut_or_default(id.with("s2u")));
                    let mut s3: Parameters =
                        ui.data_mut(|data| *data.get_temp_mut_or_default(id.with("s3")));
                    ui.collapsing("Settings", |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Hildtch").clicked() {
                                u3 = Parameters {
                                    x0: 0.0,
                                    x1: 1.0 / 3.0,
                                    y0: -1.0,
                                    y1: 0.0,
                                    k0: 0.0,
                                    k1: 1.0,
                                };
                                // y0 * (((x - x0) / (x1 - x0)).abs().powf(k0) * (1.0 - (x - x0) / (x1 - x0)).abs().powf(k1) - y1)
                                // (x-(1)/(3))/((2)/(3)-(1)/(3))

                                // (x-(1)/(3))/((2)/(3)-(1)/(3))
                                // (x-(2)/(3))/((1)/(3)-(2)/(3))
                                su2 = [
                                    Parameters {
                                        x0: 0.0,
                                        x1: 1.0 / 3.0,
                                        y0: 1.0,
                                        y1: 1.0,
                                        k0: 0.0,
                                        k1: 1.0,
                                    },
                                    Parameters {
                                        x0: 1.0 / 3.0,
                                        x1: 2.0 / 3.0,
                                        y0: 1.0,
                                        y1: 1.0,
                                        k0: 1.0,
                                        k1: 0.0,
                                    },
                                ];
                                s2u = [
                                    Parameters {
                                        x0: 1.0 / 3.0,
                                        x1: 2.0 / 3.0,
                                        y0: 1.0,
                                        y1: 1.0,
                                        k0: 0.0,
                                        k1: 1.0,
                                    },
                                    Parameters {
                                        x0: 2.0 / 3.0,
                                        x1: 1.0,
                                        y0: 1.0,
                                        y1: 1.0,
                                        k0: 1.0,
                                        k1: 0.0,
                                    },
                                ];
                                s3 = Parameters {
                                    x0: 2.0 / 3.0,
                                    x1: 1.0,
                                    y0: -1.0,
                                    y1: 0.0,
                                    k0: 1.0,
                                    k1: 0.0,
                                };
                            }
                            if ui.button("Gunstone").clicked() {
                                u3 = Parameters {
                                    x0: 0.0,
                                    x1: 2.0 / 3.0,
                                    y0: -1.0,
                                    y1: 0.0,
                                    k0: 0.0,
                                    k1: 2.0,
                                };
                                su2 = [
                                    Parameters {
                                        x0: 0.0,
                                        x1: 1.0 / 3.0,
                                        y0: 0.5,
                                        y1: 1.0,
                                        k0: 0.0,
                                        k1: 2.0,
                                    },
                                    Parameters {
                                        x0: 1.0 / 3.0,
                                        x1: 2.0 / 3.0,
                                        y0: 0.5,
                                        y1: 1.0,
                                        k0: 2.0,
                                        k1: 0.0,
                                    },
                                ];
                                s2u = [
                                    Parameters {
                                        x0: 0.0,
                                        x1: 2.0 / 3.0,
                                        y0: -1.0,
                                        y1: 0.0,
                                        k0: 2.0,
                                        k1: 0.0,
                                    },
                                    Parameters {
                                        x0: 2.0 / 3.0,
                                        x1: 1.0,
                                        y0: 1.0,
                                        y1: 1.0,
                                        k0: 1.0,
                                        k1: 0.0,
                                    },
                                ];
                                s3 = Parameters {
                                    x0: 2.0 / 3.0,
                                    x1: 1.0,
                                    y0: -1.0,
                                    y1: 0.0,
                                    k0: 1.0,
                                    k1: 0.0,
                                };
                            }
                            if ui.button("Vander Wal").clicked() {
                                u3 = Parameters {
                                    x0: 0.0,
                                    x1: 1.0,
                                    y0: -1.0,
                                    y1: 0.0,
                                    k0: 0.0,
                                    k1: 3.0,
                                };
                                su2 = [
                                    Parameters {
                                        x0: 0.0,
                                        x1: 1.0,
                                        y0: -3.0,
                                        y1: 0.0,
                                        k0: 1.0,
                                        k1: 2.0,
                                    },
                                    Parameters {
                                        x0: 0.0,
                                        x1: 1.0,
                                        y0: -3.0,
                                        y1: 0.0,
                                        k0: 1.0,
                                        k1: 2.0,
                                    },
                                ];
                                s2u = [
                                    Parameters {
                                        x0: 0.0,
                                        x1: 1.0,
                                        y0: -3.0,
                                        y1: 0.0,
                                        k0: 2.0,
                                        k1: 1.0,
                                    },
                                    Parameters {
                                        x0: 0.0,
                                        x1: 1.0,
                                        y0: -3.0,
                                        y1: 0.0,
                                        k0: 2.0,
                                        k1: 1.0,
                                    },
                                ];
                                s3 = Parameters {
                                    x0: 0.0,
                                    x1: 1.0,
                                    y0: -1.0,
                                    y1: 0.0,
                                    k0: 3.0,
                                    k1: 0.0,
                                };
                            }
                        });
                        ui.columns(6, |ui| {
                            ui[0].vertical(|ui| {
                                ui.label("U3");
                                ui.horizontal(|ui| {
                                    ui.label("x:");
                                    ui.add(DragValue::new(&mut u3.x0).speed(0.1));
                                    ui.add(DragValue::new(&mut u3.x1).speed(0.1));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("y:");
                                    ui.add(DragValue::new(&mut u3.y0).speed(0.1));
                                    ui.add(
                                        DragValue::new(&mut u3.y1)
                                            .clamp_range(0.0..=1.0)
                                            .speed(0.1),
                                    );
                                });
                                ui.horizontal(|ui| {
                                    ui.label("k:");
                                    ui.add(
                                        DragValue::new(&mut u3.k0)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                    ui.add(
                                        DragValue::new(&mut u3.k1)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                });
                            });
                            ui[1].vertical(|ui| {
                                ui.label("SU2[0]");
                                ui.horizontal(|ui| {
                                    ui.label("x:");
                                    ui.add(DragValue::new(&mut su2[0].x0).speed(0.1));
                                    ui.add(DragValue::new(&mut su2[0].x1).speed(0.1));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("y:");
                                    ui.add(DragValue::new(&mut su2[0].y0).speed(0.1));
                                    ui.add(
                                        DragValue::new(&mut su2[0].y1)
                                            .clamp_range(0.0..=1.0)
                                            .speed(0.1),
                                    );
                                });
                                ui.horizontal(|ui| {
                                    ui.label("k:");
                                    ui.add(
                                        DragValue::new(&mut su2[0].k0)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                    ui.add(
                                        DragValue::new(&mut su2[0].k1)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                });
                            });
                            ui[2].vertical(|ui| {
                                ui.label("SU2[1]");
                                ui.horizontal(|ui| {
                                    ui.label("x:");
                                    ui.add(DragValue::new(&mut su2[1].x0).speed(0.1));
                                    ui.add(DragValue::new(&mut su2[1].x1).speed(0.1));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("y:");
                                    ui.add(DragValue::new(&mut su2[1].y0).speed(0.1));
                                    ui.add(
                                        DragValue::new(&mut su2[1].y1)
                                            .clamp_range(0.0..=1.0)
                                            .speed(0.1),
                                    );
                                });
                                ui.horizontal(|ui| {
                                    ui.label("k:");
                                    ui.add(
                                        DragValue::new(&mut su2[1].k0)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                    ui.add(
                                        DragValue::new(&mut su2[1].k1)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                });
                            });
                            ui[3].vertical(|ui| {
                                ui.label("S2U");
                                ui.horizontal(|ui| {
                                    ui.label("x:");
                                    ui.add(DragValue::new(&mut s2u[0].x0).speed(0.1));
                                    ui.add(DragValue::new(&mut s2u[0].x1).speed(0.1));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("y:");
                                    ui.add(DragValue::new(&mut s2u[0].y0).speed(0.1));
                                    ui.add(
                                        DragValue::new(&mut s2u[0].y1)
                                            .clamp_range(0.0..=1.0)
                                            .speed(0.1),
                                    );
                                });
                                ui.horizontal(|ui| {
                                    ui.label("k:");
                                    ui.add(
                                        DragValue::new(&mut s2u[0].k0)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                    ui.add(
                                        DragValue::new(&mut s2u[0].k1)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                });
                            });
                            ui[4].vertical(|ui| {
                                ui.label("S2U");
                                ui.horizontal(|ui| {
                                    ui.label("x:");
                                    ui.add(DragValue::new(&mut s2u[1].x0).speed(0.1));
                                    ui.add(DragValue::new(&mut s2u[1].x1).speed(0.1));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("y:");
                                    ui.add(DragValue::new(&mut s2u[1].y0).speed(0.1));
                                    ui.add(
                                        DragValue::new(&mut s2u[1].y1)
                                            .clamp_range(0.0..=1.0)
                                            .speed(0.1),
                                    );
                                });
                                ui.horizontal(|ui| {
                                    ui.label("k:");
                                    ui.add(
                                        DragValue::new(&mut s2u[1].k0)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                    ui.add(
                                        DragValue::new(&mut s2u[1].k1)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                });
                            });
                            ui[5].vertical(|ui| {
                                ui.label("S3");
                                ui.horizontal(|ui| {
                                    ui.label("x:");
                                    ui.add(DragValue::new(&mut s3.x0).speed(0.1));
                                    ui.add(DragValue::new(&mut s3.x1).speed(0.1));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("y:");
                                    ui.add(DragValue::new(&mut s3.y0).speed(0.1));
                                    ui.add(
                                        DragValue::new(&mut s3.y1)
                                            .clamp_range(0.0..=1.0)
                                            .speed(0.1),
                                    );
                                });
                                ui.horizontal(|ui| {
                                    ui.label("k:");
                                    ui.add(
                                        DragValue::new(&mut s3.k0)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                    ui.add(
                                        DragValue::new(&mut s3.k1)
                                            .clamp_range(0.0..=f64::MAX)
                                            .speed(0.01),
                                    );
                                });
                            });
                        });
                    });
                    ui.data_mut(|data| data.insert_temp(id.with("u3"), u3));
                    ui.data_mut(|data| data.insert_temp(id.with("su2"), su2));
                    ui.data_mut(|data| data.insert_temp(id.with("s2u"), s2u));
                    ui.data_mut(|data| data.insert_temp(id.with("s3"), s3));

                    // let id = Id::new("u3");
                    // u3.x0 = animate_eased(ui.ctx(), id.with("x0"), u3.x0 as _, 1.0, linear) as _;
                    // u3.x1 = animate_eased(ui.ctx(), id.with("x1"), u3.x1 as _, 1.0, linear) as _;
                    // u3.y0 = animate_eased(ui.ctx(), id.with("y0"), u3.y0 as _, 1.0, linear) as _;
                    // u3.y1 = animate_eased(ui.ctx(), id.with("y1"), u3.y1 as _, 1.0, linear) as _;
                    // u3.k0 = animate_eased(ui.ctx(), id.with("k0"), u3.k0 as _, 1.0, linear) as _;
                    // u3.k1 = animate_eased(ui.ctx(), id.with("k1"), u3.k1 as _, 1.0, linear) as _;
                    // let id = Id::new("su2");
                    // su2[0].x0 =
                    //     animate_eased(ui.ctx(), id.with("x0"), su2[0].x0 as _, 1.0, linear) as _;
                    // su2[0].x1 =
                    //     animate_eased(ui.ctx(), id.with("x1"), su2[0].x1 as _, 1.0, linear) as _;
                    // su2[0].y0 =
                    //     animate_eased(ui.ctx(), id.with("y0"), su2[0].y0 as _, 1.0, linear) as _;
                    // su2[0].y1 =
                    //     animate_eased(ui.ctx(), id.with("y1"), su2[0].y1 as _, 1.0, linear) as _;
                    // su2[0].k0 =
                    //     animate_eased(ui.ctx(), id.with("k0"), su2[0].k0 as _, 1.0, linear) as _;
                    // su2[0].k1 =
                    //     animate_eased(ui.ctx(), id.with("k1"), su2[0].k1 as _, 1.0, linear) as _;
                    // su2[1].x0 =
                    //     animate_eased(ui.ctx(), id.with("x0"), su2[1].x0 as _, 1.0, linear) as _;
                    // su2[1].x1 =
                    //     animate_eased(ui.ctx(), id.with("x1"), su2[1].x1 as _, 1.0, linear) as _;
                    // su2[1].y0 =
                    //     animate_eased(ui.ctx(), id.with("y0"), su2[1].y0 as _, 1.0, linear) as _;
                    // su2[1].y1 =
                    //     animate_eased(ui.ctx(), id.with("y1"), su2[1].y1 as _, 1.0, linear) as _;
                    // su2[1].k0 =
                    //     animate_eased(ui.ctx(), id.with("k0"), su2[1].k0 as _, 1.0, linear) as _;
                    // su2[1].k1 =
                    //     animate_eased(ui.ctx(), id.with("k1"), su2[1].k1 as _, 1.0, linear) as _;
                    // let id = Id::new("s2u");
                    // s2u.x0 = animate_eased(ui.ctx(), id.with("x0"), s2u.x0 as _, 1.0, linear) as _;
                    // s2u.x1 = animate_eased(ui.ctx(), id.with("x1"), s2u.x1 as _, 1.0, linear) as _;
                    // s2u.y0 = animate_eased(ui.ctx(), id.with("y0"), s2u.y0 as _, 1.0, linear) as _;
                    // s2u.y1 = animate_eased(ui.ctx(), id.with("y1"), s2u.y1 as _, 1.0, linear) as _;
                    // s2u.k0 = animate_eased(ui.ctx(), id.with("k0"), s2u.k0 as _, 1.0, linear) as _;
                    // s2u.k1 = animate_eased(ui.ctx(), id.with("k1"), s2u.k1 as _, 1.0, linear) as _;
                    // let id = Id::new("s3");
                    // s3.x0 = animate_eased(ui.ctx(), id.with("x0"), s3.x0 as _, 1.0, linear) as _;
                    // s3.x1 = animate_eased(ui.ctx(), id.with("x1"), s3.x1 as _, 1.0, linear) as _;
                    // s3.y0 = animate_eased(ui.ctx(), id.with("y0"), s3.y0 as _, 1.0, linear) as _;
                    // s3.y1 = animate_eased(ui.ctx(), id.with("y1"), s3.y1 as _, 1.0, linear) as _;
                    // s3.k0 = animate_eased(ui.ctx(), id.with("k0"), s3.k0 as _, 1.0, linear) as _;
                    // s3.k1 = animate_eased(ui.ctx(), id.with("k1"), s3.k1 as _, 1.0, linear) as _;

                    ui.vertical_centered_justified(|ui| {
                        let plot = Plot::new("plot");
                        plot.show(ui, |ui| {
                            ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [1.0, 1.0]));
                            ui.line({
                                let Parameters {
                                    x0,
                                    x1,
                                    y0,
                                    y1,
                                    k0,
                                    k1,
                                } = u3;
                                Line::new(PlotPoints::from_explicit_callback(
                                    move |x| {
                                        y0 * (y1
                                            - ((x - x0) / (x1 - x0)).abs().powf(k0)
                                                * (1.0 - (x - x0) / (x1 - x0)).abs().powf(k1))
                                    },
                                    x0.min(x1)..=x0.max(x1),
                                    256,
                                ))
                                .color(color(0))
                                .name("U3")
                            });
                            ui.line({
                                let Parameters {
                                    x0,
                                    x1,
                                    y0,
                                    y1,
                                    k0,
                                    k1,
                                } = su2[0];
                                Line::new(PlotPoints::from_explicit_callback(
                                    move |x| {
                                        y0 * (y1
                                            - ((x - x0) / (x1 - x0)).powf(k0)
                                                * (1.0 - (x - x0) / (x1 - x0)).powf(k1))
                                    },
                                    x0.min(x1)..=x0.max(x1),
                                    256,
                                ))
                                .color(color(1))
                                .name("SU2")
                            });
                            ui.line({
                                let Parameters {
                                    x0,
                                    x1,
                                    y0,
                                    y1,
                                    k0,
                                    k1,
                                } = su2[1];
                                Line::new(PlotPoints::from_explicit_callback(
                                    move |x| {
                                        y0 * (y1
                                            - ((x - x0) / (x1 - x0)).powf(k0)
                                                * (1.0 - (x - x0) / (x1 - x0)).powf(k1))
                                    },
                                    x0.min(x1)..=x0.max(x1),
                                    256,
                                ))
                                .color(color(1))
                                .name("SU2")
                            });
                            ui.line({
                                let Parameters {
                                    x0,
                                    x1,
                                    y0,
                                    y1,
                                    k0,
                                    k1,
                                } = s2u[0];
                                Line::new(PlotPoints::from_explicit_callback(
                                    move |x| {
                                        y0 * (y1
                                            - ((x - x0) / (x1 - x0)).powf(k0)
                                                * (1.0 - (x - x0) / (x1 - x0)).powf(k1))
                                    },
                                    x0.min(x1)..=x0.max(x1),
                                    256,
                                ))
                                .color(color(2))
                                .name("S2U")
                            });
                            ui.line({
                                let Parameters {
                                    x0,
                                    x1,
                                    y0,
                                    y1,
                                    k0,
                                    k1,
                                } = s2u[1];
                                Line::new(PlotPoints::from_explicit_callback(
                                    move |x| {
                                        y0 * (y1
                                            - ((x - x0) / (x1 - x0)).powf(k0)
                                                * (1.0 - (x - x0) / (x1 - x0)).powf(k1))
                                    },
                                    x0.min(x1)..=x0.max(x1),
                                    256,
                                ))
                                .color(color(2))
                                .name("S2U")
                            });
                            ui.line({
                                let Parameters {
                                    x0,
                                    x1,
                                    y0,
                                    y1,
                                    k0,
                                    k1,
                                } = s3;
                                Line::new(PlotPoints::from_explicit_callback(
                                    move |x| {
                                        y0 * (y1
                                            - ((x - x0) / (x1 - x0)).abs().powf(k0)
                                                * (1.0 - (x - x0) / (x1 - x0)).abs().powf(k1))
                                    },
                                    x0.min(x1)..=x0.max(x1),
                                    256,
                                ))
                                .color(color(3))
                                .name("S3")
                            });
                        });
                    });
                });
            });
    }
}

/// Parameters
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub(in crate::app) struct Parameters {
    pub(in crate::app) x0: f64,
    pub(in crate::app) x1: f64,
    pub(in crate::app) y0: f64,
    pub(in crate::app) y1: f64,
    pub(in crate::app) k0: f64,
    pub(in crate::app) k1: f64,
}

impl Hash for Parameters {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x0.ord().hash(state);
        self.x1.ord().hash(state);
        self.y0.ord().hash(state);
        self.k0.ord().hash(state);
        self.k1.ord().hash(state);
    }
}

impl View for Composition<'_> {
    fn view(self, ui: &mut Ui) {
        let Self { context } = self;
        ui.collapsing(
            RichText::new(CentralTab::Composition.title()).heading(),
            |ui| {
                ui.horizontal(|ui| {
                    ui.toggle_value(&mut context.settings.composition.resizable, "↔ Resizable")
                        .on_hover_text("Resize table columns")
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Precision:");
                    let precision = &mut context.settings.composition.precision;
                    if ui.add(Slider::new(precision, 0..=MAX_PRECISION)).changed()
                        && context.settings.link
                    {
                        context.settings.configuration.precision = *precision;
                        context.settings.calculation.precision = *precision;
                        context.settings.visualization.precision = *precision;
                        context.settings.comparison.precision = *precision;
                    }
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.horizontal(|ui| {
                    ui.label("Percent:");
                    ui.checkbox(&mut context.settings.composition.percent, "");
                    ui.toggle_value(&mut context.settings.link, "🔗");
                });
                ui.horizontal(|ui| {
                    ui.label("Show empty:");
                    ui.checkbox(&mut context.settings.composition.empty, "");
                })
                .response
                .on_hover_text("Show empty branches");
                ui.separator();
                ui.horizontal(|ui| {
                    let adduct = &mut context.settings.composition.adduct;
                    ui.label("Adduct:");
                    ui.add(
                        DragValue::new(&mut adduct.0)
                            .clamp_range(0.0..=f64::MAX)
                            .speed(1.0 / 10f64.powi(context.settings.composition.precision as _)),
                    )
                    .on_hover_text(format!("{adduct}"));
                    ComboBox::from_id_source("")
                        .selected_text(match adduct.0 {
                            adduct if adduct == H => "H",
                            adduct if adduct == NH4 => "NH4",
                            adduct if adduct == NA => "Na",
                            adduct if adduct == LI => "Li",
                            _ => "",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut adduct.0, H, "H");
                            ui.selectable_value(&mut adduct.0, NH4, "NH4");
                            ui.selectable_value(&mut adduct.0, NA, "Na");
                            ui.selectable_value(&mut adduct.0, LI, "Li");
                        });
                });
                ui.separator();
                ui.horizontal_top(|ui| {
                    let response = dnd(ui, Id::new("dnd").with("branches")).show(
                        context.settings.composition.tree.branches.iter(),
                        |ui, composition, handle, _state| {
                            handle.ui(ui, |ui| {
                                let _ = ui.button(composition.text());
                            });
                        },
                    );
                    if let Some(mut update) = response.final_update() {
                        let branches = &mut context.settings.composition.tree.branches;
                        if let Some(composition) = branches.shift_remove_index(update.from) {
                            if update.to > update.from {
                                update.to -= 1;
                            }
                            branches.shift_insert(update.to, composition);
                        }
                    }
                    ui.set_enabled(false);
                    let _ = ui.button(context.settings.composition.tree.leafs.text());
                });
                ui.horizontal(|ui| {
                    ui.label("Tree:");
                    ui.menu_button("Branches", |ui| {
                        for (scope, compositions) in &*BRANCHES {
                            ui.collapsing(scope.text(), |ui| {
                                for composition in compositions {
                                    let contains = context
                                        .settings
                                        .composition
                                        .tree
                                        .branches
                                        .contains(composition);
                                    if *composition >= context.settings.composition.tree.leafs {
                                        ui.set_enabled(false);
                                    }
                                    if ui
                                        .selectable_label(contains, composition.text())
                                        .on_hover_text(composition.hover_text())
                                        .clicked()
                                    {
                                        if contains {
                                            context
                                                .settings
                                                .composition
                                                .tree
                                                .branches
                                                .shift_remove(composition);
                                        } else {
                                            context
                                                .settings
                                                .composition
                                                .tree
                                                .branches
                                                .insert(*composition);
                                        }
                                    }
                                }
                            })
                            .header_response
                            .on_hover_text(scope.hover_text());
                        }
                    });
                    // ui.menu_button("Branches", |ui| {
                    //     let response = dnd(ui, Id::new("dnd").with("branches")).show(
                    //         context.settings.composition.tree.branches.iter_mut(),
                    //         |ui, (scope, compositions), handle, state| {
                    //             ui.horizontal(|ui| {
                    //                 handle.ui(ui, |ui| {
                    //                     let _ =
                    //                         ui.button(if state.dragged { "👊" } else { "✋" });
                    //                 });
                    //                 ui.collapsing(scope.text(), |ui| {
                    //                     for composition in &GROUPS[scope] {
                    //                         let contains = compositions.contains(composition);
                    //                         if *composition
                    //                             >= context.settings.composition.tree.leafs
                    //                         {
                    //                             ui.set_enabled(false);
                    //                         }
                    //                         if ui
                    //                             .selectable_label(contains, composition.text())
                    //                             .on_hover_text(composition.hover_text())
                    //                             .clicked()
                    //                         {
                    //                             if contains {
                    //                                 compositions.remove(composition);
                    //                             } else {
                    //                                 compositions.insert(*composition);
                    //                             }
                    //                         }
                    //                     }
                    //                 });
                    //             });
                    //         },
                    //     );
                    //     if let Some(mut update) = response.final_update() {
                    //         let branches = &mut context.settings.composition.tree.branches;
                    //         if let Some((key, value)) = branches.shift_remove_index(update.from) {
                    //             if update.to > update.from {
                    //                 update.to -= 1;
                    //             }
                    //             branches.shift_insert(update.to, key, value);
                    //         }
                    //     }
                    // });
                    ui.menu_button("Leafs", |ui| {
                        let mut response = ui
                            .selectable_value(
                                &mut context.settings.composition.tree.leafs,
                                SC,
                                SC.text(),
                            )
                            .on_hover_text(SC.hover_text());
                        response |= ui
                            .selectable_value(
                                &mut context.settings.composition.tree.leafs,
                                PSC,
                                PSC.text(),
                            )
                            .on_hover_text(PSC.hover_text());
                        response |= ui
                            .selectable_value(
                                &mut context.settings.composition.tree.leafs,
                                SSC,
                                SSC.text(),
                            )
                            .on_hover_text(SSC.hover_text());
                        if response.changed() {
                            context
                                .settings
                                .composition
                                .tree
                                .branches
                                .retain(|&composition| {
                                    composition < context.settings.composition.tree.leafs
                                });
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Sort:");
                    ComboBox::from_id_source("sort")
                        .selected_text(context.settings.composition.sort.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.composition.sort,
                                Sort::Key,
                                Sort::Key.text(),
                            )
                            .on_hover_text(Sort::Key.hover_text());
                            ui.selectable_value(
                                &mut context.settings.composition.sort,
                                Sort::Value,
                                Sort::Value.text(),
                            )
                            .on_hover_text(Sort::Value.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.composition.sort.hover_text());
                });
                ui.horizontal(|ui| {
                    ui.label("Order:");
                    ComboBox::from_id_source("order")
                        .selected_text(context.settings.composition.order.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.composition.order,
                                Order::Ascending,
                                Order::Ascending.text(),
                            )
                            .on_hover_text(Order::Ascending.hover_text());
                            ui.selectable_value(
                                &mut context.settings.composition.order,
                                Order::Descending,
                                Order::Descending.text(),
                            )
                            .on_hover_text(Order::Descending.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.composition.order.hover_text());
                });
                ui.horizontal(|ui| {
                    if ui.input_mut(|input| {
                        input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::G))
                    }) {
                        context.settings.composition.method = Method::Gunstone;
                    }
                    if ui.input_mut(|input| {
                        input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::W))
                    }) {
                        context.settings.composition.method = Method::VanderWal;
                    }
                    ui.label("Method:");
                    ComboBox::from_id_source("method")
                        .selected_text(context.settings.composition.method.text())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut context.settings.composition.method,
                                Method::Gunstone,
                                Method::Gunstone.text(),
                            )
                            .on_hover_text(Method::Gunstone.hover_text());
                            ui.selectable_value(
                                &mut context.settings.composition.method,
                                Method::VanderWal,
                                Method::VanderWal.text(),
                            )
                            .on_hover_text(Method::VanderWal.hover_text());
                        })
                        .response
                        .on_hover_text(context.settings.composition.method.hover_text());
                    if ui.button("📊").clicked() {
                        context.settings.composition.window ^= true;
                    }
                });
                if let Method::Gunstone = context.settings.composition.method {
                    ui.horizontal(|ui| {
                        ui.label("Discrimination:");
                        // ui.spacing_mut().combo_width = 0.75 * ui.spacing().combo_width;
                        ui.discrimination_menu(context, Sn::One);
                        ui.discrimination_menu(context, Sn::Two);
                        ui.discrimination_menu(context, Sn::Three);
                    });
                }
            },
        );
        Self { context }.windows(ui);
    }
}

/// Extension methods for [`Ui`]
trait UiExt {
    fn discrimination_menu(&mut self, context: &mut Context, sn: Sn);
}

impl UiExt for Ui {
    fn discrimination_menu(&mut self, context: &mut Context, sn: Sn) {
        let Discrimination { sn1, sn2, sn3 } = &mut context.settings.composition.discrimination;
        let psc = context.settings.composition.tree.leafs == PSC;
        let mut changed = false;
        self.menu_button(
            self.subscripted_text(
                "SN",
                sn.text(),
                SubscriptedTextFormat {
                    widget: true,
                    ..Default::default()
                },
            ),
            |ui| {
                for (index, label) in context.state.entry().meta.labels.iter().enumerate() {
                    let mut checked = match sn {
                        Sn::One => !sn1.contains(&index),
                        Sn::Two => !sn2.contains(&index),
                        Sn::Three => !sn3.contains(&index),
                    };
                    if ui.checkbox(&mut checked, label).changed() {
                        changed |= true;
                        if !checked {
                            match sn {
                                Sn::One | Sn::Three if psc => {
                                    sn1.insert(index);
                                    sn3.insert(index);
                                }
                                Sn::One => {
                                    sn1.insert(index);
                                }
                                Sn::Two => {
                                    sn2.insert(index);
                                }
                                Sn::Three => {
                                    sn3.insert(index);
                                }
                            }
                        } else {
                            match sn {
                                Sn::One | Sn::Three if psc => {
                                    sn1.remove(&index);
                                    sn3.remove(&index);
                                }
                                Sn::One => {
                                    sn1.remove(&index);
                                }
                                Sn::Two => {
                                    sn2.remove(&index);
                                }
                                Sn::Three => {
                                    sn3.remove(&index);
                                }
                            }
                        }
                    }
                }
            },
        )
        .response
        .context_menu(|ui| {
            if ui.button("Check all").clicked() {
                match sn {
                    Sn::One | Sn::Three if psc => {
                        sn1.clear();
                        sn3.clear();
                    }
                    Sn::One => {
                        sn1.clear();
                    }
                    Sn::Two => {
                        sn2.clear();
                    }
                    Sn::Three => {
                        sn3.clear();
                    }
                }
                ui.close_menu();
            } else if ui.button("Uncheck all").clicked() {
                let all = (0..context.state.entry().meta.labels.len()).collect();
                match sn {
                    Sn::One | Sn::Three if psc => {
                        *sn1 = all;
                        *sn3 = sn1.clone();
                    }
                    Sn::One => {
                        *sn1 = all;
                    }
                    Sn::Two => {
                        *sn2 = all;
                    }
                    Sn::Three => {
                        *sn3 = all;
                    }
                }
                ui.close_menu();
            }
        });
    }
}
