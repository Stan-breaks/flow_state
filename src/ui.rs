use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
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
            let habit_chucks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(body_chunks[0]);

            let build_habit = List::new(
                app.build_habits
                    .iter()
                    .enumerate()
                    .map(|(index, habit)| {
                        if index + 1 == app.habits_counter {
                            ListItem::new(format!(
                                "[{}]. {}, {}  {}",
                                index + 1,
                                habit.name,
                                habit.created,
                                habit.days_completed.len()
                            ))
                            .bg(Color::Green)
                        } else {
                            ListItem::new(format!(
                                "[{}]. {}, {}  {}",
                                index + 1,
                                habit.name,
                                habit.created,
                                habit.days_completed.len()
                            ))
                        }
                    })
                    .collect::<Vec<ListItem>>(),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🌟 Build These Habits")
                    .border_style(Style::default().fg(Color::Green)),
            );
            frame.render_widget(build_habit, habit_chucks[0]);
            let build_habits_len = app.build_habits.len() + 1;
            let avoid_habit = List::new(
                app.avoid_habits
                    .iter()
                    .enumerate()
                    .map(|(index, habit)| {
                        if index + build_habits_len == app.habits_counter {
                            ListItem::new(format!(
                                "[{}]. {}, {}  {}",
                                index + build_habits_len,
                                habit.name,
                                habit.created,
                                habit.days_completed.len()
                            ))
                            .bg(Color::Red)
                        } else {
                            ListItem::new(format!(
                                "[{}]. {}, {}  {}",
                                index + build_habits_len,
                                habit.name,
                                habit.created,
                                habit.days_completed.len()
                            ))
                        }
                    })
                    .collect::<Vec<ListItem>>(),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🚫 Avoid These Habits")
                    .border_style(Style::default().fg(Color::Red)),
            );
            frame.render_widget(avoid_habit, habit_chucks[1]);
            let footer_area = body_chunks[1];
            let footer_block = Block::default().borders(Borders::ALL);
            frame.render_widget(&footer_block, footer_area);

            let inner_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(1), Constraint::Min(1)])
                .split(footer_block.inner(footer_area));

            let stat_lines = vec![
                ListItem::new(
                    Line::from("Today: ████████░░░░ 67% (4/6 active)")
                        .fg(Color::Green)
                        .centered(),
                ),
                ListItem::new(
                    Line::from("Week: ███████░░░░░ 58% trending up ↗")
                        .fg(Color::Green)
                        .centered(),
                ),
            ];
            let stat_list = List::new(stat_lines);
            frame.render_widget(stat_list, inner_chunks[0]);

            let hint = Paragraph::new(
                Line::from("[1-7] Toggle Habits  •[↑↓]/[hjkl] Navigate • [TAB] Switch Views")
                    .fg(Color::Green)
                    .centered(),
            );
            frame.render_widget(hint, inner_chunks[1]);
        }
        CurrentScreen::Manage => {
            let hints = Paragraph::new(
                Line::from(
                    "[a] Add • [e] Edit • [d] Delete • [p] Pause/Resume • [↑↓]/[hjkl] Navigate",
                )
                .centered()
                .fg(Color::Green),
            )
            .block(Block::default().borders(Borders::ALL));
            frame.render_widget(hints, body_chunks[1]);
        }
        CurrentScreen::Stats => {
            let hints = Paragraph::new(
                Line::from("[P] Bulk pause • [↑↓]/[hjkl] Navigate")
                    .fg(Color::Green)
                    .centered(),
            )
            .block(Block::default().borders(Borders::ALL));
            frame.render_widget(hints, body_chunks[1]);
        }
    }
}
