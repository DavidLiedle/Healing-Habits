use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

/// Draw the weekly stats view
pub fn draw(f: &mut Frame, app: &App) {
    let stats = app.storage.get_stats(app.current_week.start, app.current_week.end());
    let habits = app.habits();

    let mut items = vec![
        ListItem::new(Line::from(Span::styled(
            format!("Weekly Stats - {}", app.current_week.format()),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))),
        ListItem::new(Line::from("")),
    ];

    if habits.is_empty() {
        items.push(ListItem::new("No habits tracked yet."));
    } else {
        for habit in habits {
            if let Some((done, skipped, unmarked)) = stats.get(&habit.id) {
                let total = done + skipped + unmarked;
                let completion_pct = if total > 0 {
                    (done * 100) / total
                } else {
                    0
                };

                items.push(ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("{:<20}", habit.name),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::raw(format!(
                        " Done: {}/7 ({}%)  Skipped: {}  Unmarked: {}",
                        done, completion_pct, skipped, unmarked
                    )),
                ])));
            }
        }
    }

    items.push(ListItem::new(Line::from("")));
    items.push(ListItem::new(Line::from(Span::styled(
        "Press 'q' or Esc to return",
        Style::default().fg(Color::Green),
    ))));

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Weekly Statistics")
        .style(Style::default());
    let list = List::new(items).block(block);
    f.render_widget(list, f.area());
}
