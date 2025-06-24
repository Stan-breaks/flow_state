use std::rc::Rc;

use crate::app::{App, CurrentScreen, ScreenMode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = create_main_layout(frame);
    render_main_ui(&chunks, frame, app);
}

fn create_main_layout(frame: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(frame.area())
}
fn render_main_ui(chunks: &Rc<[Rect]>, frame: &mut Frame, app: &App) {
    render_title(chunks[0], frame);
    render_tab(chunks[1], frame, app);
    render_body(chunks[2], frame, app);
    let area = frame.area();
    match app.screen_mode {
        ScreenMode::Adding => {
            add_float_render(frame, area);
        }
        ScreenMode::Editing => {
            edit_float_render(frame, area);
        }
        _ => {}
    };
}

fn render_title(chunk: Rect, frame: &mut Frame) {
    let title_items = vec![
        ListItem::new(
            Line::from("🌊 Flow State 🌊")
                .style(Style::default().bold())
                .centered(),
        ),
        ListItem::new(Line::from("Minimalist Habit Tracker").centered()),
    ];
    let title = List::new(title_items).block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunk);
}

fn render_tab(chunk: Rect, frame: &mut Frame, app: &App) {
    let tab_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Min(1)])
        .split(chunk);

    let today_tab = match app.current_screen {
        CurrentScreen::Today => {
            Paragraph::new(Line::from("Today").fg(Color::Black).bg(Color::LightYellow)).centered()
        }
        _ => Paragraph::new(Line::from("Today")).centered(),
    }
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(today_tab, tab_chunks[0]);

    let stats_tab = match app.current_screen {
        CurrentScreen::Stats => {
            Paragraph::new(Line::from("Stats").fg(Color::Black).bg(Color::LightYellow)).centered()
        }
        _ => Paragraph::new(Line::from("Stats")).centered(),
    }
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(stats_tab, tab_chunks[1]);
}

fn render_body(chunk: Rect, frame: &mut Frame, app: &App) {
    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(4)])
        .split(chunk);
    match app.current_screen {
        CurrentScreen::Today => {
            let habit_chucks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(body_chunks[0]);

            let build_habit = List::new(
                app.build_habits
                    .iter()
                    .enumerate()
                    .map(|(index, habit)| {
                        if index + 1 == app.habits_counter {
                            ListItem::new(format!(
                                "{} [{}]. {}, {}  {}",
                                habit.check_status().emoji(),
                                index + 1,
                                habit.name,
                                habit.created,
                                habit.days_completed.len()
                            ))
                            .bg(Color::Green)
                        } else {
                            ListItem::new(format!(
                                "{} [{}]. {}, {}  {}",
                                habit.check_status().emoji(),
                                index + 1,
                                habit.name,
                                habit.created,
                                habit.days_completed.len()
                            ))
                        }
                    })
                    .collect::<Vec<ListItem>>(),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🌟 Build These Habits")
                    .border_style(Style::default().fg(Color::Green)),
            );
            frame.render_widget(build_habit, habit_chucks[0]);
            let build_habits_len = app.build_habits.len() + 1;
            let avoid_habit = List::new(
                app.avoid_habits
                    .iter()
                    .enumerate()
                    .map(|(index, habit)| {
                        if index + build_habits_len == app.habits_counter {
                            ListItem::new(format!(
                                "{} [{}]. {}, {}  {}",
                                habit.check_status().emoji(),
                                index + build_habits_len,
                                habit.name,
                                habit.created,
                                habit.days_completed.len()
                            ))
                            .bg(Color::Red)
                        } else {
                            ListItem::new(format!(
                                "{} [{}]. {}, {}  {}",
                                habit.check_status().emoji(),
                                index + build_habits_len,
                                habit.name,
                                habit.created,
                                habit.days_completed.len()
                            ))
                        }
                    })
                    .collect::<Vec<ListItem>>(),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🚫 Avoid These Habits")
                    .border_style(Style::default().fg(Color::Red)),
            );
            frame.render_widget(avoid_habit, habit_chucks[1]);
            let footer_area = body_chunks[1];
            let footer_block = Block::default().borders(Borders::ALL);
            frame.render_widget(&footer_block, footer_area);

            let inner_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(1), Constraint::Min(1)])
                .split(footer_block.inner(footer_area));

            let stat_lines = vec![
                ListItem::new(
                    Line::from(format!("Today: {}", app.check_todays_progress()))
                        .fg(Color::Green)
                        .centered(),
                ),
                ListItem::new(
                    Line::from(format!("Week: {}", app.check_weeks_progress()))
                        .fg(Color::Green)
                        .centered(),
                ),
            ];
            let stat_list = List::new(stat_lines);
            frame.render_widget(stat_list, inner_chunks[0]);

            let hint_lines = vec![
                ListItem::new(
                    Line::from("[Enter] Toggle Habits • [↑↓]/[hjkl] Navigate • [TAB] Switch Views")
                        .fg(Color::Green)
                        .centered(),
                ),
                ListItem::new(
                    Line::from("[a] Add • [e] Edit • [d] Delete • [p] Pause/Resume ")
                        .fg(Color::Green)
                        .centered(),
                ),
            ];
            let hint_list = List::new(hint_lines);
            frame.render_widget(hint_list, inner_chunks[1]);
        }
        CurrentScreen::Stats => {
            let hints = Paragraph::new(
                Line::from("[P] Bulk pause • [↑↓]/[hjkl] Navigate")
                    .fg(Color::Green)
                    .centered(),
            )
            .block(Block::default().borders(Borders::ALL));
            frame.render_widget(hints, body_chunks[1]);
        }
    }
}

fn add_float_render(frame: &mut Frame, area: Rect) {
    let popup_area = centered_rect(area);
    let popup_block = Block::default().borders(Borders::ALL).title("Add habit");
    let inner_area = popup_block.inner(popup_area);
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // Input section
            Constraint::Percentage(30), // Type section
            Constraint::Percentage(30), // Button section
        ])
        .split(inner_area);
    let input_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Label
            Constraint::Percentage(70), // Input field
        ])
        .split(main_chunks[0]);
    let button_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Cancel button
            Constraint::Percentage(50), // Submit button
        ])
        .split(main_chunks[2]);
    let name_label = Paragraph::new("Name:");
    let name_input = Paragraph::new("") // Your input text here
        .block(Block::default().borders(Borders::ALL));

    let cancel_button = Paragraph::new("Cancel")
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    let submit_button = Paragraph::new("Submit")
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    // 8. Render everything
    frame.render_widget(Clear, popup_area); // Clear background
    frame.render_widget(popup_block, popup_area); // Main bordered container
    frame.render_widget(name_label, input_chunks[0]); // Label
    frame.render_widget(name_input, input_chunks[1]); // Input
    frame.render_widget(cancel_button, button_chunks[0]); // Cancel
    frame.render_widget(submit_button, button_chunks[1]); // Submit
}

fn edit_float_render(frame: &mut Frame, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title("Edit habit");
    let paragraph = Paragraph::new("").block(block);
    let popup_area = centered_rect(area);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(paragraph, popup_area);
}

fn centered_rect(area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - 35) / 2),
            Constraint::Percentage(35),
            Constraint::Percentage((100 - 35) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - 60) / 2),
            Constraint::Percentage(60),
            Constraint::Percentage((100 - 60) / 2),
        ])
        .split(popup_layout[1])[1]
}
