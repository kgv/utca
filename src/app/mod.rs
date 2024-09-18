use self::{
    panes::{Behavior, Pane},
    windows::{About, Github},
};
use crate::{
    localization::{titlecase, UiExt},
    widgets::FileDialog,
};
use data::Data;
use eframe::{get_value, set_value, CreationContext, Storage, APP_KEY};
use egui::{
    menu::bar, warn_if_debug_build, Align, Align2, Button, CentralPanel, Color32, FontDefinitions,
    Id, LayerId, Layout, Order, RichText, ScrollArea, SidePanel, TextStyle, TopBottomPanel, Ui,
    Visuals,
};
use egui_ext::{DroppedFileExt, HoveredFileExt, LightDarkButton};
use egui_notify::Toasts;
use egui_phosphor::{
    add_to_fonts,
    regular::{
        ARROWS_CLOCKWISE, CHART_BAR, CLOUD_ARROW_DOWN, FLOPPY_DISK, INFO, SIDEBAR_SIMPLE, TRASH,
    },
    Variant,
};
use egui_tiles::Tree;
use panes::{Settings, TreeExt};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, fmt::Write, str, time::Duration};
use tracing::{debug, error, info, trace};
use url::Url;

/// IEEE 754-2008
const MAX_PRECISION: usize = 16;

const NOTIFICATIONS_DURATION: Duration = Duration::from_secs(15);

// const DESCRIPTION: &str = "Positional-species and positional-type composition of TAG from mature fruit arils of the Euonymus section species, mol % of total TAG";

const SIZE: f32 = 32.0;

// pub(crate) macro icon($icon:expr,x32) {
//     RichText::new($icon).size(SIZE)
// }
pub(crate) macro icon {
    ($icon:expr, x8) => { RichText::new($icon).size(8.0) },
    ($icon:expr, x16) => { RichText::new($icon).size(16.0) },
    ($icon:expr, x32) => { RichText::new($icon).size(32.0) },
    ($icon:expr, x64) => { RichText::new($icon).size(64.0) }
}

fn custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = custom_visuals(style.visuals);
    ctx.set_style(style);
}

fn custom_visuals<T: BorrowMut<Visuals>>(mut visuals: T) -> T {
    visuals.borrow_mut().collapsing_header_frame = true;
    visuals
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    // Panels
    left_panel: bool,
    // Panes
    tree: Tree<Pane>,
    // Data
    data: Data,
    settings: Settings,

    #[serde(skip)]
    file_dialog: FileDialog,
    // Windows
    #[serde(skip)]
    about: About,
    #[serde(skip)]
    github: Github,
    // Notifications
    #[serde(skip)]
    toasts: Toasts,
}

impl Default for App {
    fn default() -> Self {
        Self {
            left_panel: true,
            tree: Tree::empty("central_tree"),
            data: Data::default(),
            settings: Default::default(),
            toasts: Default::default(),
            file_dialog: Default::default(),
            about: Default::default(),
            github: Default::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext) -> Self {
        // Customize style of egui.
        let mut fonts = FontDefinitions::default();
        add_to_fonts(&mut fonts, Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);
        custom_style(&cc.egui_ctx);

        // return Default::default();
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.storage
            .and_then(|storage| get_value(storage, APP_KEY))
            .unwrap_or_default()
    }

    // pub fn load_configs(&mut self) {
    //     if let Some(promise) = &self.promise {
    //         if let Some(entries) = promise.ready() {
    //             println!("entries: {entries:?}");
    //         } else {
    //             println!("loading");
    //         }
    //     } else {
    //         let request = Request {
    //             headers: Headers::new(&[
    //                 ("Accept", "application/vnd.github+json"),
    //                 (
    //                     "Authorization",
    //                     "Bearer ghp_",
    //                 ),
    //                 ("X-GitHub-Api-Version", "2022-11-28"),
    //             ]),
    //             ..Request::get(
    //                 "https://api.github.com/repos/ippras/utca/contents/configs?recursive=true",
    //             )
    //         };
    //         let (sender, promise) = Promise::new();
    //         self.promise = Some(promise);
    //         fetch(
    //             request,
    //             move |response: Result<Response, String>| match response {
    //                 Ok(response) => {
    //                     info!("Status code: {}", response.status);
    //                     match response.json::<Vec<Entry>>() {
    //                         Ok(entries) => {
    //                             println!("entries: {entries:#?}");
    //                             sender.send(Some(entries));
    //                         }
    //                         Err(error) => {
    //                             error!(%error);
    //                             sender.send(None);
    //                         }
    //                     }
    //                 }
    //                 Err(error) => error!(%error),
    //             },
    //         );
    //     }
    //     // // let toasts = self.toasts.clone();
    //     // fetch(request, move |response: Result<Response, String>| {
    //     //     match response {
    //     //         Ok(response) => {
    //     //             info!("Status code: {}", response.status);
    //     //             match response.json::<Vec<Entry>>() {
    //     //                 Ok(entries) => {
    //     //                     println!("entries: {entries:#?}");
    //     //                 }
    //     //                 Err(error) => {
    //     //                     error!(%error);
    //     //                     return;
    //     //                 }
    //     //             }
    //     //             // if let Ok(json) = response.json::<Vec<Entry>>() {
    //     //             //     let parsed:  = match text.parse() {
    //     //             //         Ok(file) => file,
    //     //             //         Err(error) => {
    //     //             //             error!(%error);
    //     //             //             return;
    //     //             //         }
    //     //             //     };
    //     //             //     println!("parsed: {parsed:?}");
    //     //             // }
    //     //         }
    //     //         Err(error) => error!(%error),
    //     //     }
    //     //     // if let Err(error) = response {
    //     //     //     error!(%error);
    //     //     //     // toasts
    //     //     //     //     .error(format!("Load configs: {error}"))
    //     //     //     //     .set_closable(true)
    //     //     //     //     .set_duration(Some(NOTIFICATIONS_DURATION));
    //     //     // } else {
    //     //     //     info!("Load configs: OK");
    //     //     // }
    //     //     // println!("Status code: {:?}", response.unwrap().status);
    //     // });
    // }
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
                let mut behavior = Behavior {
                    data: &mut self.data,
                    settings: &self.settings,
                    close: None,
                };
                self.tree.ui(&mut behavior, ui);
            });
    }

    // Left panel
    fn left_panel(&mut self, ctx: &egui::Context) {
        SidePanel::left("left_panel")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(0.0))
            .resizable(true)
            .show_animated(ctx, self.left_panel, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    self.settings.ui(ui, &mut self.tree);
                    ui.separator();
                });
                // let mut style = Style::from_egui(&ctx.style());
                // style.tab_bar.fill_tab_bar = true;
                // self.tree.ui(&mut SettingsBehavior, ui);

                // DockArea::new(&mut self.docks.left.state)
                //     .id(Id::new("left_dock"))
                //     .style(style)
                //     .show_inside(
                //         ui,
                //         &mut LeftTabs {
                //             context: &mut self.context,
                //             state: &self.docks.central,
                //         },
                //     );
            });
    }

    // Top panel
    fn top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            bar(ui, |ui| {
                // Left panel
                ui.toggle_value(&mut self.left_panel, icon!(SIDEBAR_SIMPLE, x32))
                    .on_hover_text(titlecase!("left_panel"));
                ui.separator();
                // Light/Dark
                ui.light_dark_button(SIZE);
                ui.separator();
                // Reset
                if ui
                    .button(icon!(TRASH, x32))
                    .on_hover_text(titlecase!("reset_application"))
                    .clicked()
                {
                    *self = Default::default();
                }
                ui.separator();
                if ui
                    .button(icon!(ARROWS_CLOCKWISE, x32))
                    .on_hover_text(titlecase!("reset_gui"))
                    .clicked()
                {
                    ui.memory_mut(|memory| *memory = Default::default());
                }
                ui.separator();
                let mut toggle = |ui: &mut Ui, pane| {
                    let tile_id = self.tree.tiles.find_pane(&pane);
                    if ui
                        .selectable_label(
                            tile_id.is_some_and(|tile_id| self.tree.is_visible(tile_id)),
                            icon!(pane.icon(), x32),
                        )
                        .on_hover_text(pane.title())
                        .clicked()
                    {
                        if let Some(id) = tile_id {
                            self.tree.tiles.toggle_visibility(id);
                        } else {
                            self.tree.insert_pane(pane);
                        }
                    }
                };
                // Configuration
                toggle(ui, Pane::Configuration(Default::default()));
                // Calculation
                toggle(ui, Pane::Calculation(Default::default()));
                // Composition
                toggle(ui, Pane::Composition(Default::default()));
                if ui.button(icon!(CHART_BAR, x32)).clicked() {}
                ui.separator();
                // Load
                if ui.button(icon!(CLOUD_ARROW_DOWN, x32)).clicked() {
                    self.github.toggle();
                }
                // Save
                if ui.button(icon!(FLOPPY_DISK, x32)).clicked() {
                    if let Err(error) = self.data.save("df.utca.ron") {
                        error!(%error);
                    }
                }
                ui.separator();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    // About
                    if ui
                        .add(Button::new(icon!(INFO, x32)))
                        .on_hover_text("About window")
                        .clicked()
                    {
                        self.about.open ^= true;
                    }
                    ui.separator();
                    // Locale
                    ui.locale_button().on_hover_text(titlecase!("language"));
                });

                // if ui.button("Cl").clicked() {
                //     let mut children = vec![self.tree.tiles.insert_pane(Default::default())];
                //     if let Some(root) = self.tree.root {
                //         children.push(root);
                //     }
                //     self.tree.root = Some(self.tree.tiles.insert_vertical_tile(children));
                // }
                // if ui.button("Cm").clicked() {
                //     let mut children = vec![self.tree.tiles.insert_pane(Default::default())];
                //     if let Some(root) = self.tree.root {
                //         children.push(root);
                //     }
                //     self.tree.root = Some(self.tree.tiles.insert_vertical_tile(children));
                // }

                //     ui.visuals_mut().button_frame = false;
                //     global_dark_light_mode_switch(ui);
                //     ui.separator();
                //     if ui
                //         .add(Button::new(icon!("🗑")))
                //         .on_hover_text("Reset data")
                //         .clicked()
                //     {
                //         *self = Self {
                //             docks: take(&mut self.docks),
                //             ..Default::default()
                //         };
                //     }
                //     // Reset gui
                //     if ui
                //         .add(Button::new(icon!("🔃")))
                //         .on_hover_text("Reset gui")
                //         .clicked()
                //     {
                //         ui.with_visuals(|ui, _| ui.memory_mut(|memory| *memory = Default::default()));
                //     }
                //     // Organize windows
                //     if ui
                //         .add(Button::new(icon!("▣")))
                //         .on_hover_text("Organize windows")
                //         .clicked()
                //     {
                //         self.docks.left = Default::default();
                //         self.docks.central = Default::default();
                //         ui.ctx().memory_mut(|memory| memory.reset_areas());
                //     }
                //     ui.separator();
                //     #[derive(Clone, Copy, Debug, PartialEq)]
                //     enum Format {
                //         Toml,
                //     }
                //     // Import file
                //     if ui
                //         .button("G📤")
                //         .on_hover_text("Load config files")
                //         .clicked()
                //     {
                //         self.github.open ^= true;
                //     }
                //     // Import file
                //     if ui.button("📤").on_hover_text("Import file").clicked() {
                //         if let Err(error) = self.import() {
                //             error!(?error);
                //         }
                //     }
                //     if let Some(bytes) = self.file_dialog.take() {
                //         match String::from_utf8(bytes) {
                //             Err(error) => {
                //                 error!(%error);
                //                 return;
                //             }
                //             Ok(content) => {
                //                 trace!(?content);
                //                 let parsed = match content.parse() {
                //                     Ok(file) => file,
                //                     Err(error) => {
                //                         error!(%error);
                //                         return;
                //                     }
                //                 };
                //                 trace!(?parsed);
                //                 self.context.init(parsed);
                //             }
                //         }
                //     }
                //     let mut format = Format::Toml;
                //     ComboBox::from_id_source("export")
                //         .selected_text(format!("{format:?}"))
                //         .show_ui(ui, |ui| {
                //             ui.selectable_value(&mut format, Format::Toml, "Toml");
                //         });
                //     // Export file
                //     if ui.button("📥").on_hover_text("Export file").clicked() {
                //         if let Err(error) = self.export() {
                //             error!(?error);
                //         }
                //     }
                //     ui.separator();
                //     // Settings
                //     let tab = LeftTab::Settings;
                //     let checked = self.docks.left.state.find_tab(&tab).is_some();
                //     if ui
                //         .selectable_label(checked, tab.sign())
                //         .on_hover_text(tab.to_string())
                //         .clicked()
                //     {
                //         self.docks.left.toggle(tab);
                //     }
                //     // Files
                //     let tab = LeftTab::Files;
                //     let checked = self.docks.left.state.find_tab(&tab).is_some();
                //     let text = if checked { "📂" } else { "📁" };
                //     if ui
                //         .selectable_label(checked, icon!(text))
                //         .on_hover_text(tab.to_string())
                //         .clicked()
                //     {
                //         self.docks.left.toggle(tab);
                //     }
                //     ui.separator();
                //     // Configuration
                //     let tab = CentralTab::Configuration;
                //     let found = self.docks.central.find_tab(&tab);
                //     if ui
                //         .selectable_label(found.is_some(), tab.sign())
                //         .on_hover_text(tab.to_string())
                //         .clicked()
                //     {
                //         if let Some(index) = found {
                //             self.docks.central.remove_tab(index);
                //         } else {
                //             self.docks.central.push_to_focused_leaf(tab);
                //         }
                //     }
                //     // Calculation
                //     let tab = CentralTab::Calculation;
                //     let found = self.docks.central.find_tab(&tab);
                //     if ui
                //         .selectable_label(found.is_some(), tab.sign())
                //         .on_hover_text(tab.to_string())
                //         .clicked()
                //     {
                //         if let Some(index) = found {
                //             self.docks.central.remove_tab(index);
                //         } else {
                //             self.docks.central.push_to_focused_leaf(tab);
                //         }
                //     }
                //     // Filtration
                //     ui.selectable_label(false, "🔎").on_hover_text("Filtration");
                //     // Composition
                //     let tab = CentralTab::Composition;
                //     let found = self.docks.central.find_tab(&tab);
                //     if ui
                //         .selectable_label(found.is_some(), tab.sign())
                //         .on_hover_text(tab.to_string())
                //         .clicked()
                //     {
                //         if let Some(index) = found {
                //             self.docks.central.remove_tab(index);
                //         } else {
                //             self.docks.central.push_to_focused_leaf(tab);
                //         }
                //     }
                //     // Comparison
                //     let tab = CentralTab::Comparison;
                //     let found = self.docks.central.find_tab(&tab);
                //     if ui
                //         .selectable_label(found.is_some(), tab.sign())
                //         .on_hover_text(tab.to_string())
                //         .clicked()
                //     {
                //         if let Some(index) = found {
                //             self.docks.central.remove_tab(index);
                //         } else {
                //             self.docks.central.push_to_focused_leaf(tab);
                //         }
                //     }
                //     // Visualization
                //     let tab = CentralTab::Visualization;
                //     let found = self.docks.central.find_tab(&tab);
                //     if ui
                //         .selectable_label(found.is_some(), tab.sign())
                //         .on_hover_text(tab.to_string())
                //         .clicked()
                //     {
                //         if let Some(index) = found {
                //             self.docks.central.remove_tab(index);
                //         } else {
                //             self.docks.central.push_to_focused_leaf(tab);
                //         }
                //     }
                //     ui.separator();
                //     // About
                //     ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                //         if ui
                //             .add(Button::new(icon!("ℹ")))
                //             .on_hover_text("About window")
                //             .clicked()
                //         {
                //             self.about.open ^= true;
                //         }
                //         ui.separator();
                //     });
            });
        });
    }
}

// Windows
impl App {
    fn windows(&mut self, ctx: &egui::Context) {
        self.about.window(ctx);
        self.github.window(ctx);
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
            for dropped in dropped_files {
                trace!(?dropped);
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
                let data_frame: DataFrame = match ron::de::from_str(&content) {
                    Ok(data_frame) => data_frame,
                    Err(error) => {
                        error!(%error);
                        self.toasts
                            .error(format!("{}: {error}", dropped.display()))
                            .set_closable(true)
                            .set_duration(Some(NOTIFICATIONS_DURATION));
                        continue;
                    }
                };
                trace!(?data_frame);
                self.data.fatty_acids = data_frame;
            }
        }
    }

    // fn paste(&mut self, ctx: &egui::Context) {
    //     if !ctx.memory(|memory| memory.focused().is_some()) {
    //         ctx.input(|input| {
    //             for event in &input.raw.events {
    //                 if let Event::Paste(paste) = event {
    //                     if let Err(error) = self.parse(paste) {
    //                         error!(?error);
    //                         self.toasts
    //                             .error(error.to_string().chars().take(64).collect::<String>())
    //                             .set_duration(Some(Duration::from_secs(5)))
    //                             .set_closable(true);
    //                     }
    //                 }
    //             }
    //         });
    //     }
    // }

    // fn parse(&mut self, paste: &str) -> Result<()> {
    //     use crate::parsers::whitespace::Parser;
    //     let parsed = Parser::parse(paste)?;
    //     debug!(?parsed);
    //     for parsed in parsed {
    //         // self.docks.central.tabs.input.add(match parsed {
    //         //     Parsed::All(label, (c, n), tag, dag, mag) => FattyAcid {
    //         //         label,
    //         //         formula: ether!(c as usize, n as usize),
    //         //         values: [tag, dag, mag],
    //         //     },
    //         //     // Parsed::String(label) => Row { label, ..default() },
    //         //     // Parsed::Integers(_) => Row { label, ..default() },
    //         //     // Parsed::Float(tag) => Row { label, ..default() },
    //         //     _ => unimplemented!(),
    //         // })?;
    //         // self.config.push_row(Row {
    //         //     acylglycerols,
    //         //     label:  parsed.,
    //         //     ether: todo!(),
    //         //     // ..default()
    //         // })?;
    //     }
    //     // let mut rows = Vec::new();
    //     // for row in paste.split('\n') {
    //     //     let mut columns = [0.0; COUNT];
    //     //     for (j, column) in row.split('\t').enumerate() {
    //     //         ensure!(j < COUNT, "Invalid shape, columns: {COUNT} {j}");
    //     //         columns[j] = column.replace(',', ".").parse()?;
    //     //     }
    //     //     rows.push(columns);
    //     // }
    //     // for acylglycerols in rows {
    //     //     self.config.push_row(Row {
    //     //         acylglycerol: acylglycerols,
    //     //         ..default()
    //     //     })?;
    //     // }
    //     Ok(())
    // }

    // fn export(&self) -> Result<(), impl Debug> {
    //     let content = to_string(&TomlParsed {
    //         name: self.context.state.entry().meta.name.clone(),
    //         fatty_acids: self.context.state.entry().fatty_acids(),
    //     })
    //     .unwrap();
    //     self.file_dialog
    //         .save(
    //             &format!("{}.toml", self.context.state.entry().meta.name),
    //             content,
    //         )
    //         .unwrap();
    //     Ok::<_, ()>(())
    // }

    // fn import(&mut self) -> Result<(), impl Debug> {
    //     self.file_dialog.load()
    // }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Pre update
        self.panels(ctx);
        self.windows(ctx);
        self.notifications(ctx);
        // Post update
        self.drag_and_drop(ctx);
        // self.paste(ctx);
    }
}

mod computers;
mod data;
mod panes;
mod windows;
