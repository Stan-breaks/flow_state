use std::io;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod habit;
mod input;
mod storage;
mod ui;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create app and load data
    let mut app = App::new();
    if let Err(e) = app.load_habits() {
        eprintln!("Warning: Failed to load habits: {}", e);
    }

    // Run the app
    let res = input::run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}
