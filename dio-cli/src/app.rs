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

const TITLES: [&str; 5] = ["Home", "Facts", "Principles", "Input", "Popup"];

// ----------------------------------------------------------------------------

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<&'a str>,
    pub show_help_popup: bool,
    pub progress: f64,
    pub facts: HashMap<String, String>,
    pub principles: HashMap<String, String>,
    pub shortcuts: StatefulList<&'a str>,
    /// Current value of the input box.
    pub input: String,
    /// Current input mode.
    pub input_mode: InputMode,
    /// History of recorded messages.
    pub messages: Vec<String>,
    /// Enhanced TUI graphics. More CPU usage.
    pub enhanced_graphics: bool,
}

/// Facts page, Principles page. Each have lists of condensed titles.
/// When one sleects a item, it expands. or opens a dialog buffer.

impl<'a> App<'a> {
    pub fn new(title: &'a str, db: DB, enhanced_graphics: bool) -> Self {
        Self {
            title,
            should_quit: false,
            tabs: TabsState::new(TITLES.to_vec()),
            show_help_popup: true,
            progress: 0f64,
            facts: db.facts,
            principles: db.principles,
            shortcuts: StatefulList::with_items(LIST_SHORTCUTS.to_vec()),
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
        self.shortcuts.previous();
    }
    pub fn on_down(&mut self) {
        self.shortcuts.next();
    }
}

// ----------------------------------------------------------------------------

pub struct TabsState<T> {
    /// Title of the tab.
    // pub titles: [&'a str],
    pub titles: Vec<T>,
    /// Index or location of the tab in the app.
    pub index: usize,
}

impl<T> TabsState<T> {
    pub fn new(titles: Vec<T>) -> Self {
        Self {
            titles,
            index: 0usize,
        }
    }

    // fn new(titles: Vec<&'a str>) -> Self {
    //     Self {
    //         titles: TITLES,
    //         index: 0usize,
    //     }
    // }

    fn next(&mut self) {
        let len_total = self.titles.len();
        self.index = (self.index + 1usize) % len_total
    }

    fn previous(&mut self) {
        if self.index > 0usize {
            self.index -= 1usize;
        } else {
            let len_total = self.titles.len();
            self.index = len_total - 1usize;
        }
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
