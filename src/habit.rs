use std::{collections::HashSet, fmt};

use chrono::{Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Day {
    Today,
    Yesterday,
}

impl Day {
    pub fn as_str(&self) -> &'static str {
        match self {
            Day::Today => "Today",
            Day::Yesterday => "Yesterday",
        }
    }

    pub fn resolve_date(&self) -> NaiveDate {
        let today = Utc::now().date_naive();
        match self {
            Day::Today => today,
            Day::Yesterday => today - Duration::days(1),
        }
    }
}

pub enum HabitStatus {
    Complete,
    InComplete,
}

impl fmt::Display for HabitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HabitStatus::Complete => write!(f, "✅"),
            HabitStatus::InComplete => write!(f, "⚪"),
        }
    }
}

#[derive(PartialEq)]
pub enum HabitPattern {
    Chaotic,
    Struggling,
    Developing,
    Established,
    Mastered,
}

impl fmt::Display for HabitPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HabitPattern::Chaotic => write!(f, "Chaotic 🌪️"),
            HabitPattern::Struggling => write!(f, "Struggling  😤"),
            HabitPattern::Developing => write!(f, "Developing 🌱"),
            HabitPattern::Established => write!(f, "Established ⚖️"),
            HabitPattern::Mastered => write!(f, "Mastered 🎯"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum HabitType {
    Build,
    Avoid,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Habit {
    pub name: String,
    pub habit_type: HabitType,
    pub days_completed: HashSet<NaiveDate>,
    pub created: NaiveDate,
}

impl Default for Habit {
    fn default() -> Self {
        Habit {
            name: String::default(),
            habit_type: HabitType::Build,
            days_completed: HashSet::default(),
            created: NaiveDate::default(),
        }
    }
}

impl Habit {
    pub fn check_status(&self, day: &Day) -> HabitStatus {
        let date = day.resolve_date();
        if self.days_completed.contains(&date) {
            HabitStatus::Complete
        } else {
            HabitStatus::InComplete
        }
    }

    pub fn toggle_complete(&mut self, day: &Day) {
        let date = day.resolve_date();
        if !self.days_completed.insert(date) {
            self.days_completed.remove(&date);
        }
    }

    pub fn reset(&mut self) {
        self.days_completed.clear();
        self.created = Utc::now().date_naive();
    }

    fn days_since_creation(&self) -> i64 {
        Utc::now()
            .date_naive()
            .signed_duration_since(self.created)
            .num_days()
            .max(1)
    }

    pub fn check_raw_pattern(&self) -> i32 {
        let days = self.days_since_creation();
        let check_ins = self.days_completed.len();
        ((check_ins as f32 / days as f32 * 5.0).round() as i32)
            .max(0)
            .min(5)
    }

    pub fn check_pattern(&self) -> HabitPattern {
        let days = self.days_since_creation();
        let check_ins = self.days_completed.len();
        let pattern = ((check_ins as f32 / days as f32 * 5.0).round() as u32)
            .max(1)
            .min(5);
        match pattern {
            2 => HabitPattern::Struggling,
            3 => HabitPattern::Developing,
            4 => HabitPattern::Established,
            5 => {
                if days < 7 {
                    HabitPattern::Developing
                } else {
                    HabitPattern::Mastered
                }
            }
            _ => HabitPattern::Chaotic,
        }
    }
}

/// Find the habit with the highest raw pattern score from a slice.
pub fn find_best_habit(habits: &[Habit]) -> Option<&Habit> {
    habits
        .iter()
        .max_by_key(|h| h.check_raw_pattern())
}

/// Find the habit with the lowest raw pattern score from a slice.
pub fn find_worst_habit(habits: &[Habit]) -> Option<&Habit> {
    habits
        .iter()
        .min_by_key(|h| h.check_raw_pattern())
}
