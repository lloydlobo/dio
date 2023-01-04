//! [`ui`] implements TUI.

use crate::app::App;

use tui::{
    self,
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
};

/* let chunks = Layout::default()
.direction(Direction::Vertical)
.margin(1u16)
.constraints([ Constraint::Percentage(20u16), Constraint::Percentage(60u16), Constraint::Percentage(20u16)] .as_ref(),)
.split(f.size()); */
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
fn draw_first_tab<B>(f: &mut tui::Frame<B>, app: &mut App, area: tui::layout::Rect)
where
    B: Backend,
{
    f.render_widget(draw_footer(gen_str(1usize).as_str()), area);
}

/// FACTS
fn draw_second_tab<B>(f: &mut tui::Frame<B>, app: &mut App, area: tui::layout::Rect)
where
    B: Backend,
{
    f.render_widget(draw_footer(gen_str(2usize).as_str()), area);
}

/// PRINCIPLES
fn draw_third_tab<B>(f: &mut tui::Frame<B>, app: &mut App, area: tui::layout::Rect)
where
    B: Backend,
{
    f.render_widget(draw_footer(gen_str(3usize).as_str()), area);
}

// ----------------------------------------------------------------------------

fn draw_footer(string: &str) -> Paragraph {
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
}

// ----------------------------------------------------------------------------

fn gen_str(idx: usize) -> String {
    let result = vec![
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit,",
        "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
        "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
        "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur",
    ];

    result[idx].to_owned()
}
