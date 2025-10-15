use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::models::Week;

/// Draw the week strip showing 7 days with status symbols
pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // Split into 7 equal columns for each day
    let day_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(15),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(15),
        ])
        .split(block.inner(area));

    f.render_widget(block, area);

    // Draw each day
    for day_idx in 0..7 {
        draw_day(f, day_chunks[day_idx], app, day_idx);
    }
}

/// Draw a single day in the week strip
fn draw_day(f: &mut Frame, area: Rect, app: &App, day_idx: usize) {
    let day_name = Week::weekday_name(day_idx);
    let status_symbol = app.get_day_status(day_idx);

    // Highlight if this is the selected day
    let is_selected = day_idx == app.selected_day_idx;
    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    // Combine day name and status symbol on the same line
    let text = vec![
        Line::from(vec![
            Span::styled(format!("{} ", day_name), style),
            Span::styled(format!("[{}]", status_symbol), style),
        ]),
    ];

    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, area);
}
