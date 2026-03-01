use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph},
    Frame,
};

use crate::{app::App, ui::layout::render_body};

pub fn render_heatmap_page(chunks: Rc<[Rect]>, frame: &mut Frame, app: &App) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(10)])
        .split(chunks[0]);

    render_heatmap_body(body_chunks[0], frame);

    let years_list = render_year_list(&app.years, app.counter.year_counter);
    frame.render_widget(years_list, body_chunks[1]);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);
    let footer = Paragraph::new("hints")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .centered();
    frame.render_widget(footer, footer_chunks[1]);
}

fn render_year_list(years: &[String], selected_index: usize) -> List<'_> {
    let items: Vec<ListItem> = years
        .iter()
        .enumerate()
        .map(|(idx, year)| {
            let text = Line::from(format!("{}", year)).alignment(Alignment::Center);
            if idx == selected_index {
                ListItem::new(text).bg(Color::Gray).fg(Color::Black)
            } else {
                ListItem::new(text)
            }
        })
        .collect();
    List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
}
fn render_month_label(chunk: Rect, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let block_inner = block.inner(chunk);
    let months = vec![
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    let months_len = months.len();
    let constraints = vec![Constraint::Ratio(1, months_len as u32); months_len];
    let month_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(block_inner);
    for (i, &month) in months.iter().enumerate() {
        frame.render_widget(Paragraph::new(month).centered(), month_chunks[i]);
    }
    frame.render_widget(block, chunk);
}
fn render_day_label(chunk: Rect, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::vertical(1));
    let block_inner = block.inner(chunk);
    let days = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let days_len = days.len();
    let constraints = vec![Constraint::Ratio(1, days_len as u32); days_len];
    let day_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(block_inner);

    for (i, &day) in days.iter().enumerate() {
        let area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .split(day_chunks[i]);

        frame.render_widget(Paragraph::new(day).centered(), area[1]);
    }
    frame.render_widget(block, chunk);
}

fn render_heatmap_body(chunk: Rect, frame: &mut Frame) {
    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(chunk);
    let label_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(5), Constraint::Min(1)])
        .split(body_chunks[0]);
    render_month_label(label_chunks[1], frame);

    let day_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(5), Constraint::Min(1)])
        .split(body_chunks[1]);
    render_day_label(day_chunks[0], frame);

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        day_chunks[1],
    );
}
