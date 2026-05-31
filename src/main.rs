use std::io::{self, Result};
use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use std::str::FromStr;
use chrono::Utc;

use notify_rust::Notification;
use cron::Schedule;
use cron::ScheduleIterator;
use std::iter::Peekable;

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
use crate::app::Notif;

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

    let app_clone = Arc::clone(&app.notif);
    thread::spawn(move || {
        let expression = "0 0 20 * * * *";
        let schedule = Schedule::from_str(expression).unwrap();
        let mut iterator = schedule.upcoming(Utc).peekable();
        loop {
            thread::sleep(Duration::from_millis(60000));
            check_notification_trigger(app_clone.lock().unwrap().clone(), &mut iterator);
        }
    });

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

pub fn check_notification_trigger(count: Notif, iter: &mut Peekable<ScheduleIterator<'_, Utc>>) {
    let datetime = iter.peek();
    let now = Utc::now();
    match datetime {
        Some(x) => if *x <= now {
            send_notification(count);
            iter.next();
        }
        // The division was invalid
        None    => {},
    }
}

pub fn send_notification(count: Notif) {
    let s = count.get_notification_text();
    if !s.is_empty() {
        let _ = Notification::new().summary("flow_state for today")
            .body(&s[..])
            .icon("firefox")
            .show();
    }
}


