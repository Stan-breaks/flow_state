use std::rc::Rc;

use crate::app::App;
use crate::habit::{Day, Habit};
use ratatui::widgets::BorderType;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render_today_page(body_chunks: Rc<[Rect]>, frame: &mut Frame, app: &App) {
    let habit_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(body_chunks[0]);

    let build_habit_list = render_habit_list(
        &app.build_habits,
        &app.current_day,
        app.counter.build_counter,
        !app.counter.switch,
        0,
        "🌟 Build These Habits",
        Color::Green,
    );
    let build_len = app.build_habits.len();
    frame.render_widget(build_habit_list, habit_chunks[0]);

    let avoid_habit_list = render_habit_list(
        &app.avoid_habits,
        &app.current_day,
        app.counter.avoid_counter,
        app.counter.switch,
        build_len,
        "🚫 Avoid These Habits",
        Color::Red,
    );
    frame.render_widget(avoid_habit_list, habit_chunks[1]);

    render_footer(body_chunks[1], frame, app);
}

fn render_habit_list<'a>(
    habits: &[Habit],
    current_day: &Day,
    selected_index: usize,
    is_active: bool,
    index_offset: usize,
    title: &'a str,
    color: Color,
) -> List<'a> {
    let items: Vec<ListItem> = habits
        .iter()
        .enumerate()
        .map(|(idx, habit)| {
            let text = format!(
                "{} [{}] {}  •  {}",
                habit.check_status(current_day),
                idx + index_offset + 1,
                habit.name,
                habit.check_pattern()
            );
            if idx == selected_index && is_active {
                ListItem::new(text).bg(color).fg(Color::Black)
            } else {
                ListItem::new(text)
            }
        })
        .collect();

    List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(title)
    )
}

fn render_footer(area: Rect, frame: &mut Frame, app: &App) {
    let footer_block = Block::default().borders(Borders::ALL);
    frame.render_widget(&footer_block, area);

    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Min(1)])
        .split(footer_block.inner(area));

    let day_name = app.current_day.as_str();

    let stat_lines = vec![
        ListItem::new(
            Line::from(format!(
                "{}: {}",
                day_name,
                app.check_todays_progress(&app.current_day)
            ))
            .fg(Color::Green)
            .centered(),
        ),
        ListItem::new(
            Line::from(format!("Week: {}", app.check_weeks_progress()))
                .fg(Color::Green)
                .centered(),
        ),
    ];
    frame.render_widget(List::new(stat_lines), inner_chunks[0]);

    let hint_lines = vec![
        ListItem::new(
            Line::from("[Enter]/[Space] Toggle Habits • [↑↓]/[jk] Navigate • [r] reset habit")
                .fg(Color::Green)
                .centered(),
        ),
        ListItem::new(
            Line::from("[a] Add • [e] Edit • [d] Delete • [y] Switch Day • [TAB] Switch Views ")
                .fg(Color::Green)
                .centered(),
        ),
    ];
    frame.render_widget(List::new(hint_lines), inner_chunks[1]);
}
