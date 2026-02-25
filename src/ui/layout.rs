use crate::app::{App, CurrentScreen};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{ Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

//use super::heatmap::render_heatmap_page;
use super::stats::render_stats_page;
use super::today::render_today_page;

pub fn render_title(chunk: Rect, frame: &mut Frame) {
    let title_items = vec![
        ListItem::new(
            Line::styled(
                "🌊 Flow State 🌊",
                Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
            )
            .centered(),
        ),
        ListItem::new(Line::from("Minimalist Habit Tracker").centered()),
    ];
    let title = List::new(title_items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    frame.render_widget(title, chunk);
}

pub fn render_tab(chunk: Rect, frame: &mut Frame, app: &App) {
    let tab_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
        .split(chunk);

    let day_name = app.current_day.as_str();

    let outer_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let today_tab = render_tab_item(day_name, matches!(app.current_screen, CurrentScreen::Today));
    let today_inner = outer_block.inner(tab_chunks[0]);
    frame.render_widget(outer_block.clone(), tab_chunks[0]);
    frame.render_widget(today_tab, today_inner);

    let stats_tab = render_tab_item("Stats", matches!(app.current_screen, CurrentScreen::Stats));
    let stats_inner = outer_block.inner(tab_chunks[1]);
    frame.render_widget(outer_block.clone(), tab_chunks[1]);
    frame.render_widget(stats_tab, stats_inner);

    let heatmap_tab = render_tab_item(
        "Heatmap",
        matches!(app.current_screen, CurrentScreen::Heatmap),
    );
    let heatmap_inner = outer_block.inner(tab_chunks[2]);
    frame.render_widget(outer_block.clone(), tab_chunks[2]);
    frame.render_widget(heatmap_tab, heatmap_inner);
}

fn render_tab_item(label: &str, is_active: bool) -> Paragraph<'_> {
    let content = Paragraph::new(Line::from(label).fg(if is_active {
        Color::Black
    } else {
        Color::default()
    }))
    .centered();
    content.block(Block::default().style(Style::new().bg(if is_active {
        Color::Gray
    } else {
        Color::default()
    })))
}

pub fn render_body(chunk: Rect, frame: &mut Frame, app: &App) {
    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(4)])
        .split(chunk);
    match app.current_screen {
        CurrentScreen::Today => render_today_page(body_chunks, frame, app),
        CurrentScreen::Stats => render_stats_page(body_chunks, frame, app),
        CurrentScreen::Heatmap => {}
    }
}
