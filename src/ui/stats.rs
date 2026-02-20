use std::rc::Rc;

use crate::app::App;
use crate::habit::{find_best_habit, find_worst_habit, HabitPattern};
use ratatui::style::Stylize;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_stats_page(body_chunks: Rc<[Rect]>, frame: &mut Frame, app: &App) {
    let stat_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(body_chunks[0]);

    render_pattern_health(stat_chunks[0], frame, app);
    render_habit_maturity(stat_chunks[1], frame, app);

    let hints = Paragraph::new(Line::from("").centered())
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(hints, body_chunks[1]);
}

fn render_pattern_health(area: Rect, frame: &mut Frame, app: &App) {
    let all_habits: Vec<_> = app
        .build_habits
        .iter()
        .chain(app.avoid_habits.iter())
        .collect();

    let total_len = all_habits.len();
    if total_len == 0 {
        let empty = Paragraph::new("No habits tracked yet")
            .centered()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Pattern Health Check")
            ).fg(Color::LightYellow);
        frame.render_widget(empty, area);
        return;
    }

    let mastered_len = all_habits
        .iter()
        .filter(|h| h.check_pattern() == HabitPattern::Mastered)
        .count();
    let developing_len = all_habits
        .iter()
        .filter(|h| h.check_pattern() == HabitPattern::Developing)
        .count();
    let chaotic_len = all_habits
        .iter()
        .filter(|h| h.check_pattern() == HabitPattern::Chaotic)
        .count();

    let pct = |count: usize| (count as f32 / total_len as f32 * 100.0) as u32;

    let pattern_list = List::new([
        ListItem::new(Line::from(vec![
            Span::styled("• Mastered: ", Style::default().fg(Color::Green)),
            Span::styled(
                format!("{} habits({}%)", mastered_len, pct(mastered_len)),
                Style::default().fg(Color::White),
            ),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("• Developing: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("{} habits({}%)", developing_len, pct(developing_len)),
                Style::default().fg(Color::White),
            ),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("• Chaotic: ", Style::default().fg(Color::Red)),
            Span::styled(
                format!("{} habits({}%)", chaotic_len, pct(chaotic_len)),
                Style::default().fg(Color::White),
            ),
        ])),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Pattern Health Check")
            .fg(Color::LightYellow),
    );

    frame.render_widget(pattern_list, area);
}

fn render_habit_maturity(area: Rect, frame: &mut Frame, app: &App) {
    let all_habits: Vec<_> = app
        .build_habits
        .iter()
        .chain(app.avoid_habits.iter())
        .collect();

    if all_habits.is_empty() {
        let empty = Paragraph::new("No habits tracked yet")
            .centered()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Habit maturity")
                    .fg(Color::LightYellow),
            );
        frame.render_widget(empty, area);
        return;
    }

    let newest = all_habits.iter().max_by_key(|h| h.created).unwrap();
    let oldest = all_habits.iter().min_by_key(|h| h.created).unwrap();

    let binding = all_habits.iter().map(|h| (*h).clone()).collect::<Vec<_>>();
    let best = find_best_habit(&binding);
    let worst = find_worst_habit(&binding);

    let best_name = best.map(|h| h.name.as_str()).unwrap_or("N/A");
    let worst_name = worst.map(|h| h.name.as_str()).unwrap_or("N/A");

    let maturity_list = List::new([
        ListItem::new(Line::from(vec![
            Span::styled("• Newest Habit: ", Style::default().fg(Color::LightYellow)),
            Span::styled(newest.name.clone(), Style::default().fg(Color::White)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("• Oldest Habit: ", Style::default().fg(Color::LightYellow)),
            Span::styled(oldest.name.clone(), Style::default().fg(Color::White)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("• Best Habit: ", Style::default().fg(Color::Green)),
            Span::styled(best_name.to_string(), Style::default().fg(Color::White)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("• Worst Habit: ", Style::default().fg(Color::Red)),
            Span::styled(worst_name.to_string(), Style::default().fg(Color::White)),
        ])),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Habit maturity")
            .fg(Color::LightYellow),
    );

    frame.render_widget(maturity_list, area);
}
