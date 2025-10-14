use chrono::Datelike;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::models::{HabitStatus, Week};

/// Draw the day detail view showing habits for the selected day
pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let selected_date = app.selected_date();
    let day_name = Week::full_weekday_name(selected_date);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Date header
            Constraint::Min(5),     // Habits list
            Constraint::Length(4),  // Notes section
        ])
        .split(area);

    // Draw date header
    let title = format!("{}, {} {}", day_name, selected_date.format("%b"), selected_date.day());
    let header_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let header = Paragraph::new(title)
        .block(header_block)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Draw habits list
    draw_habits_list(f, chunks[1], app);

    // Draw note section
    draw_note_section(f, chunks[2], app);
}

/// Draw the habits list for the selected day
fn draw_habits_list(f: &mut Frame, area: Rect, app: &App) {
    let selected_date = app.selected_date();
    let habits = app.habits();

    if habits.is_empty() {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Habits for this day");
        let text = Paragraph::new("No habits configured. Press 'h' to add habits.")
            .block(block)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(text, area);
        return;
    }

    let items: Vec<ListItem> = habits
        .iter()
        .enumerate()
        .map(|(idx, habit)| {
            let status = app.get_habit_status(habit.id, selected_date);
            let status_str = status.display_str();

            // Highlight the selected habit
            let is_selected = idx == app.selected_habit_idx;
            let prefix = if is_selected { "► " } else { "  " };

            let style = match status {
                HabitStatus::Done => Style::default().fg(Color::Green),
                HabitStatus::Skipped => Style::default().fg(Color::Red),
                HabitStatus::Unmarked => Style::default().fg(Color::Gray),
            };

            let selected_style = if is_selected {
                style.add_modifier(Modifier::BOLD)
            } else {
                style
            };

            let line = Line::from(vec![
                Span::styled(prefix, selected_style),
                Span::styled(format!("{:<20}", habit.name), selected_style),
                Span::raw("  "),
                Span::styled(status_str, selected_style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Habits for this day");
    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

/// Draw the note section for the selected habit
fn draw_note_section(f: &mut Frame, area: Rect, app: &App) {
    let note = app.get_current_note();

    let text = if let Some(note_text) = note {
        format!("Note: {}", note_text)
    } else {
        "No note for this habit. Press 'n' to add one (not yet implemented).".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Note");
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(paragraph, area);
}
