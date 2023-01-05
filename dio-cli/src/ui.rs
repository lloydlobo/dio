//! [`ui`] implements TUI.

use crate::app::App;
use std::collections::HashMap;
use tui::{
    self,
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
};

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
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size()); // Two chunks [0->Len 3, 1->Min 0]. 0 for tab, 1 for body.
    let titles: Vec<Spans> = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Cyan))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0usize]);

    // Draw the selected tab (page) and navigate to it.
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1usize]),
        1 => draw_second_tab(f, app, chunks[1usize]),
        2 => draw_third_tab(f, app, chunks[1usize]),
        _ => {}
    }
}

// ----------------------------------------------------------------------------

/// HOME
/// Iterate through all elements in the `shortcuts` app.
/// Create a `List` from all list items and highlight the currently selected one.
fn draw_first_tab<B>(f: &mut tui::Frame<B>, app: &mut App, area: tui::layout::Rect)
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
                .border_style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title("Shortcuts"),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(items, area, &mut app.shortcuts.state);
}

/// FACTS
fn draw_second_tab<B>(f: &mut tui::Frame<B>, app: &mut App, area: tui::layout::Rect)
where
    B: Backend,
{
    let facts: HashMap<String, String> = app.facts.clone();
    let len = facts.len();
    let mut text = Vec::<Spans>::with_capacity(len);
    (1..=len).for_each(|id: usize| {
        text.push(Spans::from(format!(
            "{id}. {fact}",
            fact = facts
                .get(&id.to_string())
                .expect("Failed to get fact from facts map. This should never happen.")
                .to_owned(),
        )));
    });
    let block = Block::default()
        .title(Span::styled(
            "Facts",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default());
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);

    // let binding = gen_str(2usize);
    // let block = draw_footer(binding.as_str());

    // f.render_widget(block, area);
}

/// PRINCIPLES
fn draw_third_tab<B>(f: &mut tui::Frame<B>, app: &mut App, area: tui::layout::Rect)
where
    B: Backend,
{
    let principles: HashMap<String, String> = app.principles.clone();
    let len = principles.len();
    let mut text = Vec::<Spans>::with_capacity(len);
    (1..=len).for_each(|id: usize| {
        text.push(Spans::from(format!(
            "{id}. {principle}",
            principle = principles
                .get(&id.to_string())
                .expect("Failed to get principle from principles map. This should never happen.")
                .to_owned(),
        )));
    });
    let block = Block::default()
        .title(Span::styled(
            "Principles",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default());
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

// ----------------------------------------------------------------------------

/* fn draw_footer(string: &str) -> Paragraph {
    let text: Vec<Spans> = vec![Spans::from(string), Spans::from(""), Spans::from("")];
    let block = Block::default()
        .title(Span::styled(
            "",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default());

    Paragraph::new(text).block(block).wrap(Wrap { trim: true })
} */

// ----------------------------------------------------------------------------
