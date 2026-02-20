
use crate::app::{App, CurrentScreen};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::heatmap::render_heatmap_page;
use super::stats::render_stats_page;
use super::today::render_today_page;

pub fn render_title(chunk: Rect, frame: &mut Frame) {
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

pub fn render_tab(chunk: Rect, frame: &mut Frame, app: &App) {
    let tab_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
        .split(chunk);

    let day_name = app.current_day.as_str();

    let today_tab = render_tab_item(day_name, matches!(app.current_screen, CurrentScreen::Today));
    frame.render_widget(today_tab, tab_chunks[0]);

    let stats_tab = render_tab_item("Stats", matches!(app.current_screen, CurrentScreen::Stats));
    frame.render_widget(stats_tab, tab_chunks[1]);

    let heatmap_tab =
        render_tab_item("Heatmap", matches!(app.current_screen, CurrentScreen::Heatmap));
    frame.render_widget(heatmap_tab, tab_chunks[2]);
}

fn render_tab_item(label: &str, is_active: bool) -> Paragraph<'_> {
    let content = if is_active {
        Paragraph::new(Line::from(label).fg(Color::Black).bg(Color::LightYellow)).centered()
    } else {
        Paragraph::new(Line::from(label)).centered()
    };
    content.block(Block::default().borders(Borders::ALL))
}

pub fn render_body(chunk: Rect, frame: &mut Frame, app: &App) {
    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(4)])
        .split(chunk);
    match app.current_screen {
        CurrentScreen::Today => render_today_page(body_chunks, frame, app),
        CurrentScreen::Stats => render_stats_page(body_chunks, frame, app),
        CurrentScreen::Heatmap => render_heatmap_page(body_chunks, frame, app),
    }
}
