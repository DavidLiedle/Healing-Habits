use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a habit that can be tracked
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Habit {
    /// Unique identifier for the habit
    pub id: Uuid,
    /// Display name of the habit
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Display order (lower numbers appear first)
    pub order: usize,
}

impl Habit {
    /// Create a new habit with a generated UUID
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: None,
            order: 0,
        }
    }

    /// Create a new habit with a specific ID (useful for testing)
    pub fn with_id(id: Uuid, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            description: None,
            order: 0,
        }
    }

    /// Create a new habit with a description
    pub fn with_description(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: Some(description.into()),
            order: 0,
        }
    }

    /// Set the description for this habit
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    /// Set the display order
    pub fn set_order(&mut self, order: usize) {
        self.order = order;
    }
}

/// Default habits for new users
pub fn default_habits() -> Vec<Habit> {
    vec![
        {
            let mut h = Habit::new("Shower");
            h.set_order(0);
            h
        },
        {
            let mut h = Habit::new("Brush teeth");
            h.set_order(1);
            h
        },
        {
            let mut h = Habit::new("Trim nails");
            h.set_description(Some("As needed".to_string()));
            h.set_order(2);
            h
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_habit_new() {
        let habit = Habit::new("Test Habit");
        assert_eq!(habit.name, "Test Habit");
        assert_eq!(habit.description, None);
        assert_eq!(habit.order, 0);
    }

    #[test]
    fn test_habit_with_description() {
        let habit = Habit::with_description("Test Habit", "A test description");
        assert_eq!(habit.name, "Test Habit");
        assert_eq!(habit.description, Some("A test description".to_string()));
    }

    #[test]
    fn test_habit_set_order() {
        let mut habit = Habit::new("Test");
        habit.set_order(5);
        assert_eq!(habit.order, 5);
    }

    #[test]
    fn test_default_habits() {
        let habits = default_habits();
        assert_eq!(habits.len(), 3);
        assert_eq!(habits[0].name, "Shower");
        assert_eq!(habits[1].name, "Brush teeth");
        assert_eq!(habits[2].name, "Trim nails");
        assert_eq!(habits[2].description, Some("As needed".to_string()));
    }

    #[test]
    fn test_habit_ordering() {
        let habits = default_habits();
        assert_eq!(habits[0].order, 0);
        assert_eq!(habits[1].order, 1);
        assert_eq!(habits[2].order, 2);
    }
}
