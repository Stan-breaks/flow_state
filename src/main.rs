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
    app::{App, CurrentScreen, Habit, HabitType, ScreenMode},
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
                CurrentScreen::Today => match app.screen_mode {
                    ScreenMode::Normal => match key.code {
                        KeyCode::Tab => {
                            app.toggle_page();
                        }
                        KeyCode::Char('j') | KeyCode::Down => match app.screen_mode {
                            ScreenMode::Normal => {
                                app.increment_habits_counter();
                            }
                            _ => {}
                        },
                        KeyCode::Char('k') | KeyCode::Up => match app.screen_mode {
                            ScreenMode::Normal => {
                                app.decrement_habits_counter();
                            }
                            _ => {}
                        },
                        KeyCode::Char('a') => match app.screen_mode {
                            ScreenMode::Normal => {
                                app.toggle_add_mode();
                            }
                            _ => {}
                        },
                        KeyCode::Char('e') => match app.screen_mode {
                            ScreenMode::Normal => {
                                if !app.counter.switch && app.counter.build_counter > 0 {
                                    let build_habit = app
                                        .habits
                                        .iter()
                                        .filter(|habit| habit.habit_type == HabitType::Build)
                                        .collect::<Vec<&Habit>>()[app.counter.build_counter - 1];
                                    let mut index = 0;
                                    for i in 0..app.habits.len() {
                                        if &app.habits[i] == build_habit {
                                            index = i;
                                            break;
                                        }
                                    }
                                    app.toggle_edit_mode(app.habits[index].clone());
                                }
                                if app.counter.switch && app.counter.avoid_counter > 0 {
                                    let avoid_habit = app
                                        .habits
                                        .iter()
                                        .filter(|habit| habit.habit_type == HabitType::Avoid)
                                        .collect::<Vec<&Habit>>()[app.counter.avoid_counter - 1];
                                    let mut index = 0;
                                    for i in 0..app.habits.len() {
                                        if &app.habits[i] == avoid_habit {
                                            index = i;
                                            break;
                                        }
                                    }
                                    app.toggle_edit_mode(app.habits[index].clone());
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Enter => {
                            if !app.counter.switch && app.counter.build_counter > 0 {
                                let build_habit = app
                                    .habits
                                    .iter()
                                    .filter(|habit| habit.habit_type == HabitType::Build)
                                    .collect::<Vec<&Habit>>()[app.counter.build_counter - 1];
                                let mut index = 0;
                                for i in 0..app.habits.len() {
                                    if &app.habits[i] == build_habit {
                                        index = i;
                                        break;
                                    }
                                }
                                app.habits[index].toggle_complete();
                            }
                            if app.counter.switch && app.counter.avoid_counter > 0 {
                                let avoid_habit = app
                                    .habits
                                    .iter()
                                    .filter(|habit| habit.habit_type == HabitType::Avoid)
                                    .collect::<Vec<&Habit>>()[app.counter.avoid_counter - 1];
                                let mut index = 0;
                                for i in 0..app.habits.len() {
                                    if &app.habits[i] == avoid_habit {
                                        index = i;
                                        break;
                                    }
                                }
                                app.habits[index].toggle_complete();
                            }
                        }
                        KeyCode::Char('d') => {
                            if !app.counter.switch && app.counter.build_counter > 0 {
                                app.toggle_delete_mode();
                            }
                            if app.counter.switch && app.counter.avoid_counter > 0 {
                                app.toggle_delete_mode();
                            }
                        }
                        _ => {}
                    },
                    ScreenMode::Adding => match key.code {
                        KeyCode::Tab => {
                            app.toggle_habit_type();
                        }
                        KeyCode::Backspace => {
                            app.current_habit.name.pop();
                        }
                        KeyCode::Enter => {
                            app.add_habit();
                        }
                        KeyCode::Char(value) => {
                            app.current_habit.name.push(value);
                        }
                        _ => {}
                    },
                    ScreenMode::Editing => match key.code {
                        KeyCode::Tab => {
                            app.toggle_habit_type();
                        }
                        KeyCode::Backspace => {
                            app.current_habit.name.pop();
                        }
                        KeyCode::Enter => {
                            if !app.counter.switch && app.counter.build_counter > 0 {
                                let build_habit = app
                                    .habits
                                    .iter()
                                    .filter(|habit| habit.habit_type == HabitType::Build)
                                    .collect::<Vec<&Habit>>()[app.counter.build_counter - 1];
                                let mut index = 0;
                                for i in 0..app.habits.len() {
                                    if &app.habits[i] == build_habit {
                                        index = i;
                                        break;
                                    }
                                }
                                app.edit_habit(index);
                            }
                            if app.counter.switch && app.counter.avoid_counter > 0 {
                                let avoid_habit = app
                                    .habits
                                    .iter()
                                    .filter(|habit| habit.habit_type == HabitType::Avoid)
                                    .collect::<Vec<&Habit>>()[app.counter.avoid_counter - 1];
                                let mut index = 0;
                                for i in 0..app.habits.len() {
                                    if &app.habits[i] == avoid_habit {
                                        index = i;
                                        break;
                                    }
                                }
                                app.edit_habit(index);
                            }
                        }
                        KeyCode::Char(value) => {
                            app.current_habit.name.push(value);
                        }
                        _ => {}
                    },
                    ScreenMode::Deleting => {}
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
                    _ => {}
                },
                KeyCode::Esc => match app.screen_mode {
                    ScreenMode::Normal => {}
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
