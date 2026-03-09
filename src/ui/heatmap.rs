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
const MONTH_GAPS: usize = 11;
const TOTAL_DRAW_COLS: usize = WEEK_COLS + MONTH_GAPS;

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

fn compute_month_start_cols(year: i32) -> [usize; 12] {
    let jan1 = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let start_weekday = jan1.weekday().num_days_from_monday() as usize;
    let days_in_year = if jan1.leap_year() { 366 } else { 365 };

    let mut starts = [0usize; 12];
    let mut day_of_year = 0usize;
    let mut current_month = 1u32;
    let mut draw_col = 0usize;

    for col in 0..WEEK_COLS {
        let mut gap_added = false;
        for row in 0..DAYS_PER_WEEK {
            if col == 0 && row < start_weekday {
                continue;
            }
            if day_of_year >= days_in_year {
                break;
            }
            let date = jan1 + chrono::Duration::days(day_of_year as i64);
            if date.month() != current_month && !gap_added {
                gap_added = true;
                draw_col += 1;
                current_month = date.month();
                starts[current_month as usize - 1] = draw_col;
            }
            day_of_year += 1;
        }
        draw_col += 1;
    }
    starts
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

fn render_month_label(chunk: Rect, frame: &mut Frame, month_start_cols: [usize; 12]) {
    let months = [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let inner = block.inner(chunk);
    let cell_w = inner.width / TOTAL_DRAW_COLS as u16;

    for (i, &label) in months.iter().enumerate() {
        let x_offset = month_start_cols[i] as u16 * cell_w;
        let next_offset = if i + 1 < 12 {
            month_start_cols[i + 1] as u16 * cell_w
        } else {
            inner.width
        };
        let w = next_offset.saturating_sub(x_offset);
        if w == 0 {
            continue;
        }
        let area = Rect::new(inner.x + x_offset, inner.y, w, inner.height);
        frame.render_widget(Paragraph::new(label).centered(), area);
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

fn cell_rect(chunk: Rect, row: usize, col: usize) -> Rect {
    let cell_w = chunk.width / TOTAL_DRAW_COLS as u16;
    let cell_h = chunk.height / DAYS_PER_WEEK as u16;
    Rect::new(
        chunk.x + col as u16 * cell_w,
        chunk.y + row as u16 * cell_h,
        cell_w,
        cell_h,
    )
}

fn render_grid(chunk: Rect, frame: &mut Frame, app: &App) {
    let selected_year: i32 = app
        .years
        .get(app.counter.year_counter)
        .and_then(|y| y.parse().ok())
        .unwrap_or(2026);

    let jan1 = NaiveDate::from_ymd_opt(selected_year, 1, 1).unwrap();
    let start_weekday = jan1.weekday().num_days_from_monday() as usize;
    let days_in_year = if jan1.leap_year() { 366 } else { 365 };

    let mut day_of_year: usize = 0;
    let mut current_month = jan1.month();
    let mut draw_col: usize = 0;

    for col in 0..WEEK_COLS {
        let mut month_gap_added = false;
        for row in 0..DAYS_PER_WEEK {
            if col == 0 && row < start_weekday {
                continue;
            }
            if day_of_year >= days_in_year {
                return;
            }

            let date = jan1 + chrono::Duration::days(day_of_year as i64);
            if date.month() != current_month && !month_gap_added {
                month_gap_added = true;
                draw_col += 1;
                current_month = date.month();
            }

            let color = rate_to_color(app.completion_rate_for_date(date));
            let area = cell_rect(chunk, row, draw_col);
            let grid = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                ])
                .split(area);
            frame.render_widget(
                Block::default()
                    .bg(color)
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL),
                grid[1],
            );
            day_of_year += 1;
        }
        draw_col += 1;
    }
}

fn rate_to_color(rate: f32) -> Color {
    if rate <= 0.0 {
        Color::default()
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
    let selected_year: i32 = app
        .years
        .get(app.counter.year_counter)
        .and_then(|y| y.parse().ok())
        .unwrap_or(2026);
    let month_start_cols = compute_month_start_cols(selected_year);

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(chunk);
    let label_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(5), Constraint::Min(1)])
        .split(body_chunks[0]);

    render_month_label(label_chunks[1], frame, month_start_cols);

    let day_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(5), Constraint::Min(1)])
        .split(body_chunks[1]);
    render_day_label(day_chunks[0], frame);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(1, 1, 1, 0));
    let block_inner = block.inner(day_chunks[1]);
    render_grid(block_inner, frame, app);
    frame.render_widget(block, day_chunks[1]);
}
