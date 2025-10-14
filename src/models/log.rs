use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a habit for a given day
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HabitStatus {
    /// Habit was completed
    Done,
    /// Habit was intentionally skipped
    Skipped,
    /// No status recorded (default)
    Unmarked,
}

impl Default for HabitStatus {
    fn default() -> Self {
        Self::Unmarked
    }
}

impl HabitStatus {
    /// Cycle through statuses: Done -> Skipped -> Unmarked -> Done
    pub fn cycle(&self) -> Self {
        match self {
            Self::Done => Self::Skipped,
            Self::Skipped => Self::Unmarked,
            Self::Unmarked => Self::Done,
        }
    }

    /// Get display string for the status
    pub fn display_str(&self) -> &'static str {
        match self {
            Self::Done => "[Done]",
            Self::Skipped => "[Skipped]",
            Self::Unmarked => "[ ]",
        }
    }
}

/// Log entry for a habit on a specific day
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HabitLog {
    /// ID of the habit this log entry refers to
    pub habit_id: Uuid,
    /// Date for this log entry
    pub date: NaiveDate,
    /// Status of the habit on this day
    pub status: HabitStatus,
    /// Optional note about this habit on this day
    pub note: Option<String>,
}

impl HabitLog {
    /// Create a new habit log entry
    pub fn new(habit_id: Uuid, date: NaiveDate) -> Self {
        Self {
            habit_id,
            date,
            status: HabitStatus::default(),
            note: None,
        }
    }

    /// Create a new habit log entry with a specific status
    pub fn with_status(habit_id: Uuid, date: NaiveDate, status: HabitStatus) -> Self {
        Self {
            habit_id,
            date,
            status,
            note: None,
        }
    }

    /// Add or update the note for this log entry
    pub fn set_note(&mut self, note: Option<String>) {
        self.note = note;
    }

    /// Toggle the status to the next value
    pub fn toggle_status(&mut self) {
        self.status = self.status.cycle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_habit_status_cycle() {
        assert_eq!(HabitStatus::Done.cycle(), HabitStatus::Skipped);
        assert_eq!(HabitStatus::Skipped.cycle(), HabitStatus::Unmarked);
        assert_eq!(HabitStatus::Unmarked.cycle(), HabitStatus::Done);
    }

    #[test]
    fn test_habit_status_display() {
        assert_eq!(HabitStatus::Done.display_str(), "[Done]");
        assert_eq!(HabitStatus::Skipped.display_str(), "[Skipped]");
        assert_eq!(HabitStatus::Unmarked.display_str(), "[ ]");
    }

    #[test]
    fn test_habit_log_new() {
        let habit_id = Uuid::new_v4();
        let date = NaiveDate::from_ymd_opt(2025, 10, 14).unwrap();
        let log = HabitLog::new(habit_id, date);

        assert_eq!(log.habit_id, habit_id);
        assert_eq!(log.date, date);
        assert_eq!(log.status, HabitStatus::Unmarked);
        assert_eq!(log.note, None);
    }

    #[test]
    fn test_habit_log_toggle() {
        let habit_id = Uuid::new_v4();
        let date = NaiveDate::from_ymd_opt(2025, 10, 14).unwrap();
        let mut log = HabitLog::new(habit_id, date);

        log.toggle_status();
        assert_eq!(log.status, HabitStatus::Done);

        log.toggle_status();
        assert_eq!(log.status, HabitStatus::Skipped);

        log.toggle_status();
        assert_eq!(log.status, HabitStatus::Unmarked);
    }

    #[test]
    fn test_habit_log_note() {
        let habit_id = Uuid::new_v4();
        let date = NaiveDate::from_ymd_opt(2025, 10, 14).unwrap();
        let mut log = HabitLog::new(habit_id, date);

        log.set_note(Some("Had a tough day".to_string()));
        assert_eq!(log.note, Some("Had a tough day".to_string()));

        log.set_note(None);
        assert_eq!(log.note, None);
    }
}
