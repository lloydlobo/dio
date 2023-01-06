//! [`backend`] is `crossterm` backend.

use crate::{
    app::{self, App, InputMode},
    db::DB,
    server, ui,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    string,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

/// .
///
/// # Errors
///
/// This function will return an error if .
pub async fn run(tick_rate: Duration, enhanced_graphics: bool) -> anyhow::Result<()> {
    // Setup terminal.
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (facts, principles) = server::get_server_data().await?;
    let db = DB { facts, principles };

    let app = App::new("Dio App", db, enhanced_graphics);

    // Run the app.
    run_app(&mut terminal, app, tick_rate).await?;

    // Restore terminal.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

// ----------------------------------------------------------------------------

async fn run_app<B>(
    terminal: &mut Terminal<B>,
    mut app: App<'_>,
    tick_rate: Duration,
) -> anyhow::Result<(), anyhow::Error>
where
    B: Backend,
{
    // let daily_message: String = db::get_todays_fact_or_principle(db)?;
    let mut last_tick = Instant::now();
    loop {
        // `draw` - Synchronizes terminal size, calls the rendering closure, flushes the current
        // internal state and prepares for the next draw call.
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0u64));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char(c) => {
                            app.on_key(c);
                        }
                        KeyCode::Left => {
                            app.on_left();
                        }
                        KeyCode::Right => {
                            app.on_right();
                        }
                        KeyCode::Up => {
                            app.on_up();
                        }
                        KeyCode::Down => app.on_down(),
                        KeyCode::Esc => {
                            app.shortcuts_help.unselect();
                        }
                        KeyCode::Enter => {
                            app.jump_to_tab();
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            let new: string::Drain = app.input.drain(..); // Removes the specified range from the string in bulk,
                                                                          // returning all removed characters as an iterator.
                            app.messages.push(new.collect());
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    } // end of loop.
}

// ----------------------------------------------------------------------------
