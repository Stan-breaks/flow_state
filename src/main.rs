use std::io;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, ScreenMode},
    ui::ui,
};

fn main() -> color_eyre::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();

    app.load_habits().unwrap();

    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Today => match key.code {
                    KeyCode::Tab => {
                        app.toggle_page();
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.increment_habits_counter();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.decrement_habits_counter();
                    }
                    KeyCode::Char('a') => match app.screen_mode {
                        ScreenMode::Normal => {
                            app.toggle_add_mode();
                        }
                        _ => {}
                    },
                    KeyCode::Char('e') => match app.screen_mode {
                        ScreenMode::Normal => {
                            app.toggle_edit_mode();
                        }
                        _ => {}
                    },
                    KeyCode::Enter => {
                        let build_habit_len = app.build_habits.len();
                        if app.habits_counter <= build_habit_len {
                            app.build_habits[app.habits_counter - 1].toggle_complete();
                        } else {
                            app.avoid_habits[app.habits_counter - build_habit_len - 1]
                                .toggle_complete();
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Stats => match key.code {
                    KeyCode::Tab => {
                        app.toggle_page();
                    }
                    _ => {}
                },
            }
            match key.code {
                KeyCode::Char('q') => match app.screen_mode {
                    ScreenMode::Normal => {
                        app.save_habits().unwrap();
                        break;
                    }
                    _ => {
                        app.toggle_normal_mode();
                    }
                },
                _ => {}
            };
        }
    }
    Ok(())
}
