use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(frame.area());

    let title_items = vec![
        ListItem::new(
            Line::from("🌊 Flow State 🌊")
                .style(Style::default().bold())
                .centered(),
        ),
        ListItem::new(Line::from("Progress Stats").centered()),
    ];
    let title = List::new(title_items).block(Block::default().borders(Borders::ALL));

    frame.render_widget(title, chunks[0]);

    let tab_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
        .split(chunks[1]);

    let today_tab = match app.current_screen {
        CurrentScreen::Today => {
            Paragraph::new(Line::from("Today").fg(Color::Black).bg(Color::LightYellow)).centered()
        }
        _ => Paragraph::new(Line::from("Today")).centered(),
    }
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(today_tab, tab_chunks[0]);

    let manage_tab = match app.current_screen {
        CurrentScreen::Manage => {
            Paragraph::new(Line::from("Manage").fg(Color::Black).bg(Color::LightYellow)).centered()
        }
        _ => Paragraph::new(Line::from("Manage")).centered(),
    }
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(manage_tab, tab_chunks[1]);

    let stats_tab = match app.current_screen {
        CurrentScreen::Stats => {
            Paragraph::new(Line::from("Stats").fg(Color::Black).bg(Color::LightYellow)).centered()
        }
        _ => Paragraph::new(Line::from("Stats")).centered(),
    }
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(stats_tab, tab_chunks[2]);

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(4)])
        .split(chunks[2]);

    match app.current_screen {
        CurrentScreen::Today => {
            let footer_chucks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ])
                .split(body_chunks[1]);
            let today_stats = Line::from("Today: ████████░░░░ 67% (4/6 active)")
                .fg(Color::Green)
                .centered();
            frame.render_widget(today_stats, footer_chucks[0]);
            let weekly_stats = Line::from("Week:  ███████░░░░░ 58% trending up ↗")
                .fg(Color::Green)
                .centered();
            frame.render_widget(weekly_stats, footer_chucks[1]);
            let hints = Line::from("Toggle: 1-7 • Menu: hjkl • Views: TAB")
                .fg(Color::Green)
                .centered();
            frame.render_widget(hints, footer_chucks[2]);
        }
        CurrentScreen::Manage => {
            let hints = Line::from(
                "[a] Add • [e] Edit • [d] Delete • [p] Pause/Resume • [↑↓]/[hjkl] Navigate",
            )
            .centered()
            .fg(Color::Green);
            frame.render_widget(hints, body_chunks[1]);
        }
        CurrentScreen::Stats => {
            let hints = Line::from("[P] Bulk pause • [↑↓]/[hjkl] Navigate")
                .fg(Color::Green)
                .centered();
            frame.render_widget(hints, body_chunks[1]);
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
