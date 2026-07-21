use crate::app::App;
use crate::habit::HabitType;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
    Frame,
};

use super::helpers::centered_rect;

pub fn habit_form_float(frame: &mut Frame, area: Rect, app: &App, title: &str) {
    let popup_area = centered_rect(area, 50, 40);
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_type(BorderType::Rounded)
        .padding(Padding::proportional(1));
    let inner_area = popup_block.inner(popup_area);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(40),
        ])
        .split(inner_area);

    let button_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[2]);

    let name_input = Paragraph::new(app.current_habit.name.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Name:")
            .border_type(BorderType::Rounded),
    );

    let button_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let (build_tab, avoid_tab) = habit_type_tabs(&app.current_habit.habit_type);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);
    frame.render_widget(name_input, main_chunks[0]);

    let inner_build_button = button_block.inner(button_chunks[0]);
    frame.render_widget(button_block.clone(), button_chunks[0]);
    frame.render_widget(build_tab, inner_build_button);

    let inner_avoid_button = button_block.inner(button_chunks[1]);
    frame.render_widget(button_block.clone(), button_chunks[1]);
    frame.render_widget(avoid_tab, inner_avoid_button);

    let x_offset = app.current_habit.name.len() as u16;
    let input_area = main_chunks[0];
    let position = Position::new(input_area.x + x_offset + 1, input_area.y + 1);
    frame.set_cursor_position(position);
}

pub fn holiday_form_float(frame: &mut Frame, area: Rect, app: &App) {
    let popup_area = centered_rect(area, 50, 45);
    let title = format!("🌴 Holiday — {}", app.current_habit.name);
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_type(BorderType::Rounded)
        .padding(Padding::proportional(1));
    let inner_area = popup_block.inner(popup_area);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner_area);

    let field_block = |title: &'static str, focused: bool| {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(title)
            .fg(if focused { Color::Yellow } else { Color::default() })
    };

    let start_input = Paragraph::new(app.holiday_input.start.as_str())
        .block(field_block("Start (YYYY-MM-DD):", !app.holiday_input.focus_end));
    let end_input = Paragraph::new(app.holiday_input.end.as_str())
        .block(field_block("End (YYYY-MM-DD):", app.holiday_input.focus_end));

    let hint = Paragraph::new("Missed days in this range won't count against your pattern")
        .fg(Color::LightYellow)
        .centered();

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);
    frame.render_widget(start_input, main_chunks[0]);
    frame.render_widget(end_input, main_chunks[1]);
    frame.render_widget(hint, main_chunks[2]);

    if let Some(error) = &app.holiday_input.error {
        let error_msg = Paragraph::new(error.as_str()).fg(Color::Red).centered();
        frame.render_widget(error_msg, main_chunks[3]);
    } else {
        let footer_hint = Paragraph::new("Tab switch field · Enter save").centered();
        frame.render_widget(footer_hint, main_chunks[3]);
    }

    let (field_area, text_len) = if app.holiday_input.focus_end {
        (main_chunks[1], app.holiday_input.end.len())
    } else {
        (main_chunks[0], app.holiday_input.start.len())
    };
    let position = Position::new(field_area.x + text_len as u16 + 1, field_area.y + 1);
    frame.set_cursor_position(position);
}

pub fn confirm_float(frame: &mut Frame, area: Rect, app: &App, message: &str) {
    let popup_area = centered_rect(area, 35, 35);
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded).padding(Padding::proportional(1));
    let inner_area = popup_block.inner(popup_area);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(inner_area);

    let msg = Paragraph::new(message)
        .fg(Color::Red)
        .centered()
        .style(Style::default().bold());
    let habit_title = Paragraph::new(if app.counter.switch {
        app.avoid_habits[app.counter.avoid_counter].name.as_str()
    } else {
        app.build_habits[app.counter.build_counter].name.as_str()
    })
    .fg(Color::White)
    .centered();
    let choices = Paragraph::new("y/n")
        .style(Style::default().bold())
        .fg(Color::White)
        .centered();

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);
    frame.render_widget(msg, main_chunks[0]);
    frame.render_widget(habit_title, main_chunks[1]);
    frame.render_widget(choices, main_chunks[2]);
}

fn habit_type_tabs(habit_type: &HabitType) -> (Paragraph<'static>, Paragraph<'static>) {
    let (build_tab, avoid_tab) = match habit_type {
        HabitType::Build => (
            Paragraph::new(Line::from("Build Habit").fg(Color::Black)),
            Paragraph::new("Avoid Habit"),
        ),
        HabitType::Avoid => (
            Paragraph::new("Build Habit"),
            Paragraph::new(Line::from("Avoid Habit").fg(Color::Black)),
        ),
    };

    (
        build_tab
            .block(
                Block::default().bg(if matches!(habit_type, HabitType::Build) {
                    Color::Gray
                } else {
                    Color::default()
                }),
            )
            .alignment(Alignment::Center),
        avoid_tab
            .block(
                Block::default().bg(if matches!(habit_type, HabitType::Avoid) {
                    Color::Gray
                } else {
                    Color::default()
                }),
            )
            .alignment(Alignment::Center),
    )
}
