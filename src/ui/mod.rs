// UI components for Healing-Habits TUI
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppView};

pub mod week_strip;
pub mod day_view;
pub mod stats;
pub mod habit_mgmt;

/// Main draw function - routes to appropriate view
pub fn draw(f: &mut Frame, app: &App) {
    match app.view {
        AppView::Main => draw_main_view(f, app),
        AppView::Stats => stats::draw(f, app),
        AppView::Help => draw_help_view(f, app),
        AppView::HabitManagement => habit_mgmt::draw(f, app),
        AppView::NoteInput => draw_note_input(f, app),
        AppView::ExportConfirmation => draw_export_confirmation(f, app),
    }
}

/// Draw the main view (week strip + day details)
fn draw_main_view(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Week header
            Constraint::Length(3),  // Week strip
            Constraint::Min(10),    // Day details
            Constraint::Length(3),  // Footer with shortcuts
        ])
        .split(f.area());

    // Draw week header
    draw_week_header(f, chunks[0], app);

    // Draw week strip
    week_strip::draw(f, chunks[1], app);

    // Draw day details
    day_view::draw(f, chunks[2], app);

    // Draw footer
    draw_footer(f, chunks[3]);
}

/// Draw the week header showing the week range
fn draw_week_header(f: &mut Frame, area: Rect, app: &App) {
    let title = format!("Week of {}", app.current_week.format());
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let paragraph = Paragraph::new(title)
        .block(block)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(paragraph, area);
}

/// Draw the footer with keyboard shortcuts
fn draw_footer(f: &mut Frame, area: Rect) {
    let shortcuts = vec![
        Span::raw("["),
        Span::styled("←→", Style::default().fg(Color::Yellow)),
        Span::raw("] Days  ["),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw("] Habits  ["),
        Span::styled("Space", Style::default().fg(Color::Yellow)),
        Span::raw("] Toggle  ["),
        Span::styled("h", Style::default().fg(Color::Yellow)),
        Span::raw("] Manage  ["),
        Span::styled("v", Style::default().fg(Color::Yellow)),
        Span::raw("] Stats  ["),
        Span::styled("?", Style::default().fg(Color::Yellow)),
        Span::raw("] Help  ["),
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::raw("] Quit"),
    ];

    let block = Block::default()
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(Line::from(shortcuts))
        .block(block);
    f.render_widget(paragraph, area);
}

/// Draw the help view
fn draw_help_view(f: &mut Frame, _app: &App) {
    let help_text = vec![
        Line::from(Span::styled("Healing-Habits - Keyboard Shortcuts", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(Span::styled("Navigation:", Style::default().fg(Color::Yellow))),
        Line::from("  ← / → : Move between days"),
        Line::from("  ↑ / ↓ : Select different habits"),
        Line::from("  [ / ] : Previous/Next week"),
        Line::from("  t     : Go to today"),
        Line::from(""),
        Line::from(Span::styled("Actions:", Style::default().fg(Color::Yellow))),
        Line::from("  Enter / Space : Toggle habit status (cycles through without saving)"),
        Line::from("  Esc           : Cancel staged status change"),
        Line::from("  n     : Add/edit note for selected habit"),
        Line::from(""),
        Line::from("  Status changes save automatically when you navigate away."),
        Line::from(""),
        Line::from(Span::styled("Views:", Style::default().fg(Color::Yellow))),
        Line::from("  v     : View weekly stats"),
        Line::from("  h     : Manage habits (add/edit/delete/reorder)"),
        Line::from("  x     : Export week to markdown"),
        Line::from("  ?     : Show this help"),
        Line::from(""),
        Line::from(Span::styled("Other:", Style::default().fg(Color::Yellow))),
        Line::from("  q / Esc : Return to main view / Quit"),
        Line::from("  Ctrl+C  : Quit immediately"),
        Line::from(""),
        Line::from(Span::styled("Press any key to return...", Style::default().fg(Color::Green))),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Help")
        .style(Style::default());
    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, f.area());
}

/// Draw the note input view
fn draw_note_input(f: &mut Frame, app: &App) {
    let habit_name = app.selected_habit()
        .map(|h| h.name.as_str())
        .unwrap_or("Unknown");
    let date = app.selected_date();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(5),  // Input box
            Constraint::Length(3),  // Instructions
            Constraint::Min(0),     // Spacer
        ])
        .split(f.area());

    // Header
    let header_text = format!("Edit Note for {} on {}", habit_name, date.format("%b %d, %Y"));
    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Input box
    let input_text = format!("{}", app.input_buffer);
    let input = Paragraph::new(input_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Note")
            .style(Style::default().fg(Color::Yellow)))
        .wrap(Wrap { trim: false });
    f.render_widget(input, chunks[1]);

    // Instructions
    let instructions = vec![
        Span::raw("Type your note. "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(" to save, "),
        Span::styled("Esc", Style::default().fg(Color::Red)),
        Span::raw(" to cancel."),
    ];
    let instructions_widget = Paragraph::new(Line::from(instructions))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions_widget, chunks[2]);
}

/// Draw the export confirmation view
fn draw_export_confirmation(f: &mut Frame, app: &App) {
    let file_path = app.last_export_path.as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let text = vec![
        Line::from(Span::styled("Export Successful!", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Your weekly habit report has been exported to:"),
        Line::from(""),
        Line::from(Span::styled(file_path, Style::default().fg(Color::Cyan))),
        Line::from(""),
        Line::from("You can share this report with your therapist or use it for personal reflection."),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled("Press any key to return...", Style::default().fg(Color::Yellow))),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Export Complete")
        .style(Style::default());
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, f.area());
}
