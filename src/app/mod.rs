use self::{
    tabs::{CentralDock, CentralTab, CentralTabs, LeftDock, LeftTab, LeftTabs},
    windows::About,
};
use crate::{
    app::context::Context,
    parsers::{
        toml::{to_string, Parsed as TomlParsed},
        whitespace::Parsed,
    },
    widgets::FileDialog,
};
use anyhow::Result;
use eframe::{get_value, set_value, CreationContext, Frame, Storage, APP_KEY};
use egui::{
    global_dark_light_mode_switch, vec2, warn_if_debug_build, Align, Align2, Button, CentralPanel,
    Color32, ComboBox, Event, FontId, Id, Label, LayerId, Layout, Order, RichText, Sense,
    SidePanel, TextStyle, TopBottomPanel, Vec2, Visuals,
};
use egui_dock::{DockArea, Style};
use egui_ext::{DroppedFileExt, HoveredFileExt, WithVisuals};
use egui_notify::Toasts;
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    fmt::{Debug, Write},
    mem::take,
    str,
    time::Duration,
};
use tracing::{debug, error, info, trace};

/// IEEE 754-2008
const MAX_PRECISION: usize = 16;

const NOTIFICATIONS_DURATION: Duration = Duration::from_secs(15);

// const DESCRIPTION: &str = "Positional-species and positional-type composition of TAG from mature fruit arils of the Euonymus section species, mol % of total TAG";

fn custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = custom_visuals(style.visuals);
    ctx.set_style(style);
}

fn custom_visuals<T: BorrowMut<Visuals>>(mut visuals: T) -> T {
    visuals.borrow_mut().collapsing_header_frame = true;
    visuals
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    context: Context,
    // Docks
    docks: Docks,
    #[serde(skip)]
    file_dialog: FileDialog,
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
        custom_style(&cc.egui_ctx);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.storage
            .and_then(|storage| get_value(storage, APP_KEY))
            .unwrap_or_default()
    }
}

// Panels
impl App {
    fn panels(&mut self, ctx: &egui::Context) {
        self.top_panel(ctx);
        self.bottom_panel(ctx);
        self.left_panel(ctx);
        self.central_panel(ctx);
    }

    // Bottom panel
    fn bottom_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                warn_if_debug_build(ui);
                ui.label(RichText::new(env!("CARGO_PKG_VERSION")).small());
                ui.separator();
            });
        });
    }

    // Central panel
    fn central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.0))
            .show(ctx, |ui| {
                DockArea::new(&mut self.docks.central)
                    .id(Id::new("central_dock"))
                    .scroll_area_in_tabs(false)
                    .style(Style::from_egui(&ctx.style()))
                    .show_inside(
                        ui,
                        &mut CentralTabs {
                            context: &mut self.context,
                        },
                    );
            });
    }

    // Left panel
    fn left_panel(&mut self, ctx: &egui::Context) {
        SidePanel::left("left_panel")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(0.0))
            .resizable(true)
            .show_animated(ctx, !self.docks.left.tree.is_empty(), |ui| {
                let mut style = Style::from_egui(&ctx.style());
                style.tabs.fill_tab_bar = true;
                DockArea::new(&mut self.docks.left.tree)
                    .id(Id::new("left_dock"))
                    .scroll_area_in_tabs(false)
                    .style(style)
                    .show_inside(
                        ui,
                        &mut LeftTabs {
                            context: &mut self.context,
                            tree: &self.docks.central,
                        },
                    );
            });

        // SidePanel::left("left_panel")
        //     .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(0.0))
        //     .resizable(true)
        //     .show_animated(ctx, self.settings, |ui| {
        //         // ui.heading("⚙ Settings");
        //         // ui.separator();
        //         // if let Some((_, tab)) = self.central_panel.tree.find_active_focused() {
        //         //     match *tab {
        //         //         Tab::Input => {
        //         //             self.tabs.input.settings(ui);
        //         //             ui.separator();
        //         //             ui.vertical_centered_justified(|ui| {
        //         //                 if ui.button(RichText::new("🖩 Calculate").heading()).clicked() {
        //         //                     let tab = self.tabs.calculation();
        //         //                     self.tree.push_to_focused_leaf(tab);
        //         //                 }
        //         //             });
        //         //         }
        //         //         Tab::Output(Output::Calculation { index }) => {
        //         //             self.tabs
        //         //                 .calculations
        //         //                 .get_mut(&index)
        //         //                 .map(|calculation| calculation.settings(ui));
        //         //             ui.separator();
        //         //             ui.vertical_centered_justified(|ui| {
        //         //                 if ui.button(RichText::new("Compose").heading()).clicked() {
        //         //                     let tab = self.tabs.composition(index);
        //         //                     self.tree.push_to_focused_leaf(tab);
        //         //                 }
        //         //             });
        //         //         }
        //         //         Tab::Output(Output::Composition { index }) => {
        //         //             self.tabs
        //         //                 .compositions
        //         //                 .get_mut(&index)
        //         //                 .map(|composition| composition.settings(ui));
        //         //             ui.separator();
        //         //             ui.vertical_centered_justified(|ui| {
        //         //                 if ui.button(RichText::new("📊 Visualize").heading()).clicked() {
        //         //                     self.tree
        //         //                         .push_to_first_leaf(Tab::Output(Output::Visualization {
        //         //                             index,
        //         //                         }));
        //         //                 }
        //         //             });
        //         //         }
        //         //         Tab::Output(Output::Visualization { index }) => {
        //         //             self.tabs.visualization.settings(ui);
        //         //         }
        //         //     }
        //         // }
        //         let mut style = Style::from_egui(&ctx.style());
        //         style.tabs.fill_tab_bar = true;
        //         DockArea::new(&mut self.docks.left.tree)
        //             .id(Id::new("left_dock"))
        //             .scroll_area_in_tabs(false)
        //             .style(style)
        //             .show_inside(ui, &mut self.docks.left.tabs);
        //     });
    }

    // Top panel
    fn top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;

                ui.group(|ui| {
                    global_dark_light_mode_switch(ui);
                });
                ui.separator();
                ui.group(|ui| {
                    if ui
                        .add(Button::new(RichText::new("🗑")))
                        .on_hover_text("Reset data")
                        .clicked()
                    {
                        *self = Self {
                            docks: take(&mut self.docks),
                            ..Default::default()
                        };
                    }
                    // Reset gui
                    if ui
                        .add(Button::new(RichText::new("🔃")))
                        .on_hover_text("Reset gui")
                        .clicked()
                    {
                        ui.with_visuals(|ui, _| {
                            ui.memory_mut(|memory| *memory = Default::default())
                        });
                    }
                    // Organize windows
                    if ui
                        .add(Button::new(RichText::new("▣")))
                        .on_hover_text("Organize windows")
                        .clicked()
                    {
                        self.docks.left = Default::default();
                        self.docks.central = Default::default();
                        ui.ctx().memory_mut(|memory| memory.reset_areas());
                    }
                });
                ui.separator();

                #[derive(Clone, Copy, Debug, PartialEq)]
                enum Format {
                    Toml,
                }
                ui.group(|ui| {
                    // Import file
                    if ui.button("📤").on_hover_text("Import file").clicked() {
                        if let Err(error) = self.import() {
                            error!(?error);
                        }
                    }
                    if let Some(bytes) = self.file_dialog.take() {
                        match String::from_utf8(bytes) {
                            Err(error) => {
                                error!(%error);
                                return;
                            }
                            Ok(content) => {
                                trace!(?content);
                                let parsed: TomlParsed = match content.parse() {
                                    Ok(file) => file,
                                    Err(error) => {
                                        error!(%error);
                                        return;
                                    }
                                };
                                trace!(?parsed);
                                self.context.state = parsed.into();
                            }
                        }
                    }
                    let mut format = Format::Toml;
                    ComboBox::from_id_source("export")
                        .selected_text(format!("{format:?}"))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut format, Format::Toml, "Toml");
                        });
                    // Export file
                    if ui.button("📥").on_hover_text("Export file").clicked() {
                        if let Err(error) = self.export() {
                            error!(?error);
                        }
                    }
                });
                ui.separator();
                ui.group(|ui| {
                    // Settings
                    let checked = self.docks.left.tree.find_tab(&LeftTab::Settings).is_some();
                    if ui
                        .selectable_label(checked, RichText::new("⚙"))
                        .on_hover_text("Settings")
                        .clicked()
                    {
                        self.docks.left.toggle(LeftTab::Settings);
                    }
                    // Files
                    let checked = self.docks.left.tree.find_tab(&LeftTab::Files).is_some();
                    let text = if checked { "📂" } else { "📁" };
                    if ui
                        .selectable_label(checked, RichText::new(text))
                        .on_hover_text("Files")
                        .clicked()
                    {
                        self.docks.left.toggle(LeftTab::Files);
                    }
                });
                ui.separator();
                ui.group(|ui| {
                    // Configuration
                    let tab = CentralTab::Configuration;
                    let found = self.docks.central.find_tab(&tab);
                    if ui
                        .selectable_label(found.is_some(), RichText::new("📝"))
                        .on_hover_text("Configuration")
                        .clicked()
                    {
                        if let Some(index) = found {
                            self.docks.central.remove_tab(index);
                        } else {
                            self.docks.central.push_to_focused_leaf(tab);
                        }
                    }
                    // Calculation
                    let tab = CentralTab::Calculation;
                    let found = self.docks.central.find_tab(&tab);
                    if ui
                        .selectable_label(found.is_some(), RichText::new("🖩"))
                        .on_hover_text("Calculation")
                        .clicked()
                    {
                        if let Some(index) = found {
                            self.docks.central.remove_tab(index);
                        } else {
                            self.docks.central.push_to_focused_leaf(tab);
                        }
                    }
                    // Composition
                    let tab = CentralTab::Composition;
                    let found = self.docks.central.find_tab(&tab);
                    if ui
                        .selectable_label(found.is_some(), RichText::new("⛃"))
                        .on_hover_text("Composition")
                        .clicked()
                    {
                        if let Some(index) = found {
                            self.docks.central.remove_tab(index);
                        } else {
                            self.docks.central.push_to_focused_leaf(tab);
                        }
                    }
                    // Visualization
                    let tab = CentralTab::Visualization;
                    let found = self.docks.central.find_tab(&tab);
                    if ui
                        .selectable_label(found.is_some(), RichText::new("📊"))
                        .on_hover_text("Visualization")
                        .clicked()
                    {
                        if let Some(index) = found {
                            self.docks.central.remove_tab(index);
                        } else {
                            self.docks.central.push_to_focused_leaf(tab);
                        }
                    }
                });
                ui.separator();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .add(Button::new(RichText::new("ℹ")))
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
}

// Windows
impl App {
    fn windows(&mut self, ctx: &egui::Context) {
        self.about.window(ctx);
    }
}

// Notifications
impl App {
    fn notifications(&mut self, ctx: &egui::Context) {
        self.toasts.show(ctx);
    }
}

// Copy/Paste, Drag&Drop
impl App {
    fn drag_and_drop(&mut self, ctx: &egui::Context) {
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
        if let Some(dropped_files) = ctx.input(|input| {
            (!input.raw.dropped_files.is_empty()).then_some(input.raw.dropped_files.clone())
        }) {
            info!(?dropped_files);
            // self.docks.left.tabs.files = Files {
            //     files,
            //     ..self.docks.left.tabs.files
            // };
            ctx.data_mut(|data| data.remove_by_type::<Parsed>());
            for dropped in dropped_files {
                let content = match dropped.content() {
                    Ok(content) => content,
                    Err(error) => {
                        error!(%error);
                        self.toasts
                            .error(format!("{}: {error}", dropped.display()))
                            .set_closable(true)
                            .set_duration(Some(NOTIFICATIONS_DURATION));
                        continue;
                    }
                };
                trace!(content);
                let parsed: TomlParsed = match content.parse() {
                    Ok(file) => file,
                    Err(error) => {
                        error!(%error);
                        self.toasts
                            .error(format!("{}: {error}", dropped.display()))
                            .set_closable(true)
                            .set_duration(Some(NOTIFICATIONS_DURATION));
                        continue;
                    }
                };
                trace!(?parsed);
                self.context.state = parsed.into();
            }
        }
    }

    fn paste(&mut self, ctx: &egui::Context) {
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
            // self.docks.central.tabs.input.add(match parsed {
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

    fn export(&self) -> Result<(), impl Debug> {
        let content = to_string(&TomlParsed {
            fatty_acids: self.context.state.fatty_acids(),
            ..Default::default()
        })
        .unwrap();
        self.file_dialog
            .save(&format!("{}.toml", self.context.state.meta.name), &content)
            .unwrap();
        Ok::<_, ()>(())
    }

    fn import(&mut self) -> Result<(), impl Debug> {
        self.file_dialog.load()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Pre update
        self.panels(ctx);
        self.windows(ctx);
        self.notifications(ctx);
        // Post update
        self.drag_and_drop(ctx);
        self.paste(ctx);
    }
}

#[derive(Default, Deserialize, Serialize)]
struct Docks {
    left: LeftDock,
    central: CentralDock,
}

mod computers;
mod context;
mod tabs;
mod windows;
