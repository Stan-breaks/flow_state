use std::io::{self, Result};
use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use std::str::FromStr;
use chrono::Local;

use cron::Schedule;

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
mod notifications;

use crate::app::App;

fn main() -> Result<()> {

    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    if let Err(e) = app.load_habits() {
        eprintln!("Warning: Failed to load habits: {}", e);
    }

    let notification_settings = storage::load_notification_settings();

    match notification_settings {
        Err(ref e) => eprintln!("Warning: Failed to load notification settings: {}", e),
        Ok(_) => {},
    }

    let notifications = notification_settings.unwrap();
    app.set_notifications(notifications.clone());

    if notifications.enable {
        let app_clone = Arc::clone(&app.notif);

        thread::spawn(move || {
            let expression = format!("0 {} {} * * * *", notifications.minute, notifications.hour);
            let schedule = Schedule::from_str(&expression).unwrap();
            let mut iterator = schedule.upcoming(Local).peekable();
            loop {
                thread::sleep(Duration::from_secs(30));
                notifications::check_notification_trigger(app_clone.lock().unwrap().clone(), &mut iterator);
            }
        });
    }

    input::run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

