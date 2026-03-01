use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_heatmap_page(body_chunks: Rc<[Rect]>, frame: &mut Frame, app: &App) {
    let heatmap_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(10)])
        .split(body_chunks[0]);

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        heatmap_chunks[0],
    );

    let years_list = render_year_list(&app.years, app.counter.year_counter);
    frame.render_widget(years_list, heatmap_chunks[1]);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(body_chunks[1]);
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
            let text = format!("{}", year);
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
