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

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Today => Span::styled(
                "Today",
                Style::default().bg(Color::LightYellow).fg(Color::Green),
            ),
            CurrentScreen::Manage => Span::styled(
                "Manage",
                Style::default().bg(Color::LightYellow).fg(Color::Green),
            ),
            CurrentScreen::Stats => Span::styled(
                "Stats",
                Style::default().bg(Color::LightYellow).fg(Color::Green),
            ),
        }
        .to_owned(),
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Today => Span::styled(
                "Today: ████████░░░░ 67% (4/6 active) \nWeek:  ███████░░░░░ 58% trending up ↗\nToggle: 1-7 • Menu: hjkl • Views: TAB",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Manage => Span::styled(
                "[a] Add • [e] Edit • [d] Delete  [p] Pause/Resume • [↑↓] Navigate ",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Stats => Span::styled(
                " [←→] Navigate • [P] Bulk pause • [s] Accept suggestion • [r] Reset habit",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
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
