use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};

/// Helper struct for working with weeks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Week {
    /// The Monday of this week
    pub start: NaiveDate,
}

impl Week {
    /// Create a week containing the given date
    pub fn containing(date: NaiveDate) -> Self {
        let monday = Self::get_monday(date);
        Self { start: monday }
    }

    /// Create a week for the current date
    pub fn current() -> Self {
        Self::containing(Local::now().date_naive())
    }

    /// Get the Monday of the week containing the given date
    fn get_monday(date: NaiveDate) -> NaiveDate {
        let weekday = date.weekday();
        let days_since_monday = weekday.num_days_from_monday();
        date - Duration::days(days_since_monday as i64)
    }

    /// Get all 7 days of this week (Monday through Sunday)
    pub fn days(&self) -> [NaiveDate; 7] {
        [
            self.start,
            self.start + Duration::days(1),
            self.start + Duration::days(2),
            self.start + Duration::days(3),
            self.start + Duration::days(4),
            self.start + Duration::days(5),
            self.start + Duration::days(6),
        ]
    }

    /// Get the day at a specific index (0 = Monday, 6 = Sunday)
    pub fn day(&self, index: usize) -> Option<NaiveDate> {
        if index < 7 {
            Some(self.start + Duration::days(index as i64))
        } else {
            None
        }
    }

    /// Get the Sunday of this week
    pub fn end(&self) -> NaiveDate {
        self.start + Duration::days(6)
    }

    /// Move to the next week
    pub fn next(&self) -> Self {
        Self {
            start: self.start + Duration::days(7),
        }
    }

    /// Move to the previous week
    pub fn prev(&self) -> Self {
        Self {
            start: self.start - Duration::days(7),
        }
    }

    /// Format the week as "Oct 7-13, 2025"
    pub fn format(&self) -> String {
        let end = self.end();
        if self.start.month() == end.month() {
            format!(
                "{} {}-{}, {}",
                self.start.format("%b"),
                self.start.day(),
                end.day(),
                self.start.year()
            )
        } else {
            format!(
                "{} {}-{} {}, {}",
                self.start.format("%b"),
                self.start.day(),
                end.format("%b"),
                end.day(),
                self.start.year()
            )
        }
    }

    /// Get the weekday name for a day index (0 = Mon, 6 = Sun)
    pub fn weekday_name(index: usize) -> &'static str {
        match index {
            0 => "Mon",
            1 => "Tue",
            2 => "Wed",
            3 => "Thu",
            4 => "Fri",
            5 => "Sat",
            6 => "Sun",
            _ => "???",
        }
    }

    /// Get the full weekday name for a date
    pub fn full_weekday_name(date: NaiveDate) -> &'static str {
        match date.weekday() {
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday",
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
            Weekday::Sun => "Sunday",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_monday() {
        // Test various days in a week
        let wed = NaiveDate::from_ymd_opt(2025, 10, 15).unwrap(); // Wednesday
        let monday = Week::get_monday(wed);
        assert_eq!(monday, NaiveDate::from_ymd_opt(2025, 10, 13).unwrap());

        let sun = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap(); // Sunday
        let monday = Week::get_monday(sun);
        assert_eq!(monday, NaiveDate::from_ymd_opt(2025, 10, 13).unwrap());

        let mon = NaiveDate::from_ymd_opt(2025, 10, 13).unwrap(); // Monday
        let monday = Week::get_monday(mon);
        assert_eq!(monday, NaiveDate::from_ymd_opt(2025, 10, 13).unwrap());
    }

    #[test]
    fn test_week_containing() {
        let date = NaiveDate::from_ymd_opt(2025, 10, 15).unwrap(); // Wednesday
        let week = Week::containing(date);
        assert_eq!(week.start, NaiveDate::from_ymd_opt(2025, 10, 13).unwrap());
    }

    #[test]
    fn test_week_days() {
        let week = Week {
            start: NaiveDate::from_ymd_opt(2025, 10, 13).unwrap(),
        };
        let days = week.days();
        assert_eq!(days.len(), 7);
        assert_eq!(days[0], NaiveDate::from_ymd_opt(2025, 10, 13).unwrap());
        assert_eq!(days[6], NaiveDate::from_ymd_opt(2025, 10, 19).unwrap());
    }

    #[test]
    fn test_week_day() {
        let week = Week {
            start: NaiveDate::from_ymd_opt(2025, 10, 13).unwrap(),
        };
        assert_eq!(
            week.day(0),
            Some(NaiveDate::from_ymd_opt(2025, 10, 13).unwrap())
        );
        assert_eq!(
            week.day(6),
            Some(NaiveDate::from_ymd_opt(2025, 10, 19).unwrap())
        );
        assert_eq!(week.day(7), None);
    }

    #[test]
    fn test_week_navigation() {
        let week = Week {
            start: NaiveDate::from_ymd_opt(2025, 10, 13).unwrap(),
        };
        let next = week.next();
        assert_eq!(next.start, NaiveDate::from_ymd_opt(2025, 10, 20).unwrap());

        let prev = week.prev();
        assert_eq!(prev.start, NaiveDate::from_ymd_opt(2025, 10, 6).unwrap());
    }

    #[test]
    fn test_week_format() {
        let week = Week {
            start: NaiveDate::from_ymd_opt(2025, 10, 13).unwrap(),
        };
        let formatted = week.format();
        assert_eq!(formatted, "Oct 13-19, 2025");
    }

    #[test]
    fn test_week_format_cross_month() {
        let week = Week {
            start: NaiveDate::from_ymd_opt(2025, 9, 29).unwrap(),
        };
        let formatted = week.format();
        assert_eq!(formatted, "Sep 29-Oct 5, 2025");
    }

    #[test]
    fn test_weekday_names() {
        assert_eq!(Week::weekday_name(0), "Mon");
        assert_eq!(Week::weekday_name(3), "Thu");
        assert_eq!(Week::weekday_name(6), "Sun");
    }

    #[test]
    fn test_full_weekday_names() {
        let mon = NaiveDate::from_ymd_opt(2025, 10, 13).unwrap();
        assert_eq!(Week::full_weekday_name(mon), "Monday");

        let thu = NaiveDate::from_ymd_opt(2025, 10, 16).unwrap();
        assert_eq!(Week::full_weekday_name(thu), "Thursday");
    }
}
