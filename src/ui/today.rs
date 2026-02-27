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
        "🌟 Build These Habits",
        Color::Green,
    );
    frame.render_widget(build_habit_list, habit_chunks[0]);

    let avoid_habit_list = render_habit_list(
        &app.avoid_habits,
        &app.current_day,
        app.counter.avoid_counter,
        app.counter.switch,
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
                idx + 1,
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
            .title(title),
    )
}

fn render_footer(area: Rect, frame: &mut Frame, app: &App) {
    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    let day_name = app.current_day.as_str();

    let stat_lines = vec![
        ListItem::new(
            Line::from(format!(
                "{}: {}",
                day_name,
                app.check_todays_progress(&app.current_day)
            ))
            .centered(),
        ),
        ListItem::new(Line::from(format!("Week: {}", app.check_weeks_progress())).centered()),
    ];
    frame.render_widget(
        List::new(stat_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        inner_chunks[1],
    );
}
