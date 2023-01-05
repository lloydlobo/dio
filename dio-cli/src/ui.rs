//! [`ui`] implements TUI.

use crate::app::{self, App};
use std::{
    collections::HashMap,
    fmt::{Binary, Debug, LowerHex, Octal, UpperHex},
};
use tui::{
    self,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Paragraph, Tabs, Wrap},
};
use unicode_width::UnicodeWidthStr; // Determine displayed width of char and str types according to Unicode Standard Annex #11 rules.

// ----------------------------------------------------------------------------

/// Draw the app tui.
///
/// # Arguments
///
/// * `f` - A mutable reference to a `tui::Frame`.
/// * `app` - A mutable reference to an `App`.
pub fn draw<B>(f: &mut tui::Frame<B>, app: &mut App)
where
    B: Backend,
{
    // Two chunks [0->Len 3, 1->Min 0]. 0 for tab, 1 for body.
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3), // Tabs.
                Constraint::Min(0),    // Body.
                Constraint::Length(8), // Preview.
                Constraint::Length(3), // Tick rate progress.
            ]
            .as_ref(),
        )
        .split(f.size());

    // Set the tabs of the app menu navigation.
    let titles: Vec<Spans> = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Cyan))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Dio"),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .select(app.tabs.index);
    f.render_widget(tabs.clone(), chunks[index_from(Chunk::Tabs)]);

    // Draw the selected tab (page) and navigate to it.
    match app.tabs.index {
        0 => draw_tab_0_home(f, app, chunks[(index_from(Chunk::Body))]),
        1 => draw_tab_1_facts(f, app, chunks[(index_from(Chunk::Body))]),
        2 => draw_tab_2_principles(f, app, chunks[(index_from(Chunk::Body))]),
        3 => draw_tab_3_inputs(f, app, chunks[(index_from(Chunk::Body))]),
        _ => {}
    }

    // Temporary rendering gauge. Add hover selected preview here. line in input messages.
    f.render_widget(tabs, chunks[index_from(Chunk::Preview)]);

    // Track tick rate progress of the app. Resets again after a while.
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Span::styled(
                    "Tick rate",
                    Style::default().add_modifier(Modifier::ITALIC),
                ))
                .style(Style::default().add_modifier(modifier(
                    app.progress,
                    Modifier::ITALIC,
                    Modifier::BOLD,
                )))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .gauge_style(
            Style::default()
                .fg(Color::LightGreen)
                .bg(Color::Reset)
                .add_modifier(modifier(app.progress, Modifier::BOLD, Modifier::ITALIC)),
        )
        .ratio(app.progress) // .percent(app.progress) // for u16.
        .use_unicode(true);
    f.render_widget(gauge, chunks[index_from(Chunk::Gauge)]);

    // Help Popup uses the full layout and draws over everything.
    let rect = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .split(f.size());
    draw_help_popup(f, app, rect[0]);
}

// ----------------------------------------------------------------------------

/// Draws tui popup.
///
/// # Arguments
///
/// * `f` - A mutable reference to a tui::Frame.
/// * `app` - A mutable reference to an App.
/// * `area` - A Rect.
///
fn draw_help_popup<B>(f: &mut tui::Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let items: Vec<ListItem> = app
        .shortcuts
        .items
        .iter()
        .map(|item| {
            ListItem::new(vec![Spans::from(item.to_string())])
                .style(Style::default().fg(Color::White).bg(Color::Reset))
        })
        .collect();
    let items = List::new(items)
        .block(
            Block::default()
                .title("Shortcuts")
                .border_style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    if app.show_help_popup {
        let area: Rect = centered_rect(60u16, 20u16, area); // `60, 20, size`.
        f.render_widget(Clear, area); // `Clear` - A widget to clear/reset a certain area to allow overdrawing (e.g. for popups).
        f.render_stateful_widget(items, area, &mut app.shortcuts.state);
    }
}

// ----------------------------------------------------------------------------

/// HOME
/// Iterate through all elements in the `shortcuts` app.
/// Create a `List` from all list items and highlight the currently selected one.
fn draw_tab_0_home<B>(f: &mut tui::Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    f.render_widget(
        Block::default()
            .title(Span::styled(
                "Home",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default()),
        area,
    );

    let inline_center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25u16),
                Constraint::Percentage(50u16), // To be picked.
                Constraint::Percentage(25u16),
            ]
            .as_ref(),
        )
        .split(area);
    let grid_center = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10u16),
                Constraint::Percentage(20u16), // logo.
                Constraint::Percentage(10u16),
                Constraint::Percentage(50u16), // body.
                Constraint::Percentage(10u16),
            ]
            .as_ref(),
        )
        .split(inline_center[1usize]);

    let items: Vec<ListItem> = app
        .shortcuts
        .items
        .iter()
        .map(|item| {
            ListItem::new(vec![Spans::from(item.to_string())])
                .style(Style::default().fg(Color::White).bg(Color::Reset))
        })
        .collect();
    let items = List::new(items)
        .block(
            Block::default()
                .title("Shortcuts")
                .border_style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(items, grid_center[3usize], &mut app.shortcuts.state);

    let paragraph = Paragraph::new(BANNER)
        .block(Block::default())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, grid_center[1usize]);
}

/// FACTS
fn draw_tab_1_facts<B>(f: &mut tui::Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let facts: HashMap<String, String> = app.facts.clone();
    let len = facts.len();
    let mut text = Vec::<Spans>::with_capacity(len);
    (1..=len).for_each(|id: usize| {
        let fact: String = facts
            .get(&id.to_string())
            .expect("Failed to get fact from facts map. This should never happen.")
            .to_owned();
        let fact: &str = fact.split_terminator('.').collect::<Vec<_>>()[0];
        text.push(Spans::from(format!("{id:0>2}. {fact}.")));
        // text.push(Spans::from(Span::raw("")));
    });
    let block = Block::default()
        .title(Span::styled(
            "Facts",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default());
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// PRINCIPLES
fn draw_tab_2_principles<B>(f: &mut tui::Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let principles: HashMap<String, String> = app.principles.clone();
    let len = principles.len();
    let mut text = Vec::<Spans>::with_capacity(len);
    (1..=len).for_each(|id: usize| {
        let principle: String = principles
            .get(&id.to_string())
            .expect("Failed to get principle from principles map. This should never happen.")
            .to_owned();
        let principle: &str = principle.split_terminator('.').collect::<Vec<_>>()[0];
        // [See for formatting digits](https://doc.rust-lang.org/std/fmt/index.html#syntax)
        text.push(Spans::from(format!("{id:0>2}. {principle}.",)));
    });
    let block = Block::default()
        .title(Span::styled(
            "Principles",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default());
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

/// USER INPUTS
///
/// This is a very simple example:
///   * A input box always focused. Every character you type is registered
///   here
///   * Pressing Backspace erases a character
///   * Pressing Enter pushes the current input in the history of previous
///   messages
///
/// [Reference](https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs)
fn draw_tab_3_inputs<B>(f: &mut tui::Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .margin(2u16)
        .constraints(
            [
                Constraint::Length(1u16),
                Constraint::Length(3u16),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(area);

    let (msg, style): (Vec<Span>, Style) = match app.input_mode {
        app::InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        app::InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message."),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0usize]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            app::InputMode::Normal => Style::default(),
            app::InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Input"),
        );
    f.render_widget(input, chunks[1usize]);

    match app.input_mode {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here.
        app::InputMode::Normal => {}
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering.
        app::InputMode::Editing => f.set_cursor(
            // Put cursor past the end of the input text.
            chunks[1].x + app.input.width() as u16 + 1u16,
            // Move one line down, from the border to the input line.
            chunks[1usize].y + 1u16,
        ),
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages).block(
        Block::default()
            .title("Messages")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    f.render_widget(messages, chunks[2usize]);
}

// ----------------------------------------------------------------------------

/// Enum for getting the position of a widget in a layout to draw in.
///
/// A chunk is usually a `tui::Rect` split from `Layout`.
#[derive(Debug)]
enum Chunk {
    /// Index for `tui::widgets::Tabs` used for navigation.
    Tabs,
    /// Index for main body layout.
    Body,
    /// Index for preview of selected main items.
    Preview,
    /// Index for `tui::widget::gauge` for tick_rate progress.
    Gauge,
}

/// Returns the index of the given chunk.
///
/// # Examples
///
/// ```
/// use chunk::Chunk;
///
/// assert_eq!(index_from(Chunk::Tabs), 0usize);
/// assert_eq!(index_from(Chunk::Body), 1usize);
/// assert_eq!(index_from(Chunk::Preview), 2usize);
/// assert_eq!(index_from(Chunk::Gauge), 3usize);
/// ```
fn index_from(chunk: Chunk) -> usize {
    match chunk {
        Chunk::Tabs => 0usize,
        Chunk::Body => 1usize,
        Chunk::Preview => 2usize,
        Chunk::Gauge => 3usize,
    }
}

/// Returns a centered rectangle that uses a certain percentage of the available rect `rect: Rect`.
///
/// Use with popups.
///
/// # Arguments
///
/// * `percent_x` - The percentage of the width of the rectangle.
/// * `percent_y` - The percentage of the height of the rectangle.
/// * `rect` - The rectangle to be centered.
///
/// # Example
///
/// ```
/// use tui::layout::{Constraint, Direction, Layout, Rect};
///
/// let rect = Rect::new(0, 0, 100, 100);
/// let centered_rect = centered_rect(50, 50, rect);
/// ```
///
/// [Reference](https://github.com/fdehau/tui-rs/blob/master/examples/popup.rs)
fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
    let popup_layout: Vec<Rect> = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100u16 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100u16 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(rect);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100u16 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100u16 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

/// This function takes a progress `ratio` and two modifiers and returns one of the modifiers based on
/// the progress ratio.
fn modifier<T>(ratio: f64, m_1: T, m_2: T) -> T
where
    T: Debug + LowerHex + UpperHex + Octal + Binary,
{
    assert!(
        (ratio * 100f64) as u16 <= 100,
        "Progress ratio percentages should be between 0 and 100 inclusively."
    );

    // Simulate a second by speeding up tick_rate by 2.
    if ((ratio * 200f64) as u16) % 2u16 == 0u16 {
        m_1
    } else {
        m_2
    }
}

// ----------------------------------------------------------------------------

/// [`banner`] is the CLI banner that appears at startup.
///
/// [Credits](https://fsymbols.com/generators/carty/).
pub const BANNER: &str = "
██████╗░██╗░█████╗
██╔══██╗██║██╔══██╗
██║░░██║██║██║░░██║
██║░░██║██║██║░░██║
██████╔╝██║╚█████╔╝
╚═════╝░╚═╝░╚════╝░
";

// ----------------------------------------------------------------------------

/* /// Draws the popup.
///
/// # Arguments
///
/// * `f` - A mutable reference to the frame.
/// * `app` - A mutable reference to the application.
/// * `area` - The area to draw the popup in.
///
/// # Examples
///
/// ```
/// use tui::widgets::{Block, Borders};
///
/// let block = Block::default().title("Popup").borders(Borders::ALL);
/// ```
fn draw_popup<B>(f: &mut tui::Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // let size: Rect = f.size();
    let chunks: Vec<Rect> = Layout::default()
        .constraints([Constraint::Percentage(20u16), Constraint::Percentage(80u16)].as_ref())
        .split(area); // `split(size)`.

    let text = if app.show_help_popup {
        "Press ? to close the popup"
    } else {
        "Press ? to show the popup"
    };
    let paragraph = Paragraph::new(Span::styled(
        text,
        Style::default().add_modifier(Modifier::SLOW_BLINK),
    ))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0usize]);

    let block = Block::default()
        .title("Content")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default())
        .style(Style::default().bg(Color::Blue));
    f.render_widget(block, chunks[1usize]);

    if app.show_help_popup {
        let block = Block::default()
            .title("Popup")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let area: Rect = centered_rect(60u16, 20u16, area); // `60, 20, size`.

        // `Clear` - A widget to clear/reset a certain area to allow overdrawing (e.g. for popups).
        f.render_widget(Clear, area);
        f.render_widget(block, area);
    }
} */
