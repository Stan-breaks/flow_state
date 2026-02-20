mod helpers;
mod heatmap;
mod layout;
mod popups;
mod stats;
mod today;

use crate::app::App;
use layout::{render_body, render_tab, render_title};
use popups::{confirm_float, habit_form_float};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::ScreenMode;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(frame.area());

    render_title(chunks[0], frame);
    render_tab(chunks[1], frame, app);
    render_body(chunks[2], frame, app);

    let area = frame.area();
    match app.screen_mode {
        ScreenMode::Adding => habit_form_float(frame, area, app, "Add habit"),
        ScreenMode::Editing => habit_form_float(frame, area, app, "Edit habit"),
        ScreenMode::Deleting => confirm_float(frame, area, app, "Confirm delete"),
        ScreenMode::Reset => confirm_float(frame, area, app, "Confirm reset"),
        _ => {}
    }
}
