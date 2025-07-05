use std::rc::Rc;

use crate::app::{App, CurrentScreen, Habit, HabitPattern, HabitType, ScreenMode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{block, Block, Borders, Clear, List, ListItem, Paragraph},
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
            add_float_render(frame, area, app);
        }
        ScreenMode::Editing => {
            edit_float_render(frame, area, app);
        }
        ScreenMode::Deleting => {
            delete_float(frame, area, app);
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
            render_today_page(body_chunks, frame, app);
        }
        CurrentScreen::Stats => {
            render_stats_page(body_chunks, frame, app);
        }
    }
}
fn render_stats_page(body_chunks: Rc<[Rect]>, frame: &mut Frame, app: &App) {
    let stat_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(body_chunks[0]);

    let total_len = app.habits.len();
    let mastered_len = app
        .habits
        .iter()
        .filter(|h| h.check_pattern() == HabitPattern::Mastered)
        .collect::<Vec<&Habit>>()
        .len();
    let developing_len = app
        .habits
        .iter()
        .filter(|h| h.check_pattern() == HabitPattern::Developing)
        .collect::<Vec<&Habit>>()
        .len();
    let chaotic_len = app
        .habits
        .iter()
        .filter(|h| h.check_pattern() == HabitPattern::Chaotic)
        .collect::<Vec<&Habit>>()
        .len();

    let pattern_list = List::new([
        ListItem::new(format!(
            "• Mastered: {} habits({}%)",
            mastered_len,
            (mastered_len as f32 / total_len as f32 * 100.00) as u32
        )),
        ListItem::new(format!(
            "• Developing: {} habits({}%)",
            developing_len,
            (developing_len as f32 / total_len as f32 * 100.00) as u32
        )),
        ListItem::new(format!(
            "• Chaotic: {} habits({}%)",
            chaotic_len,
            (chaotic_len as f32 / total_len as f32 * 100.00) as u32
        )),
    ])
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(pattern_list, stat_chunks[0]);

    let hints = Paragraph::new(
        Line::from("[P] Bulk pause • [↑↓]/[jk] Navigate")
            .fg(Color::Green)
            .centered(),
    )
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(hints, body_chunks[1]);
}
fn render_today_page(body_chunks: Rc<[Rect]>, frame: &mut Frame, app: &App) {
    let habit_chucks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(body_chunks[0]);

    let build_habit = List::new(
        app.habits
            .iter()
            .enumerate()
            .filter(|(_, habit)| habit.habit_type == HabitType::Build)
            .enumerate()
            .map(|(display_idx, (_, habit))| {
                if display_idx + 1 == app.counter.build_counter {
                    ListItem::new(format!(
                        "{} [{}] {}  •  {}",
                        habit.check_status().emoji(),
                        display_idx + 1,
                        habit.name,
                        habit.check_pattern().string()
                    ))
                    .bg(Color::Green)
                } else {
                    ListItem::new(format!(
                        "{} [{}] {}  •  {}",
                        habit.check_status().emoji(),
                        display_idx + 1,
                        habit.name,
                        habit.check_pattern().string()
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
    let avoid_habit = List::new(
        app.habits
            .iter()
            .enumerate()
            .filter(|(_, habit)| habit.habit_type == HabitType::Avoid)
            .enumerate()
            .map(|(display_idx, (_, habit))| {
                if display_idx + 1 == app.counter.avoid_counter {
                    ListItem::new(format!(
                        "{} [{}] {}  •  {}",
                        habit.check_status().emoji(),
                        display_idx + 1,
                        habit.name,
                        habit.check_pattern().string()
                    ))
                    .bg(Color::Red)
                } else {
                    ListItem::new(format!(
                        "{} [{}] {}  •  {}",
                        habit.check_status().emoji(),
                        display_idx + 1,
                        habit.name,
                        habit.check_pattern().string()
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
            Line::from("[Enter] Toggle Habits • [↑↓]/[jk] Navigate ")
                .fg(Color::Green)
                .centered(),
        ),
        ListItem::new(
            Line::from("[a] Add • [e] Edit • [d] Delete • [TAB] Switch Views ")
                .fg(Color::Green)
                .centered(),
        ),
    ];
    let hint_list = List::new(hint_lines);
    frame.render_widget(hint_list, inner_chunks[1]);
}

fn add_float_render(frame: &mut Frame, area: Rect, app: &App) {
    let popup_area = centered_rect(area);
    let popup_block = Block::default().borders(Borders::ALL).title("Add habit");
    let inner_area = popup_block.inner(popup_area);
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
        ])
        .split(inner_area);
    let input_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_chunks[0]);
    let button_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[2]);
    let name_label = Paragraph::new("Name:");
    let name_input = Paragraph::new(format!("{}", app.current_habit.name))
        .block(Block::default().borders(Borders::ALL));

    let build_tab = match app.current_habit.habit_type {
        HabitType::Build => Paragraph::new(
            Line::from("Build Habit")
                .bg(Color::LightYellow)
                .fg(Color::Black),
        )
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center),
        HabitType::Avoid => Paragraph::new("Build Habit")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
    };

    let avoid_tab = match app.current_habit.habit_type {
        HabitType::Build => Paragraph::new("Avoid Habit")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
        HabitType::Avoid => Paragraph::new(
            Line::from("Avoid Habit")
                .bg(Color::LightYellow)
                .fg(Color::Black),
        )
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center),
    };

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);
    frame.render_widget(name_label, input_chunks[0]);
    frame.render_widget(name_input, input_chunks[1]);
    frame.render_widget(build_tab, button_chunks[0]);
    frame.render_widget(avoid_tab, button_chunks[1]);

    let x_offset = app.current_habit.name.len() as u16;
    let input_area = input_chunks[1];
    let cursor_x = input_area.x + x_offset + 1;
    let cursor_y = input_area.y + 1;
    let postion = Position::new(cursor_x, cursor_y);
    frame.set_cursor_position(postion);
}

fn edit_float_render(frame: &mut Frame, area: Rect, app: &App) {
    let popup_area = centered_rect(area);
    let popup_block = Block::default().borders(Borders::ALL).title("Edit habit");
    let inner_area = popup_block.inner(popup_area);
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
        ])
        .split(inner_area);
    let input_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_chunks[0]);
    let button_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[2]);
    let name_label = Paragraph::new("Name:");
    let name_input = Paragraph::new(format!("{}", app.current_habit.name))
        .block(Block::default().borders(Borders::ALL));

    let build_tab = match app.current_habit.habit_type {
        HabitType::Build => Paragraph::new(
            Line::from("Build Habit")
                .bg(Color::LightYellow)
                .fg(Color::Black),
        )
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center),
        HabitType::Avoid => Paragraph::new("Build Habit")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
    };

    let avoid_tab = match app.current_habit.habit_type {
        HabitType::Build => Paragraph::new("Avoid Habit")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
        HabitType::Avoid => Paragraph::new(
            Line::from("Avoid Habit")
                .bg(Color::LightYellow)
                .fg(Color::Black),
        )
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center),
    };

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);
    frame.render_widget(name_label, input_chunks[0]);
    frame.render_widget(name_input, input_chunks[1]);
    frame.render_widget(build_tab, button_chunks[0]);
    frame.render_widget(avoid_tab, button_chunks[1]);

    let x_offset = app.current_habit.name.len() as u16;
    let input_area = input_chunks[1];
    let cursor_x = input_area.x + x_offset + 1;
    let cursor_y = input_area.y + 1;
    let postion = Position::new(cursor_x, cursor_y);
    frame.set_cursor_position(postion);
}

fn delete_float(frame: &mut Frame, area: Rect, app: &App) {
    let popup_block = Block::default().borders(Borders::ALL);
    let popup_area = smaller_centered_rect(area);
    let inner_area = popup_block.inner(popup_area);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(35),
            Constraint::Percentage(20),
        ])
        .split(inner_area);

    let msg = Paragraph::new("Confirm delete").fg(Color::Red).centered();

    let habit_title = Paragraph::new(format!("{}", app.current_habit.name))
        .fg(Color::White)
        .centered();

    let choices = Paragraph::new("y/n").fg(Color::White).centered();

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);
    frame.render_widget(msg, main_chunks[0]);
    frame.render_widget(habit_title, main_chunks[1]);
    frame.render_widget(choices, main_chunks[2]);
}

fn smaller_centered_rect(area: Rect) -> Rect {
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
            Constraint::Percentage((100 - 35) / 2),
            Constraint::Percentage(35),
            Constraint::Percentage((100 - 35) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn centered_rect(area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - 40) / 2),
            Constraint::Percentage(40),
            Constraint::Percentage((100 - 40) / 2),
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
