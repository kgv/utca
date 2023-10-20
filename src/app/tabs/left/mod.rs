use self::{files::Files, settings::Settings};
use super::CentralTab;
use crate::app::{view::View, Context};
use egui::{Ui, WidgetText};
use egui_dock::{DockState, NodeIndex, TabViewer};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

/// Left dock
#[derive(Debug, Deserialize, Serialize)]
pub(in crate::app) struct Dock {
    pub(in crate::app) state: DockState<Tab>,
}

impl Dock {
    pub(in crate::app) fn toggle(&mut self, tab: Tab) {
        match self.state.find_tab(&tab) {
            Some(index) => {
                self.state.remove_tab(index);
            }
            None if self.state.main_surface().num_tabs() == 0 => {
                self.state.push_to_first_leaf(tab);
            }
            None => {
                self.state
                    .main_surface_mut()
                    .split_below(NodeIndex::root(), 0.5, vec![tab]);
            }
        }
    }
}

impl Default for Dock {
    fn default() -> Self {
        Self {
            state: DockState::new(Vec::new()),
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
    type Target = DockState<Tab>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Dock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
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
    pub(in crate::app) state: &'a DockState<CentralTab>,
}

impl TabViewer for Tabs<'_> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.to_string().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            Tab::Files => Files::new(self.context).view(ui),
            Tab::Settings => Settings::new(self.context, self.state).view(ui),
        }
    }
}

mod files;
mod settings;
