use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use crate::models::{Frequency, Habit, HabitLog, HabitStatus};

/// Storage container for all habit tracking data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitData {
    /// All habits being tracked
    pub habits: Vec<Habit>,
    /// Log entries for habits on specific dates
    pub logs: Vec<HabitLog>,
}

impl Default for HabitData {
    fn default() -> Self {
        Self {
            habits: crate::models::habit::default_habits(),
            logs: Vec::new(),
        }
    }
}

/// Manages persistence of habit data to/from JSON
pub struct Storage {
    file_path: PathBuf,
    data: HabitData,
}

impl Storage {
    /// Create a new storage instance with the given file path
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
            data: HabitData::default(),
        }
    }

    /// Load data from disk, or create new data if file doesn't exist
    pub fn load(&mut self) -> Result<()> {
        if self.file_path.exists() {
            let contents = fs::read_to_string(&self.file_path)
                .context("Failed to read habit data file")?;

            // Handle empty file (treat as new)
            if contents.trim().is_empty() {
                self.data = HabitData::default();
                self.save()?;
            } else {
                self.data = serde_json::from_str(&contents)
                    .context("Failed to parse habit data JSON")?;
            }
        } else {
            // File doesn't exist, use default data
            self.data = HabitData::default();
            // Create parent directory if needed
            if let Some(parent) = self.file_path.parent() {
                fs::create_dir_all(parent)
                    .context("Failed to create data directory")?;
            }
            // Save the default data
            self.save()?;
        }
        Ok(())
    }

    /// Save current data to disk
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.data)
            .context("Failed to serialize habit data")?;
        fs::write(&self.file_path, json)
            .context("Failed to write habit data file")?;
        Ok(())
    }

    /// Get all habits, sorted by order
    pub fn habits(&self) -> Vec<&Habit> {
        let mut habits: Vec<&Habit> = self.data.habits.iter().collect();
        habits.sort_by_key(|h| h.order);
        habits
    }

    /// Get a habit by ID
    pub fn get_habit(&self, id: Uuid) -> Option<&Habit> {
        self.data.habits.iter().find(|h| h.id == id)
    }

    /// Add a new habit
    pub fn add_habit(&mut self, name: String) -> Result<()> {
        let order = self.data.habits.len();
        let mut habit = Habit::new(&name);
        habit.order = order;
        self.data.habits.push(habit);
        self.save()
    }

    /// Update an existing habit
    pub fn update_habit(&mut self, habit: Habit) -> Result<()> {
        if let Some(existing) = self.data.habits.iter_mut().find(|h| h.id == habit.id) {
            *existing = habit;
            self.save()
        } else {
            anyhow::bail!("Habit not found")
        }
    }

    /// Delete a habit and all its logs
    pub fn delete_habit(&mut self, id: Uuid) -> Result<()> {
        self.data.habits.retain(|h| h.id != id);
        self.data.logs.retain(|l| l.habit_id != id);
        self.save()
    }

    /// Update a habit's name
    pub fn update_habit_name(&mut self, id: Uuid, name: String) -> Result<()> {
        if let Some(habit) = self.data.habits.iter_mut().find(|h| h.id == id) {
            habit.name = name;
            self.save()
        } else {
            anyhow::bail!("Habit not found")
        }
    }

    /// Update a habit's frequency
    pub fn update_habit_frequency(&mut self, id: Uuid, frequency: Frequency) -> Result<()> {
        if let Some(habit) = self.data.habits.iter_mut().find(|h| h.id == id) {
            habit.frequency = frequency;
            self.save()
        } else {
            anyhow::bail!("Habit not found")
        }
    }

    /// Remove a habit (alias for delete_habit)
    pub fn remove_habit(&mut self, id: Uuid) -> Result<()> {
        self.delete_habit(id)
    }

    /// Reorder a habit to a new position
    pub fn reorder_habit(&mut self, id: Uuid, new_order: usize) -> Result<()> {
        // Find the habit's current position
        if let Some(current_idx) = self.data.habits.iter().position(|h| h.id == id) {
            let habit = self.data.habits.remove(current_idx);
            let insert_idx = new_order.min(self.data.habits.len());
            self.data.habits.insert(insert_idx, habit);

            // Update all habit orders
            for (idx, habit) in self.data.habits.iter_mut().enumerate() {
                habit.order = idx;
            }

            self.save()
        } else {
            anyhow::bail!("Habit not found")
        }
    }

    /// Get a log entry for a specific habit and date
    pub fn get_log(&self, habit_id: Uuid, date: NaiveDate) -> Option<&HabitLog> {
        self.data.logs.iter()
            .find(|l| l.habit_id == habit_id && l.date == date)
    }

    /// Get all logs for a specific date
    pub fn get_logs_for_date(&self, date: NaiveDate) -> Vec<&HabitLog> {
        self.data.logs.iter()
            .filter(|l| l.date == date)
            .collect()
    }

    /// Get or create a log entry for a habit and date
    pub fn get_or_create_log(&mut self, habit_id: Uuid, date: NaiveDate) -> &mut HabitLog {
        // Check if log exists
        if let Some(pos) = self.data.logs.iter().position(|l| l.habit_id == habit_id && l.date == date) {
            &mut self.data.logs[pos]
        } else {
            // Create new log
            let log = HabitLog::new(habit_id, date);
            self.data.logs.push(log);
            self.data.logs.last_mut().unwrap()
        }
    }

    /// Update a log entry status
    pub fn update_log_status(&mut self, habit_id: Uuid, date: NaiveDate, status: HabitStatus) -> Result<()> {
        let log = self.get_or_create_log(habit_id, date);
        log.status = status;
        self.save()
    }

    /// Toggle a log entry status
    pub fn toggle_log_status(&mut self, habit_id: Uuid, date: NaiveDate) -> Result<HabitStatus> {
        let log = self.get_or_create_log(habit_id, date);
        log.toggle_status();
        let new_status = log.status;
        self.save()?;
        Ok(new_status)
    }

    /// Update a log entry note
    pub fn update_log_note(&mut self, habit_id: Uuid, date: NaiveDate, note: Option<String>) -> Result<()> {
        let log = self.get_or_create_log(habit_id, date);
        log.set_note(note);
        self.save()
    }

    /// Get completion statistics for a date range
    pub fn get_stats(&self, start_date: NaiveDate, end_date: NaiveDate) -> HashMap<Uuid, (usize, usize, usize)> {
        let mut stats: HashMap<Uuid, (usize, usize, usize)> = HashMap::new();

        for habit in &self.data.habits {
            let mut done = 0;
            let mut skipped = 0;
            let mut unmarked = 0;

            let mut current = start_date;
            while current <= end_date {
                if let Some(log) = self.get_log(habit.id, current) {
                    match log.status {
                        HabitStatus::Done => done += 1,
                        HabitStatus::Skipped => skipped += 1,
                        HabitStatus::Unmarked => unmarked += 1,
                    }
                } else {
                    unmarked += 1;
                }
                current = current.succ_opt().unwrap();
            }

            stats.insert(habit.id, (done, skipped, unmarked));
        }

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_storage_new() {
        let temp_file = NamedTempFile::new().unwrap();
        let storage = Storage::new(temp_file.path());
        assert_eq!(storage.data.habits.len(), 4); // Default habits
    }

    #[test]
    fn test_storage_save_and_load() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        {
            let mut storage = Storage::new(&path);
            storage.load().unwrap();
            assert_eq!(storage.data.habits.len(), 4);
        }

        // Load again to verify persistence
        {
            let mut storage = Storage::new(&path);
            storage.load().unwrap();
            assert_eq!(storage.data.habits.len(), 4);
        }
    }

    #[test]
    fn test_add_and_get_habit() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut storage = Storage::new(temp_file.path());
        storage.load().unwrap();

        let initial_count = storage.data.habits.len();
        storage.add_habit("Test Habit".to_string()).unwrap();

        assert_eq!(storage.data.habits.len(), initial_count + 1);
        let retrieved = storage.data.habits.iter().find(|h| h.name == "Test Habit").unwrap();
        assert_eq!(retrieved.name, "Test Habit");
    }

    #[test]
    fn test_toggle_log_status() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut storage = Storage::new(temp_file.path());
        storage.load().unwrap();

        let habit_id = storage.data.habits[0].id;
        let date = NaiveDate::from_ymd_opt(2025, 10, 14).unwrap();

        // First toggle: Unmarked -> Done
        let status = storage.toggle_log_status(habit_id, date).unwrap();
        assert_eq!(status, HabitStatus::Done);

        // Second toggle: Done -> Skipped
        let status = storage.toggle_log_status(habit_id, date).unwrap();
        assert_eq!(status, HabitStatus::Skipped);

        // Third toggle: Skipped -> Unmarked
        let status = storage.toggle_log_status(habit_id, date).unwrap();
        assert_eq!(status, HabitStatus::Unmarked);
    }

    #[test]
    fn test_update_log_note() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut storage = Storage::new(temp_file.path());
        storage.load().unwrap();

        let habit_id = storage.data.habits[0].id;
        let date = NaiveDate::from_ymd_opt(2025, 10, 14).unwrap();

        storage.update_log_note(habit_id, date, Some("Test note".to_string())).unwrap();

        let log = storage.get_log(habit_id, date).unwrap();
        assert_eq!(log.note, Some("Test note".to_string()));
    }

    #[test]
    fn test_get_stats() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut storage = Storage::new(temp_file.path());
        storage.load().unwrap();

        let habit_id = storage.data.habits[0].id;
        let start = NaiveDate::from_ymd_opt(2025, 10, 14).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 10, 20).unwrap(); // 7 days

        // Mark some days
        storage.update_log_status(habit_id, start, HabitStatus::Done).unwrap();
        storage.update_log_status(habit_id, start.succ_opt().unwrap(), HabitStatus::Done).unwrap();
        storage.update_log_status(habit_id, start.succ_opt().unwrap().succ_opt().unwrap(), HabitStatus::Skipped).unwrap();

        let stats = storage.get_stats(start, end);
        let (done, skipped, unmarked) = stats.get(&habit_id).unwrap();
        assert_eq!(*done, 2);
        assert_eq!(*skipped, 1);
        assert_eq!(*unmarked, 4);
    }
}
