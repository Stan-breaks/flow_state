use std::rc::Rc;

use chrono::{Datelike, NaiveDate};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph},
    Frame,
};

use crate::app::App;

const WEEK_COLS: usize = 53;
const DAYS_PER_WEEK: usize = 7;

pub fn render_heatmap_page(chunks: Rc<[Rect]>, frame: &mut Frame, app: &App) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(10)])
        .split(chunks[0]);

    render_heatmap_body(body_chunks[0], frame, app);

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
    let footer = Paragraph::new("← → Year  ↑ ↓ Navigate  Tab Switch")
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

fn render_grid(chunk: Rect, frame: &mut Frame, app: &App) {
    let selected_year: i32 = app
        .years
        .get(app.counter.year_counter)
        .and_then(|y| y.parse().ok())
        .unwrap_or(2026);

    let jan1 = NaiveDate::from_ymd_opt(selected_year, 1, 1).unwrap();
    let start_weekday = jan1.weekday().num_days_from_monday() as usize;

    let days_in_year = if NaiveDate::from_ymd_opt(selected_year, 12, 31)
        .unwrap()
        .ordinal()
        == 366
    {
        366
    } else {
        365
    };

    // Split into 7 rows (Mon-Sun)
    let row_constraints = vec![Constraint::Ratio(1, DAYS_PER_WEEK as u32); DAYS_PER_WEEK];
    let row_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(chunk);

    // Split each row into 53 columns (weeks)
    let col_constraints = vec![Constraint::Ratio(1, WEEK_COLS as u32); WEEK_COLS];

    // Build a 2D array of cell Rects: cell_areas[row][col]
    let mut cell_areas: Vec<Vec<Rect>> = Vec::with_capacity(DAYS_PER_WEEK);
    for row in 0..DAYS_PER_WEEK {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints.clone())
            .split(row_areas[row]);
        cell_areas.push(cols.to_vec());
    }

    // Render each day cell
    let mut day_of_year: usize = 0;
    for col in 0..WEEK_COLS {
        for row in 0..DAYS_PER_WEEK {
            // Skip cells before Jan 1
            if col == 0 && row < start_weekday {
                continue;
            }

            if day_of_year >= days_in_year {
                break;
            }

            let date = jan1 + chrono::Duration::days(day_of_year as i64);
            let rate = app.completion_rate_for_date(date);
            let color = rate_to_color(rate);

            let cell = Block::default().bg(color);
            frame.render_widget(cell, cell_areas[row][col]);

            day_of_year += 1;
        }
    }
}

fn rate_to_color(rate: f32) -> Color {
    if rate <= 0.0 {
        Color::DarkGray
    } else if rate < 0.25 {
        Color::Rgb(14, 68, 41)
    } else if rate < 0.50 {
        Color::Rgb(0, 109, 50)
    } else if rate < 0.75 {
        Color::Rgb(38, 166, 65)
    } else {
        Color::Rgb(57, 211, 83)
    }
}

fn render_heatmap_body(chunk: Rect, frame: &mut Frame, app: &App) {
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

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let block_inner = block.inner(day_chunks[1]);
    render_grid(block_inner, frame, app);

    frame.render_widget(block, day_chunks[1]);
}
