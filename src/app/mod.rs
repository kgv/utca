use self::{
    tabs::{left_dock::Files, CentralDock, LeftDock},
    windows::About,
};
use crate::{
    app::context::Unnormalized,
    ether::ether,
    parsers::{
        toml::{FattyAcid, Parsed as TomlParsed},
        whitespace::Parsed,
    },
    utils::egui::{Content, Display as _},
};
use anyhow::Result;
use eframe::{get_value, set_value, CreationContext, Frame, Storage, APP_KEY};
use egui::{
    global_dark_light_mode_switch, warn_if_debug_build, Align, Align2, Button, CentralPanel,
    Color32, Context, Event, Id, LayerId, Layout, Order, RichText, SidePanel, TextStyle,
    TopBottomPanel,
};
use egui_dock::{DockArea, NodeIndex, Style};
use egui_notify::Toasts;
use serde::{Deserialize, Serialize};
use std::{default::default, fmt::Write, future::Future, str, time::Duration};
use tracing::{debug, error, info};

/// IEEE 754-2008
const MAX_PRECISION: usize = 16;

// const DESCRIPTION: &str = "Positional-species and positional-type composition of TAG from mature fruit arils of the Euonymus section species, mol % of total TAG";

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + 'static>(f: F) {}

fn style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals.collapsing_header_frame = true;
    ctx.set_style(style);
}

// #[cfg(target_arch = "wasm32")]
// fn import() {
//     let window = web_sys::window().expect("Window not found");
//     let document = window.document().expect("Document not found");
//     let overlay = document.create_element("div").unwrap();
//     overlay.set_id("rfd-overlay");
//     let card = {
//         let card = document.create_element("div").unwrap();
//         card.set_id("rfd-card");
//         overlay.append_child(&card).unwrap();
//         card
//     };
//     let input = {
//         let input_el = document.create_element("input").unwrap();
//         let input: HtmlInputElement = wasm_bindgen::JsCast::dyn_into(input_el).unwrap();
//         input.set_id("rfd-input");
//         input.set_type("file");
//         let mut accept: Vec<String> = Vec::new();
//         for filter in opt.filters.iter() {
//             accept.append(&mut filter.extensions.to_vec());
//         }
//         accept.iter_mut().for_each(|ext| ext.insert_str(0, "."));
//         input.set_accept(&accept.join(","));
//         card.append_child(&input).unwrap();
//         input
//     };
//     let button = {
//         let btn_el = document.create_element("button").unwrap();
//         let btn: HtmlButtonElement = wasm_bindgen::JsCast::dyn_into(btn_el).unwrap();
//         btn.set_id("rfd-button");
//         btn.set_inner_text("Ok");
//         card.append_child(&btn).unwrap();
//         btn
//     };
//     let style = document.create_element("style").unwrap();
//     style.set_inner_html(include_str!("./wasm/style.css"));
//     overlay.append_child(&style).unwrap();
// }

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    // Panels
    settings: bool,
    // Tabs
    left_dock: LeftDock,
    central_dock: CentralDock,
    // Windows
    #[serde(skip)]
    about: About,
    // Notifications
    #[serde(skip)]
    toasts: Toasts,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext) -> Self {
        // Customize style of egui.
        style(&cc.egui_ctx);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.storage
            .and_then(|storage| get_value(storage, APP_KEY))
            .unwrap_or_default()
    }
}

// Panels
impl App {
    fn panels(&mut self, ctx: &Context) {
        self.top_panel(ctx);
        self.bottom_panel(ctx);
        self.left_panel(ctx);
        self.central_panel(ctx);
    }

    // Top panel
    fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.separator();
                if ui
                    .add(Button::new("🗑").frame(false))
                    .on_hover_text("Reset data")
                    .clicked()
                {
                    *self = Self {
                        settings: self.settings,
                        ..default()
                    };
                }
                ui.separator();
                if ui
                    .add(Button::new("🔃").frame(false))
                    .on_hover_text("Reset gui")
                    .clicked()
                {
                    ui.ctx().memory_mut(|memory| *memory = default());
                    style(ui.ctx());
                }
                ui.separator();
                if ui
                    .add(Button::new("▣").frame(false))
                    .on_hover_text("Organize windows")
                    .clicked()
                {
                    ui.ctx().memory_mut(|memory| memory.reset_areas());
                }
                ui.separator();
                ui.toggle_value(&mut self.settings, "⚙")
                    .on_hover_text("Settings");
                ui.separator();
                {
                    use self::tabs::left_dock::Tab;

                    let found = self.left_dock.tree.find_tab(&Tab::Files);
                    let checked = found.is_some();
                    let text = if checked { "📂" } else { "📁" };
                    if ui
                        .selectable_label(checked, text)
                        .on_hover_text("Files")
                        .clicked()
                    {
                        if let Some(index) = found {
                            self.left_dock.tree.remove_tab(index);
                        } else {
                            self.left_dock.tree.split_below(
                                NodeIndex::root(),
                                0.5,
                                vec![Tab::Files],
                            );
                        }
                    }
                }
                ui.separator();
                {
                    use self::tabs::central_dock::{Output, Tab};

                    let tab = Tab::Input;
                    let found = self.central_dock.tree.find_tab(&tab);
                    if ui
                        .selectable_label(found.is_some(), "📝")
                        .on_hover_text("Input")
                        .clicked()
                    {
                        if let Some(index) = found {
                            self.central_dock.tree.remove_tab(index);
                        } else {
                            self.central_dock.tree.push_to_focused_leaf(tab);
                        }
                    }
                    ui.separator();
                    let tab = Tab::Output(Output::Calculation);
                    let found = self.central_dock.tree.find_tab(&tab);
                    if ui
                        .selectable_label(found.is_some(), "🖩")
                        .on_hover_text("Calculation")
                        .clicked()
                    {
                        if let Some(index) = found {
                            self.central_dock.tree.remove_tab(index);
                        } else {
                            self.central_dock.tree.push_to_focused_leaf(tab);
                        }
                    }
                    ui.separator();
                    let tab = Tab::Output(Output::Composition);
                    let found = self.central_dock.tree.find_tab(&tab);
                    if ui
                        .selectable_label(found.is_some(), "⛃")
                        .on_hover_text("Composition")
                        .clicked()
                    {
                        if let Some(index) = found {
                            self.central_dock.tree.remove_tab(index);
                        } else {
                            self.central_dock.tree.push_to_focused_leaf(tab);
                        }
                    }
                    ui.separator();
                    let tab = Tab::Output(Output::Visualization);
                    let found = self.central_dock.tree.find_tab(&tab);
                    if ui
                        .selectable_label(found.is_some(), "📊")
                        .on_hover_text("Visualization")
                        .clicked()
                    {
                        if let Some(index) = found {
                            self.central_dock.tree.remove_tab(index);
                        } else {
                            self.central_dock.tree.push_to_focused_leaf(tab);
                        }
                    }
                }
                ui.separator();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .add(Button::new("ℹ").frame(false))
                        .on_hover_text("About window")
                        .clicked()
                    {
                        self.about.open ^= true;
                    }
                    ui.separator();
                });
            });
        });
    }

    // Left panel
    fn left_panel(&mut self, ctx: &Context) {
        use egui::Frame;

        SidePanel::left("left_panel")
            .frame(Frame::side_top_panel(&ctx.style()).inner_margin(0.0))
            .resizable(true)
            .show_animated(ctx, self.settings, |ui| {
                // ui.heading("⚙ Settings");
                // ui.separator();
                // if let Some((_, tab)) = self.central_panel.tree.find_active_focused() {
                //     match *tab {
                //         Tab::Input => {
                //             self.tabs.input.settings(ui);
                //             ui.separator();
                //             ui.vertical_centered_justified(|ui| {
                //                 if ui.button(RichText::new("🖩 Calculate").heading()).clicked() {
                //                     let tab = self.tabs.calculation();
                //                     self.tree.push_to_focused_leaf(tab);
                //                 }
                //             });
                //         }
                //         Tab::Output(Output::Calculation { index }) => {
                //             self.tabs
                //                 .calculations
                //                 .get_mut(&index)
                //                 .map(|calculation| calculation.settings(ui));
                //             ui.separator();
                //             ui.vertical_centered_justified(|ui| {
                //                 if ui.button(RichText::new("Compose").heading()).clicked() {
                //                     let tab = self.tabs.composition(index);
                //                     self.tree.push_to_focused_leaf(tab);
                //                 }
                //             });
                //         }
                //         Tab::Output(Output::Composition { index }) => {
                //             self.tabs
                //                 .compositions
                //                 .get_mut(&index)
                //                 .map(|composition| composition.settings(ui));
                //             ui.separator();
                //             ui.vertical_centered_justified(|ui| {
                //                 if ui.button(RichText::new("📊 Visualize").heading()).clicked() {
                //                     self.tree
                //                         .push_to_first_leaf(Tab::Output(Output::Visualization {
                //                             index,
                //                         }));
                //                 }
                //             });
                //         }
                //         Tab::Output(Output::Visualization { index }) => {
                //             self.tabs.visualization.settings(ui);
                //         }
                //     }
                // }

                let mut style = Style::from_egui(&ctx.style());
                style.tabs.fill_tab_bar = true;
                DockArea::new(&mut self.left_dock.tree)
                    .id(Id::new("left_dock"))
                    .scroll_area_in_tabs(false)
                    .style(style)
                    .show_inside(ui, &mut self.left_dock.tabs);
            });
    }

    // Bottom panel
    fn bottom_panel(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                warn_if_debug_build(ui);
                ui.label(RichText::new(env!("CARGO_PKG_VERSION")).small());
                ui.separator();
            });
        });
    }

    // Central panel
    fn central_panel(&mut self, ctx: &Context) {
        use egui::Frame;

        CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()).inner_margin(0.0))
            .show(ctx, |ui| {
                DockArea::new(&mut self.central_dock.tree)
                    .id(Id::new("central_dock"))
                    .scroll_area_in_tabs(false)
                    .style(Style::from_egui(&ctx.style()))
                    .show_inside(ui, &mut self.central_dock.tabs);
            });
    }
}

// Windows
impl App {
    fn windows(&mut self, ctx: &Context) {
        self.about.window(ctx);
    }
}

// Notifications
impl App {
    fn notifications(&mut self, ctx: &Context) {
        self.toasts.show(ctx);
    }
}

// Copy/Paste, Drag&Drop
impl App {
    fn drag_and_drop(&mut self, ctx: &Context) {
        // Preview hovering files
        if let Some(text) = ctx.input(|input| {
            (!input.raw.hovered_files.is_empty()).then(|| {
                let mut text = String::from("Dropping files:");
                for file in &input.raw.hovered_files {
                    write!(text, "\n{}", file.display()).ok();
                }
                text
            })
        }) {
            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));
            let screen_rect = ctx.screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }
        // Parse dropped files
        if let Some(files) = ctx.input(|input| {
            (!input.raw.dropped_files.is_empty()).then_some(input.raw.dropped_files.clone())
        }) {
            info!(?files);
            self.left_dock.tabs.files = Files {
                files,
                ..self.left_dock.tabs.files
            };
            ctx.data_mut(|data| data.remove_by_type::<Parsed>());
            for (index, file) in self.left_dock.tabs.files.iter().enumerate() {
                let content = match file.content() {
                    Ok(content) => content,
                    Err(error) => {
                        error!(%error);
                        continue;
                    }
                };
                let parsed: TomlParsed = match content.parse() {
                    Ok(file) => file,
                    Err(error) => {
                        error!(%error);
                        continue;
                    }
                };
                use crate::app::context::Context;

                let length = parsed.fatty_acids.len();
                let (labels, (formulas, (tag123, (dag1223, mag2)))): (
                    Vec<_>,
                    (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))),
                ) = parsed
                    .fatty_acids
                    .into_iter()
                    .map(|fatty_acid| {
                        (
                            fatty_acid.label,
                            (
                                fatty_acid.formula,
                                (
                                    fatty_acid.values[0],
                                    (fatty_acid.values[1], fatty_acid.values[2]),
                                ),
                            ),
                        )
                    })
                    .unzip();
                self.central_dock.tabs.context = Context {
                    labels,
                    formulas,
                    unnormalized: Unnormalized {
                        tags123: tag123,
                        dags1223: dag1223,
                        mags2: mag2,
                    },
                    ..default()
                };
                // ctx.data_mut(|data| {
                //     data.insert_temp(Id::new("parsed"), parsed);
                // });
            }
        }
    }

    fn paste(&mut self, ctx: &Context) {
        if !ctx.memory(|memory| memory.focus().is_some()) {
            ctx.input(|input| {
                for event in &input.raw.events {
                    if let Event::Paste(paste) = event {
                        if let Err(error) = self.parse(paste) {
                            error!(?error);
                            self.toasts
                                .error(error.to_string().chars().take(64).collect::<String>())
                                .set_duration(Some(Duration::from_secs(5)))
                                .set_closable(true);
                        }
                    }
                }
            });
        }
    }

    fn parse(&mut self, paste: &str) -> Result<()> {
        use crate::parsers::whitespace::Parser;

        let parsed = Parser::parse(paste)?;
        debug!(?parsed);
        for parsed in parsed {
            // self.central_dock.tabs.input.add(match parsed {
            //     Parsed::All(label, (c, n), tag, dag, mag) => FattyAcid {
            //         label,
            //         formula: ether!(c as usize, n as usize),
            //         values: [tag, dag, mag],
            //     },
            //     // Parsed::String(label) => Row { label, ..default() },
            //     // Parsed::Integers(_) => Row { label, ..default() },
            //     // Parsed::Float(tag) => Row { label, ..default() },
            //     _ => unimplemented!(),
            // })?;

            // self.config.push_row(Row {
            //     acylglycerols,
            //     label:  parsed.,
            //     ether: todo!(),
            //     // ..default()
            // })?;
        }

        // let mut rows = Vec::new();
        // for row in paste.split('\n') {
        //     let mut columns = [0.0; COUNT];
        //     for (j, column) in row.split('\t').enumerate() {
        //         ensure!(j < COUNT, "Invalid shape, columns: {COUNT} {j}");
        //         columns[j] = column.replace(',', ".").parse()?;
        //     }
        //     rows.push(columns);
        // }
        // for acylglycerols in rows {
        //     self.config.push_row(Row {
        //         acylglycerol: acylglycerols,
        //         ..default()
        //     })?;
        // }
        Ok(())
    }
}

impl App {
    fn pre_update(&mut self, _ctx: &Context) {}
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.pre_update(ctx);
        self.panels(ctx);
        self.windows(ctx);
        self.notifications(ctx);
        self.post_update(ctx);
    }
}

impl App {
    fn post_update(&mut self, ctx: &Context) {
        self.paste(ctx);
        self.drag_and_drop(ctx);
    }
}

mod computers;
mod context;
mod settings;
mod tabs;
mod windows;

#[test]
fn test() {
    let id = Id::new("UTCA");
    println!("id: {id:?}");
}
