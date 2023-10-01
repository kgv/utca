use self::{files::Files, settings::Settings};
use super::CentralTab;
use crate::app::Context;
use egui::{Ui, WidgetText};
use egui_dock::{NodeIndex, TabViewer, Tree};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

/// Left dock
#[derive(Debug, Default, Deserialize, Serialize)]
pub(in crate::app) struct Dock {
    pub(in crate::app) tree: Tree<Tab>,
}

impl Dock {
    pub(in crate::app) fn toggle(&mut self, tab: Tab) {
        match self.tree.find_tab(&tab) {
            Some(index) => {
                self.tree.remove_tab(index);
            }
            None if self.tree.is_empty() => {
                self.tree.push_to_first_leaf(tab);
            }
            None => {
                self.tree.split_below(NodeIndex::root(), 0.5, vec![tab]);
            }
        }
    }
}

// impl Default for LeftDock {
//     fn default() -> Self {
//         let mut tree = Tree::new(vec![Tab::Settings]);
//         tree.split_below(NodeIndex::root(), 0.5, vec![Tab::Files]);
//         Self {
//             tree,
//             tabs: Default::default(),
//         }
//     }
// }

impl Deref for Dock {
    type Target = Tree<Tab>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl DerefMut for Dock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

/// Left tab
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

/// Left tabs
#[derive(Debug)]
pub(in crate::app) struct Tabs<'a> {
    pub(in crate::app) context: &'a mut Context,
    pub(in crate::app) tree: &'a Tree<CentralTab>,
}

impl TabViewer for Tabs<'_> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.to_string().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            Tab::Files => Files::view(ui, self.context),
            Tab::Settings => Settings::new(self.context, self.tree).view(ui),
        }
    }
}

mod files;
mod settings;