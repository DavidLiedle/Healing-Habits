use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use uuid::Uuid;

use crate::models::{Frequency, HabitStatus, Week};
use crate::storage::Storage;

/// Different screens/views in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    /// Main view showing week and day details
    Main,
    /// Weekly stats view
    Stats,
    /// Habit management screen
    HabitManagement,
    /// Help screen showing keyboard shortcuts
    Help,
    /// Note input mode
    NoteInput,
    /// Export confirmation view
    ExportConfirmation,
}

/// Habit management mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HabitMgmtMode {
    /// Viewing list of habits
    List,
    /// Adding a new habit
    AddHabit,
    /// Editing an existing habit
    EditHabit,
}

/// Application state
pub struct App {
    /// Storage backend
    pub storage: Storage,
    /// Current week being viewed
    pub current_week: Week,
    /// Selected day index (0 = Monday, 6 = Sunday)
    pub selected_day_idx: usize,
    /// Selected habit index in the current day
    pub selected_habit_idx: usize,
    /// Current view
    pub view: AppView,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Input buffer for note editing and habit management
    pub input_buffer: String,
    /// Habit management mode
    pub habit_mgmt_mode: HabitMgmtMode,
    /// Selected habit index in habit management view
    pub habit_mgmt_selected_idx: usize,
    /// Last export file path
    pub last_export_path: Option<std::path::PathBuf>,
    /// Staged status change (habit_id, date, new_status) that hasn't been saved yet
    pub staged_status: Option<(Uuid, NaiveDate, HabitStatus)>,
}

impl App {
    /// Create a new App instance
    pub fn new(data_path: impl Into<std::path::PathBuf>) -> Result<Self> {
        let mut storage = Storage::new(data_path);
        storage.load()?;

        // Start at current week and find today's index
        let current_week = Week::current();
        let today = Local::now().date_naive();
        let selected_day_idx = current_week.days()
            .iter()
            .position(|&d| d == today)
            .unwrap_or(0);

        Ok(Self {
            storage,
            current_week,
            selected_day_idx,
            selected_habit_idx: 0,
            view: AppView::Main,
            should_quit: false,
            input_buffer: String::new(),
            habit_mgmt_mode: HabitMgmtMode::List,
            habit_mgmt_selected_idx: 0,
            last_export_path: None,
            staged_status: None,
        })
    }

    /// Get the currently selected date
    pub fn selected_date(&self) -> NaiveDate {
        self.current_week.day(self.selected_day_idx).unwrap()
    }

    /// Get habits sorted by order
    pub fn habits(&self) -> Vec<&crate::models::Habit> {
        self.storage.habits()
    }

    /// Get habits that should be shown for a given date, respecting frequency
    pub fn habits_for_date(&self, date: NaiveDate) -> Vec<&crate::models::Habit> {
        self.storage.habits().into_iter()
            .filter(|habit| self.should_show_habit(habit, date))
            .collect()
    }

    /// Check if a habit should be shown on a given date based on its frequency
    fn should_show_habit(&self, habit: &crate::models::Habit, _date: NaiveDate) -> bool {
        // All habits show every day regardless of frequency
        // Frequency is informational only (tells you how often to do it)
        match habit.frequency {
            Frequency::Daily => true,
            Frequency::Weekly => true,
            Frequency::AsNeeded => true,
        }
    }

    /// Get the currently selected habit (from the filtered list for the selected date)
    pub fn selected_habit(&self) -> Option<&crate::models::Habit> {
        let date = self.selected_date();
        self.habits_for_date(date).get(self.selected_habit_idx).copied()
    }

    /// Navigate to the previous day
    pub fn prev_day(&mut self) -> Result<()> {
        // Commit any staged changes before navigating
        self.commit_staged_status()?;

        if self.selected_day_idx > 0 {
            self.selected_day_idx -= 1;
        } else {
            // Wrap to previous week's Sunday
            self.current_week = self.current_week.prev();
            self.selected_day_idx = 6;
        }
        // Reset habit selection when changing days
        self.selected_habit_idx = 0;
        Ok(())
    }

    /// Navigate to the next day
    pub fn next_day(&mut self) -> Result<()> {
        // Commit any staged changes before navigating
        self.commit_staged_status()?;

        if self.selected_day_idx < 6 {
            self.selected_day_idx += 1;
        } else {
            // Wrap to next week's Monday
            self.current_week = self.current_week.next();
            self.selected_day_idx = 0;
        }
        // Reset habit selection when changing days
        self.selected_habit_idx = 0;
        Ok(())
    }

    /// Navigate to the previous habit
    pub fn prev_habit(&mut self) -> Result<()> {
        // Commit any staged changes before navigating
        self.commit_staged_status()?;

        let date = self.selected_date();
        let habit_count = self.habits_for_date(date).len();
        if habit_count > 0 {
            if self.selected_habit_idx > 0 {
                self.selected_habit_idx -= 1;
            } else {
                self.selected_habit_idx = habit_count - 1;
            }
        }
        Ok(())
    }

    /// Navigate to the next habit
    pub fn next_habit(&mut self) -> Result<()> {
        // Commit any staged changes before navigating
        self.commit_staged_status()?;

        let date = self.selected_date();
        let habit_count = self.habits_for_date(date).len();
        if habit_count > 0 {
            if self.selected_habit_idx < habit_count - 1 {
                self.selected_habit_idx += 1;
            } else {
                self.selected_habit_idx = 0;
            }
        }
        Ok(())
    }

    /// Navigate to the previous week
    pub fn prev_week(&mut self) {
        self.current_week = self.current_week.prev();
    }

    /// Navigate to the next week
    pub fn next_week(&mut self) {
        self.current_week = self.current_week.next();
    }

    /// Go to the current week and today
    pub fn go_to_today(&mut self) {
        self.current_week = Week::current();
        let today = Local::now().date_naive();
        self.selected_day_idx = self.current_week.days()
            .iter()
            .position(|&d| d == today)
            .unwrap_or(0);
    }

    /// Toggle the status of the selected habit for the selected date (stages change, doesn't save)
    pub fn toggle_habit_status(&mut self) {
        if let Some(habit) = self.selected_habit() {
            let date = self.selected_date();
            let current_status = self.get_habit_status(habit.id, date);
            let new_status = current_status.cycle();

            // Stage the change instead of saving immediately
            self.staged_status = Some((habit.id, date, new_status));
        }
    }

    /// Commit any staged status changes to storage
    pub fn commit_staged_status(&mut self) -> Result<()> {
        if let Some((habit_id, date, status)) = self.staged_status.take() {
            self.storage.update_log_status(habit_id, date, status)?;

            // For Weekly habits marked as Done, auto-fill the week
            if let Some(habit) = self.storage.get_habit(habit_id) {
                if habit.frequency == Frequency::Weekly && status == HabitStatus::Done {
                    self.propagate_weekly_habit_status(habit_id, date)?;
                }
            }
        }
        Ok(())
    }

    /// Propagate Weekly habit status across the week
    /// When marked Done on a day, mark previous days as Skipped (if unmarked)
    /// Future days remain unmarked (no entry needed)
    fn propagate_weekly_habit_status(&mut self, habit_id: Uuid, done_date: NaiveDate) -> Result<()> {
        let week = Week::containing(done_date);
        let days = week.days();

        for &day in &days {
            if day < done_date {
                // Previous days in the week: mark as Skipped (if unmarked)
                let current_status = self.storage.get_log(habit_id, day)
                    .map(|log| log.status)
                    .unwrap_or(HabitStatus::Unmarked);

                if current_status == HabitStatus::Unmarked {
                    self.storage.update_log_status(habit_id, day, HabitStatus::Skipped)?;
                }
            }
            // Future days (day > done_date) remain unmarked
            // day == done_date is already marked as Done by the initial update
        }

        Ok(())
    }

    /// Cancel any staged status changes without saving
    pub fn cancel_staged_status(&mut self) {
        self.staged_status = None;
    }

    /// Get the status for a habit on a specific date (checks staged changes first)
    pub fn get_habit_status(&self, habit_id: Uuid, date: NaiveDate) -> HabitStatus {
        // Check if there's a staged change for this habit and date
        if let Some((staged_id, staged_date, staged_status)) = self.staged_status {
            if staged_id == habit_id && staged_date == date {
                return staged_status;
            }
        }

        // Otherwise get from storage
        self.storage.get_log(habit_id, date)
            .map(|log| log.status)
            .unwrap_or(HabitStatus::Unmarked)
    }

    /// Get the note for the selected habit and date
    pub fn get_current_note(&self) -> Option<String> {
        if let Some(habit) = self.selected_habit() {
            let date = self.selected_date();
            self.storage.get_log(habit.id, date)
                .and_then(|log| log.note.clone())
        } else {
            None
        }
    }

    /// Update the note for the selected habit and date
    pub fn update_current_note(&mut self, note: Option<String>) -> Result<()> {
        if let Some(habit) = self.selected_habit() {
            let date = self.selected_date();
            self.storage.update_log_note(habit.id, date, note)?;
        }
        Ok(())
    }

    /// Get the day status symbol for a specific day
    /// ✓ = All habits done, ✗ = Some skipped, ~ = Partial, space = Unmarked/future
    pub fn get_day_status(&self, day_idx: usize) -> char {
        let date = self.current_week.day(day_idx).unwrap();
        let habits = self.habits_for_date(date);

        if habits.is_empty() {
            return ' ';
        }

        let mut done_count = 0;
        let mut skipped_count = 0;
        let mut unmarked_count = 0;

        for habit in &habits {
            match self.get_habit_status(habit.id, date) {
                HabitStatus::Done => done_count += 1,
                HabitStatus::Skipped => skipped_count += 1,
                HabitStatus::Unmarked => unmarked_count += 1,
            }
        }

        // If all unmarked or future date, show space
        if unmarked_count == habits.len() || date > Local::now().date_naive() {
            ' '
        } else if done_count == habits.len() {
            '✓'
        } else if skipped_count > 0 {
            '✗'
        } else {
            '~'
        }
    }

    /// Change the current view
    pub fn set_view(&mut self, view: AppView) {
        self.view = view;
    }

    /// Request the app to quit
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Enter note editing mode
    pub fn start_note_input(&mut self) {
        // Load existing note if any
        self.input_buffer = self.get_current_note().unwrap_or_default();
        self.view = AppView::NoteInput;
    }

    /// Save the note and return to main view
    pub fn save_note_input(&mut self) -> Result<()> {
        let note = if self.input_buffer.trim().is_empty() {
            None
        } else {
            Some(self.input_buffer.trim().to_string())
        };
        self.update_current_note(note)?;
        self.input_buffer.clear();
        self.view = AppView::Main;
        Ok(())
    }

    /// Cancel note editing and return to main view
    pub fn cancel_note_input(&mut self) {
        self.input_buffer.clear();
        self.view = AppView::Main;
    }

    /// Handle character input for note editing
    pub fn input_char(&mut self, c: char) {
        self.input_buffer.push(c);
    }

    /// Handle backspace in note editing
    pub fn input_backspace(&mut self) {
        self.input_buffer.pop();
    }

    // Habit Management Methods

    /// Enter habit management view
    pub fn enter_habit_management(&mut self) {
        self.habit_mgmt_mode = HabitMgmtMode::List;
        self.habit_mgmt_selected_idx = 0;
        self.view = AppView::HabitManagement;
    }

    /// Start adding a new habit
    pub fn start_add_habit(&mut self) {
        self.input_buffer.clear();
        self.habit_mgmt_mode = HabitMgmtMode::AddHabit;
    }

    /// Start editing the selected habit
    pub fn start_edit_habit(&mut self) {
        if let Some(habit) = self.habits().get(self.habit_mgmt_selected_idx) {
            self.input_buffer = habit.name.clone();
            self.habit_mgmt_mode = HabitMgmtMode::EditHabit;
        }
    }

    /// Save new habit
    pub fn save_new_habit(&mut self) -> Result<()> {
        if !self.input_buffer.trim().is_empty() {
            self.storage.add_habit(self.input_buffer.trim().to_string())?;
        }
        self.input_buffer.clear();
        self.habit_mgmt_mode = HabitMgmtMode::List;
        Ok(())
    }

    /// Save edited habit
    pub fn save_edited_habit(&mut self) -> Result<()> {
        if !self.input_buffer.trim().is_empty() {
            if let Some(habit) = self.habits().get(self.habit_mgmt_selected_idx) {
                let habit_id = habit.id;
                self.storage.update_habit_name(habit_id, self.input_buffer.trim().to_string())?;
            }
        }
        self.input_buffer.clear();
        self.habit_mgmt_mode = HabitMgmtMode::List;
        Ok(())
    }

    /// Cancel habit input
    pub fn cancel_habit_input(&mut self) {
        self.input_buffer.clear();
        self.habit_mgmt_mode = HabitMgmtMode::List;
    }

    /// Delete the selected habit
    pub fn delete_selected_habit(&mut self) -> Result<()> {
        if let Some(habit) = self.habits().get(self.habit_mgmt_selected_idx) {
            let habit_id = habit.id;
            self.storage.remove_habit(habit_id)?;
            // Adjust selected index if necessary
            let habit_count = self.habits().len();
            if self.habit_mgmt_selected_idx >= habit_count && habit_count > 0 {
                self.habit_mgmt_selected_idx = habit_count - 1;
            }
        }
        Ok(())
    }

    /// Move selected habit up in the list
    pub fn move_habit_up(&mut self) -> Result<()> {
        if self.habit_mgmt_selected_idx > 0 {
            let habits = self.habits();
            if let Some(habit) = habits.get(self.habit_mgmt_selected_idx) {
                let habit_id = habit.id;
                let new_order = self.habit_mgmt_selected_idx - 1;
                self.storage.reorder_habit(habit_id, new_order)?;
                self.habit_mgmt_selected_idx = new_order;
            }
        }
        Ok(())
    }

    /// Move selected habit down in the list
    pub fn move_habit_down(&mut self) -> Result<()> {
        let habit_count = self.habits().len();
        if self.habit_mgmt_selected_idx < habit_count.saturating_sub(1) {
            let habits = self.habits();
            if let Some(habit) = habits.get(self.habit_mgmt_selected_idx) {
                let habit_id = habit.id;
                let new_order = self.habit_mgmt_selected_idx + 1;
                self.storage.reorder_habit(habit_id, new_order)?;
                self.habit_mgmt_selected_idx = new_order;
            }
        }
        Ok(())
    }

    /// Navigate to previous habit in management view
    pub fn habit_mgmt_prev(&mut self) {
        let habit_count = self.habits().len();
        if habit_count > 0 && self.habit_mgmt_selected_idx > 0 {
            self.habit_mgmt_selected_idx -= 1;
        }
    }

    /// Navigate to next habit in management view
    pub fn habit_mgmt_next(&mut self) {
        let habit_count = self.habits().len();
        if habit_count > 0 && self.habit_mgmt_selected_idx < habit_count - 1 {
            self.habit_mgmt_selected_idx += 1;
        }
    }

    /// Cycle through frequencies for the selected habit
    pub fn cycle_habit_frequency(&mut self) -> Result<()> {
        if let Some(habit) = self.habits().get(self.habit_mgmt_selected_idx) {
            let habit_id = habit.id;
            let current_frequency = habit.frequency;

            let new_frequency = match current_frequency {
                Frequency::Daily => Frequency::Weekly,
                Frequency::Weekly => Frequency::AsNeeded,
                Frequency::AsNeeded => Frequency::Daily,
            };

            self.storage.update_habit_frequency(habit_id, new_frequency)?;
        }
        Ok(())
    }

    // Export Methods

    /// Export the current week's data to markdown format
    pub fn export_week_to_markdown(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&format!("# Habit Tracking Report\n\n"));
        output.push_str(&format!("**Week of {}**\n\n", self.current_week.format()));
        output.push_str(&format!("Generated: {}\n\n", Local::now().format("%B %d, %Y at %I:%M %p")));

        // Weekly summary
        output.push_str("## Weekly Summary\n\n");
        let days = self.current_week.days();
        let habits = self.habits();

        if habits.is_empty() {
            output.push_str("*No habits tracked this week.*\n\n");
            return output;
        }

        // Calculate weekly stats
        let mut weekly_stats: Vec<(String, usize, usize, usize)> = Vec::new();
        for habit in &habits {
            let mut done = 0;
            let mut skipped = 0;
            let mut unmarked = 0;

            for &date in &days {
                match self.get_habit_status(habit.id, date) {
                    HabitStatus::Done => done += 1,
                    HabitStatus::Skipped => skipped += 1,
                    HabitStatus::Unmarked => unmarked += 1,
                }
            }

            weekly_stats.push((habit.name.clone(), done, skipped, unmarked));
        }

        // Display stats table
        output.push_str("| Habit | Done | Skipped | Unmarked | Completion Rate |\n");
        output.push_str("|-------|------|---------|----------|------------------|\n");

        for (name, done, skipped, unmarked) in &weekly_stats {
            let total_tracked = done + skipped;
            let rate = if total_tracked > 0 {
                ((*done as f64 / total_tracked as f64) * 100.0) as usize
            } else {
                0
            };
            output.push_str(&format!("| {} | {} | {} | {} | {}% |\n",
                name, done, skipped, unmarked, rate));
        }
        output.push_str("\n");

        // Daily breakdown
        output.push_str("## Daily Breakdown\n\n");

        for &date in &days {
            let weekday = Week::full_weekday_name(date);
            output.push_str(&format!("### {} - {}\n\n", weekday, date.format("%B %d, %Y")));

            let mut has_activity = false;

            for habit in &habits {
                let status = self.get_habit_status(habit.id, date);
                let status_str = match status {
                    HabitStatus::Done => "✓ Done",
                    HabitStatus::Skipped => "✗ Skipped",
                    HabitStatus::Unmarked => "○ Not tracked",
                };

                output.push_str(&format!("- **{}**: {}\n", habit.name, status_str));

                // Include notes if present
                if let Some(log) = self.storage.get_log(habit.id, date) {
                    if let Some(note) = &log.note {
                        if !note.trim().is_empty() {
                            output.push_str(&format!("  *Note: {}*\n", note));
                            has_activity = true;
                        }
                    }
                }

                if status != HabitStatus::Unmarked {
                    has_activity = true;
                }
            }

            if !has_activity {
                output.push_str("*No activity recorded for this day.*\n");
            }

            output.push_str("\n");
        }

        // Footer
        output.push_str("---\n\n");
        output.push_str("*Report generated by Healing-Habits habit tracker*\n");

        output
    }

    /// Export current week and save to file
    pub fn export_and_show_confirmation(&mut self) -> Result<()> {
        let markdown = self.export_week_to_markdown();

        // Determine export directory
        let export_dir = dirs::home_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join("Documents")
            .join("healing-habits-exports");

        std::fs::create_dir_all(&export_dir)
            .context("Failed to create export directory")?;

        // Generate filename with date
        let filename = format!(
            "habit-report-{}.md",
            self.current_week.days()[0].format("%Y-%m-%d")
        );
        let file_path = export_dir.join(filename);

        std::fs::write(&file_path, markdown)
            .context("Failed to write export file")?;

        self.last_export_path = Some(file_path);
        self.view = AppView::ExportConfirmation;

        Ok(())
    }
}
