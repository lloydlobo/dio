//! ['app'] module holds `App` structure.

use crate::db::DB;
use std::collections::HashMap;
use tui::widgets::ListState;

/// Navigation and other app shortcuts.
const LIST_SHORTCUTS: [&str; 5] = [
    "q - Exit",
    "? - Help Popup",
    "Esc - unselect",
    "Left - Previous tab",
    "Right - Next tab",
];

// const TITLES: [&str; 4] = ["Home", "Facts", "Principles", "Input"];
const TAB_TITLES: [TabMode; 4] = [
    TabMode::Home(0usize, "Home"),
    TabMode::Facts(1usize, "Facts"),
    TabMode::Principles(2usize, "Principles"),
    TabMode::Input(3usize, "Input"),
];

/// Index of tab in application and title.
#[derive(Debug, Clone)]
pub enum TabMode<'a> {
    Home(usize, &'a str),
    Facts(usize, &'a str),
    Principles(usize, &'a str),
    Input(usize, &'a str),
}

// ----------------------------------------------------------------------------

/// The applications state.
pub struct App<'a> {
    /// Title of the application.
    pub title: &'a str,
    /// To quit or not to quit. Activated with 'q'.
    pub should_quit: bool,
    /// Tabs or navigation routes of the app.
    pub tabs: TabsState<TabMode<'a>>,
    /// '?' activates the help popup modal.
    pub show_help_popup: bool,
    /// The `tick_rate' of the application.
    pub progress: f64,
    /// List from database.
    pub facts: HashMap<String, String>,
    /// List from database.
    pub principles: HashMap<String, String>,
    /// List of facts.
    pub list_facts: StatefulList<&'a str>,
    /// List of principles.
    pub list_principles: StatefulList<&'a str>,
    /// List of keys associated to tabs.
    pub list_tabs: StatefulList<&'a str>,
    /// List of keys associated to action.
    pub list_help: StatefulList<&'a str>,
    /// Current value of the input box.
    pub input: String,
    /// Current input mode.
    pub input_mode: InputMode,
    /// History of recorded messages.
    pub messages: Vec<String>,
    /// Enhanced TUI graphics. More CPU usage.
    pub enhanced_graphics: bool,
}

fn get_map_val(hash: &HashMap<String, String>) -> Vec<&str> {
    hash.iter()
        .map(|f| f.1.as_str())
        .collect::<Vec<_>>()
        .to_vec()
}

/// Facts page, Principles page. Each have lists of condensed titles.
/// When one selects a item, it expands. or opens a dialog buffer.

impl<'a> App<'a> {
    pub fn new(title: &'a str, db: &'a DB, enhanced_graphics: bool) -> Self {
        let l_tabs: Vec<&str> = TAB_TITLES
            .iter()
            .map(|tab| match tab {
                TabMode::Home(_, t)
                | TabMode::Facts(_, t)
                | TabMode::Principles(_, t)
                | TabMode::Input(_, t) => *t,
            })
            .collect();

        Self {
            title,
            should_quit: false,
            tabs: TabsState::new(TAB_TITLES.to_vec()),
            show_help_popup: false,
            progress: 0f64,
            facts: db.facts.to_owned(),
            principles: db.principles.to_owned(),
            list_facts: StatefulList::with_items(get_map_val(&db.facts)),
            list_principles: StatefulList::with_items(get_map_val(&db.principles)),
            list_tabs: StatefulList::with_items(l_tabs),
            list_help: StatefulList::with_items(LIST_SHORTCUTS.to_vec()),
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::<String>::new(),
            enhanced_graphics,
        }
    }
    pub fn on_tick(&mut self) {
        self.progress += 0.001f64; // Increment.
        if self.progress > 1f64 {
            self.progress = 0f64; // Reset.
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            '?' => {
                self.show_help_popup = !self.show_help_popup;
            }
            _ => {}
        }
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }
    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_up(&mut self) {
        if self.show_help_popup {
            self.list_help.previous();
        } else {
            match self.current_tab_mode() {
                TabMode::Home(_, _) => self.list_tabs.previous(),
                TabMode::Facts(_, _) => self.list_facts.previous(),
                TabMode::Principles(_, _) => self.list_principles.previous(),
                TabMode::Input(_, _) => {}
            };
        }
    }
    pub fn on_down(&mut self) {
        if self.show_help_popup {
            self.list_help.next();
        } else {
            match self.current_tab_mode() {
                TabMode::Home(_, _) => self.list_tabs.next(),
                TabMode::Facts(_, _) => self.list_facts.next(),
                TabMode::Principles(_, _) => self.list_principles.next(),
                TabMode::Input(_, _) => {}
            };
        }
    }

    /// If we are on current tab then get that tab and
    /// apply key press action relevant to that tab only.
    fn current_tab_mode(&mut self) -> TabMode {
        self.tabs.titles.to_vec()[self.tabs.index].to_owned()
    }

    /// Go to the tab index of associated tab titles in a list when selected.
    /// Using it with `KeyCode::Enter` in `InputMode::Normal`.
    pub fn jump_to_tab(&mut self) {
        if let Some(index) = self.list_tabs.state.selected() {
            self.tabs.index = index
        }
    }
}

// ----------------------------------------------------------------------------
//

pub struct TabsState<T> {
    /// Title of the tab.
    // pub titles: [&'a str],
    pub titles: Vec<T>,
    /// Index or location of the tab in the app.
    pub index: usize,
}

impl<T> TabsState<T> {
    pub fn new(tabs: Vec<T>) -> Self {
        Self {
            titles: tabs,
            index: 0usize,
        }
    }

    fn next(&mut self) {
        let len_total = self.titles.len();
        self.index = (self.index + 1usize) % len_total;
    }

    fn previous(&mut self) {
        if self.index > 0usize {
            self.index -= 1usize;
        } else {
            let len_total = self.titles.len();
            self.index = len_total - 1usize;
        }
    }

    pub fn cur_list_item_idx(&mut self) -> usize {
        self.index
    }
}

// ----------------------------------------------------------------------------

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn next(&mut self) {
        let index: usize = match self.state.selected() {
            Some(i) => {
                if i >= (self.items.len() - 1usize) {
                    0usize
                } else {
                    i + 1usize
                }
            }
            None => 0usize,
        };
        self.state.select(Some(index));
    }

    pub fn previous(&mut self) {
        let index = match self.state.selected() {
            Some(i) => {
                if i == 0usize {
                    self.items.len() - 1usize
                } else {
                    i - 1usize
                }
            }
            None => 0usize,
        };
        self.state.select(Some(index));
    }

    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

// ----------------------------------------------------------------------------

/// User input mode like `vim`'s insert, visual, normal mode.
pub enum InputMode {
    /// No input recorded.
    Normal,

    /// Allow input to be recorded.
    Editing,
}

// ----------------------------------------------------------------------------

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_selects_next_tab_state() {}
} */

// let tabs = self .tabs .titles .iter() .map(|t| t.clone()) .collect::<Vec<_>>();
// 0 => self.facts.on_up(),
// 1 => self.principles.on_up(),
// 2 => self.shortcuts.on_up(),
// _ => {}
// TabMode::Home(, )
//
// match self
//     .tabs
//     .titles
//     .iter()
//     .map(|tab| match tab {
//         TabMode::Home(_, _) => todo!(),
//         TabMode::Principles(_, _) => todo!(),
//         TabMode::Facts(_, _) => todo!(),
//         TabMode::Input(_, _) => todo!(),
//     })
//     .collect::<Vec<_>>()[self.tabs.index]
// {
//     TabMode::Home(_, _) => todo!(),
//     TabMode::Principles(_, _) => todo!(),
//     TabMode::Facts(_, _) => todo!(),
//     TabMode::Input(_, _) => todo!(),
// };
