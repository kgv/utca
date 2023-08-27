pub(in crate::app) use self::{files::Files, settings::Settings};

use egui::{Ui, WidgetText};
use egui_dock::{NodeIndex, TabViewer, Tree};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Left dock
#[derive(Debug, Deserialize, Serialize)]
pub(in crate::app) struct LeftDock {
    pub(in crate::app) tree: Tree<Tab>,
    pub(in crate::app) tabs: Tabs,
}

impl Default for LeftDock {
    fn default() -> Self {
        let mut tree = Tree::new(vec![Tab::Settings]);
        tree.split_below(NodeIndex::root(), 0.5, vec![Tab::Files]);
        Self {
            tree,
            tabs: Default::default(),
        }
    }
}

/// Left dock tab
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Tab {
    Files,
    Settings,
}

impl Display for Tab {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Files => f.write_str("📂 Files"),
            Self::Settings => f.write_str("⚙ Settings"),
        }
    }
}

/// Left dock tabs
#[derive(Debug, Default, Deserialize, Serialize)]
pub(in crate::app) struct Tabs {
    pub(in crate::app) files: Files,
    pub(in crate::app) settings: Settings,
}

impl TabViewer for Tabs {
    type Tab = Tab;

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        match *tab {
            Tab::Files => {}
            Tab::Settings => self.settings.visible = false,
        }
        true
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.to_string().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            Tab::Files => self.files.content(ui),
            Tab::Settings => self.settings.content(ui),
        }
    }
}

mod files;
mod settings;
