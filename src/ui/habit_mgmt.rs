use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, HabitMgmtMode};

/// Draw the habit management view
pub fn draw(f: &mut Frame, app: &App) {
    match app.habit_mgmt_mode {
        HabitMgmtMode::List => draw_habit_list(f, app),
        HabitMgmtMode::AddHabit => draw_habit_input(f, app, "Add New Habit"),
        HabitMgmtMode::EditHabit => draw_habit_input(f, app, "Edit Habit"),
    }
}

/// Draw the habit list view
fn draw_habit_list(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(5),     // Habit list
            Constraint::Length(5),  // Instructions
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("Habit Management")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Habit list
    let habits = app.habits();
    let items: Vec<ListItem> = habits
        .iter()
        .enumerate()
        .map(|(idx, habit)| {
            let prefix = if idx == app.habit_mgmt_selected_idx {
                "► "
            } else {
                "  "
            };
            let content = format!("{}{:<30} [{}]", prefix, habit.name, habit.frequency.description());
            let style = if idx == app.habit_mgmt_selected_idx {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!("Habits ({})", habits.len())));
    f.render_widget(list, chunks[1]);

    // Instructions
    let instructions = vec![
        Line::from(vec![
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(" Select  "),
            Span::styled("a", Style::default().fg(Color::Green)),
            Span::raw(" Add  "),
            Span::styled("e", Style::default().fg(Color::Yellow)),
            Span::raw(" Edit  "),
            Span::styled("d", Style::default().fg(Color::Red)),
            Span::raw(" Delete"),
        ]),
        Line::from(vec![
            Span::styled("[]", Style::default().fg(Color::Yellow)),
            Span::raw(" Move Up/Down  "),
            Span::styled("f", Style::default().fg(Color::Cyan)),
            Span::raw(" Change Frequency  "),
            Span::styled("q/Esc", Style::default().fg(Color::Green)),
            Span::raw(" Return"),
        ]),
    ];

    let instructions_widget = Paragraph::new(instructions)
        .block(Block::default().borders(Borders::ALL).title("Commands"));
    f.render_widget(instructions_widget, chunks[2]);
}

/// Draw the habit input view (for both add and edit)
fn draw_habit_input(f: &mut Frame, app: &App, title: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(3),  // Input box
            Constraint::Length(3),  // Instructions
            Constraint::Min(0),     // Spacer
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new(title)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Input box
    let input = Paragraph::new(app.input_buffer.as_str())
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Habit Name")
            .style(Style::default().fg(Color::Yellow)))
        .wrap(Wrap { trim: false });
    f.render_widget(input, chunks[1]);

    // Instructions
    let instructions = vec![
        Span::raw("Type the habit name. "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(" to save, "),
        Span::styled("Esc", Style::default().fg(Color::Red)),
        Span::raw(" to cancel."),
    ];
    let instructions_widget = Paragraph::new(Line::from(instructions))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions_widget, chunks[2]);
}
