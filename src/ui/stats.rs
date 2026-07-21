use std::rc::Rc;

use crate::app::App;
use crate::habit::{find_best_habit, find_worst_habit, Habit, HabitPattern};
use ratatui::style::Stylize;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
    Frame,
};

pub fn render_stats_page(body_chunks: Rc<[Rect]>, frame: &mut Frame, app: &App) {
    let stat_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(body_chunks[0]);

    let all_habits: Vec<Habit> = app
        .build_habits
        .iter()
        .chain(app.avoid_habits.iter())
        .cloned()
        .collect();

    render_pattern_health(stat_chunks[0], frame, &all_habits);
    render_habit_spotlight(stat_chunks[1], frame, &all_habits);
    render_encouragement(body_chunks[1], frame, &all_habits);
}

const TIERS: [(HabitPattern, &str, Color); 5] = [
    (HabitPattern::Chaotic, "Chaotic", Color::Red),
    (HabitPattern::Struggling, "Struggling", Color::Yellow),
    (HabitPattern::Developing, "Developing", Color::Cyan),
    (HabitPattern::Established, "Established", Color::LightGreen),
    (HabitPattern::Mastered, "Mastered", Color::Green),
];

fn render_pattern_health(area: Rect, frame: &mut Frame, habits: &[Habit]) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("🌊 Pattern Health")
        .fg(Color::LightYellow);

    if habits.is_empty() {
        let empty = Paragraph::new("Add a habit to start seeing your patterns")
            .centered()
            .fg(Color::LightYellow)
            .block(block);
        frame.render_widget(empty, area);
        return;
    }

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(TIERS.map(|_| Constraint::Ratio(1, TIERS.len() as u32)))
        .split(inner);

    let total = habits.len() as f32;
    for (i, (pattern, label, color)) in TIERS.iter().enumerate() {
        let count = habits.iter().filter(|h| h.check_pattern() == *pattern).count();
        let pct = (count as f32 / total * 100.0).round() as u16;
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(*color))
            .percent(pct)
            .label(format!("{label} · {count} ({pct}%)"));
        frame.render_widget(gauge, rows[i]);
    }
}

fn render_habit_spotlight(area: Rect, frame: &mut Frame, habits: &[Habit]) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("✨ Habit Spotlight")
        .fg(Color::LightYellow);

    if habits.is_empty() {
        let empty = Paragraph::new("No habits tracked yet")
            .centered()
            .fg(Color::LightYellow)
            .block(block);
        frame.render_widget(empty, area);
        return;
    }

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let newest = habits.iter().max_by_key(|h| h.created).unwrap();
    let thriving = find_best_habit(habits).unwrap();
    let growing_edge = find_worst_habit(habits).unwrap();

    let lines = [
        Line::from(format!("🌊  New: {}", newest.name)).fg(Color::White),
        Line::from(format!("🌟  Thriving: {}", thriving.name)).fg(Color::LightGreen),
        Line::from(format!("🌱  Growing edge: {}", growing_edge.name)).fg(Color::Cyan),
    ];

    frame.render_widget(Paragraph::new(lines.to_vec()), inner);
}

fn render_encouragement(area: Rect, frame: &mut Frame, habits: &[Habit]) {
    let (message, color) = if habits.is_empty() {
        ("Add a habit whenever you're ready — no rush", Color::LightYellow)
    } else {
        let strong = habits
            .iter()
            .filter(|h| matches!(h.check_pattern(), HabitPattern::Established | HabitPattern::Mastered))
            .count();
        let ratio = strong as f32 / habits.len() as f32;
        if ratio >= 0.6 {
            ("Strong patterns forming — keep flowing 🌊", Color::Green)
        } else if ratio >= 0.3 {
            ("Progress isn't a straight line — you're doing fine", Color::Cyan)
        } else {
            ("Patterns take time to form — show up when you can, no pressure", Color::Yellow)
        }
    };

    let hint = Paragraph::new(Line::from(message).centered())
        .fg(color)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    frame.render_widget(hint, area);
}
