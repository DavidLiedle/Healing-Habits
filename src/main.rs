use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::time::Duration;

use healing_habits::app::{App, AppView};
use healing_habits::ui;

fn main() -> Result<()> {
    // Get data directory path
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap().join(".data"))
        .join("healing-habits");
    std::fs::create_dir_all(&data_dir)?;
    let data_path = data_dir.join("habits.json");

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(data_path)?;

    // Run the app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Check for Ctrl+C
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.quit();
                    break;
                }

                handle_key_event(app, key.code)?;
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyCode) -> Result<()> {
    match app.view {
        AppView::Main => handle_main_view_keys(app, key)?,
        AppView::Stats => handle_stats_view_keys(app, key)?,
        AppView::Help => handle_help_view_keys(app, key)?,
        AppView::HabitManagement => handle_habit_mgmt_keys(app, key)?,
        AppView::NoteInput => handle_note_input_keys(app, key)?,
        AppView::ExportConfirmation => handle_export_confirmation_keys(app, key)?,
    }
    Ok(())
}

fn handle_main_view_keys(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Left => app.prev_day(),
        KeyCode::Right => app.next_day(),
        KeyCode::Up => app.prev_habit(),
        KeyCode::Down => app.next_habit(),
        KeyCode::Enter => app.toggle_habit_status()?,
        KeyCode::Char('n') => app.start_note_input(),
        KeyCode::Char('w') => {
            // Week navigation
            // For now, just go to current week
            app.go_to_today();
        }
        KeyCode::Char('v') => app.set_view(AppView::Stats),
        KeyCode::Char('h') => app.enter_habit_management(),
        KeyCode::Char('?') => app.set_view(AppView::Help),
        KeyCode::Char('t') => app.go_to_today(),
        KeyCode::Char('[') => app.prev_week(),
        KeyCode::Char(']') => app.next_week(),
        KeyCode::Char('x') => app.export_and_show_confirmation()?,
        _ => {}
    }
    Ok(())
}

fn handle_stats_view_keys(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.set_view(AppView::Main),
        _ => {}
    }
    Ok(())
}

fn handle_help_view_keys(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.set_view(AppView::Main),
        _ => {}
    }
    Ok(())
}

fn handle_habit_mgmt_keys(app: &mut App, key: KeyCode) -> Result<()> {
    use healing_habits::app::HabitMgmtMode;

    match app.habit_mgmt_mode {
        HabitMgmtMode::List => {
            match key {
                KeyCode::Char('q') | KeyCode::Esc => app.set_view(AppView::Main),
                KeyCode::Up => app.habit_mgmt_prev(),
                KeyCode::Down => app.habit_mgmt_next(),
                KeyCode::Char('a') => app.start_add_habit(),
                KeyCode::Char('e') => app.start_edit_habit(),
                KeyCode::Char('d') => app.delete_selected_habit()?,
                KeyCode::Char('[') => app.move_habit_up()?,
                KeyCode::Char(']') => app.move_habit_down()?,
                _ => {}
            }
        }
        HabitMgmtMode::AddHabit => {
            match key {
                KeyCode::Enter => app.save_new_habit()?,
                KeyCode::Esc => app.cancel_habit_input(),
                KeyCode::Char(c) => app.input_char(c),
                KeyCode::Backspace => app.input_backspace(),
                _ => {}
            }
        }
        HabitMgmtMode::EditHabit => {
            match key {
                KeyCode::Enter => app.save_edited_habit()?,
                KeyCode::Esc => app.cancel_habit_input(),
                KeyCode::Char(c) => app.input_char(c),
                KeyCode::Backspace => app.input_backspace(),
                _ => {}
            }
        }
    }
    Ok(())
}

fn handle_note_input_keys(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Enter => app.save_note_input()?,
        KeyCode::Esc => app.cancel_note_input(),
        KeyCode::Char(c) => app.input_char(c),
        KeyCode::Backspace => app.input_backspace(),
        _ => {}
    }
    Ok(())
}

fn handle_export_confirmation_keys(app: &mut App, _key: KeyCode) -> Result<()> {
    // Any key returns to main view
    app.set_view(AppView::Main);
    Ok(())
}
