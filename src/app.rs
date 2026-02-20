use chrono::{Datelike, Duration, Utc};

use crate::habit::{Day, Habit, HabitType};
use crate::storage;

pub enum CurrentScreen {
    Today,
    Stats,
    Heatmap,
}

pub enum ScreenMode {
    Normal,
    Adding,
    Editing,
    Deleting,
    Reset,
}

pub struct Counter {
    pub build_counter: usize,
    pub avoid_counter: usize,
    pub switch: bool,
}

impl Default for Counter {
    fn default() -> Self {
        Counter {
            build_counter: 0,
            avoid_counter: 0,
            switch: false,
        }
    }
}

pub struct App {
    pub build_habits: Vec<Habit>,
    pub avoid_habits: Vec<Habit>,
    pub counter: Counter,
    pub current_screen: CurrentScreen,
    pub screen_mode: ScreenMode,
    pub current_habit: Habit,
    pub current_day: Day,
}

impl App {
    pub fn new() -> Self {
        App {
            build_habits: Vec::new(),
            avoid_habits: Vec::new(),
            counter: Counter::default(),
            current_screen: CurrentScreen::Today,
            screen_mode: ScreenMode::Normal,
            current_day: Day::Today,
            current_habit: Habit::default(),
        }
    }

    pub fn load_habits(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (build, avoid) = storage::load_habits()?;
        self.build_habits = build;
        self.avoid_habits = avoid;
        Ok(())
    }

    pub fn save_habits(&self) -> Result<(), Box<dyn std::error::Error>> {
        storage::save_habits(&self.build_habits, &self.avoid_habits)
    }

    pub fn toggle_page(&mut self) {
        self.current_screen = match self.current_screen {
            CurrentScreen::Today => CurrentScreen::Stats,
            CurrentScreen::Stats => CurrentScreen::Heatmap,
            CurrentScreen::Heatmap => CurrentScreen::Today,
        };
    }

    pub fn toggle_day(&mut self) {
        self.current_day = match self.current_day {
            Day::Today => Day::Yesterday,
            Day::Yesterday => Day::Today,
        };
    }

    pub fn toggle_habit_type(&mut self) {
        self.current_habit.habit_type = match self.current_habit.habit_type {
            HabitType::Build => HabitType::Avoid,
            HabitType::Avoid => HabitType::Build,
        };
    }

    pub fn toggle_add_mode(&mut self) {
        if let ScreenMode::Normal = self.screen_mode {
            self.screen_mode = ScreenMode::Adding;
        }
    }

    pub fn toggle_edit_mode(&mut self, habit: Habit) {
        if let ScreenMode::Normal = self.screen_mode {
            self.screen_mode = ScreenMode::Editing;
            self.current_habit = habit;
        }
    }

    pub fn toggle_delete_mode(&mut self) {
        if let ScreenMode::Normal = self.screen_mode {
            self.screen_mode = ScreenMode::Deleting;
        }
    }

    pub fn toggle_reset_mode(&mut self) {
        if let ScreenMode::Normal = self.screen_mode {
            self.screen_mode = ScreenMode::Reset;
        }
    }

    pub fn toggle_normal_mode(&mut self) {
        if !matches!(self.screen_mode, ScreenMode::Normal) {
            self.screen_mode = ScreenMode::Normal;
            self.current_habit = Habit::default();
        }
    }

    pub fn increment_habits_counter(&mut self) {
        if !self.counter.switch {
            if self.counter.build_counter + 1 < self.build_habits.len() {
                self.counter.build_counter += 1;
            } else if !self.avoid_habits.is_empty() {
                self.counter.switch = true;
            }
        } else if self.counter.avoid_counter + 1 < self.avoid_habits.len() {
            self.counter.avoid_counter += 1;
        }
    }

    pub fn decrement_habits_counter(&mut self) {
        if self.counter.switch {
            if self.counter.avoid_counter > 0 {
                self.counter.avoid_counter -= 1;
            } else if !self.build_habits.is_empty() {
                self.counter.switch = false;
            }
        } else if self.counter.build_counter > 0 {
            self.counter.build_counter -= 1;
        }
    }

    pub fn add_habit(&mut self) {
        self.current_habit.created = Utc::now().date_naive();
        match self.current_habit.habit_type {
            HabitType::Build => self.build_habits.push(self.current_habit.clone()),
            HabitType::Avoid => self.avoid_habits.push(self.current_habit.clone()),
        }
        self.toggle_normal_mode();
    }

    pub fn edit_habit(&mut self) {
        match (self.counter.switch, &self.current_habit.habit_type) {
            (false, HabitType::Build) => {
                self.build_habits[self.counter.build_counter] = self.current_habit.clone();
            }
            (false, HabitType::Avoid) => {
                self.build_habits.remove(self.counter.build_counter);
                self.avoid_habits.push(self.current_habit.clone());
            }
            (true, HabitType::Build) => {
                self.avoid_habits.remove(self.counter.avoid_counter);
                self.build_habits.push(self.current_habit.clone());
            }
            (true, HabitType::Avoid) => {
                self.avoid_habits[self.counter.avoid_counter] = self.current_habit.clone();
            }
        }
        self.toggle_normal_mode();
    }

    pub fn delete_current_habit(&mut self) {
        if !self.counter.switch {
            self.build_habits.remove(self.counter.build_counter);
            // Adjust counter to stay in bounds
            if self.counter.build_counter >= self.build_habits.len() && self.counter.build_counter > 0
            {
                self.counter.build_counter -= 1;
            }
        } else {
            self.avoid_habits.remove(self.counter.avoid_counter);
            if self.counter.avoid_counter >= self.avoid_habits.len() && self.counter.avoid_counter > 0
            {
                self.counter.avoid_counter -= 1;
            }
        }
        self.toggle_normal_mode();
    }

    pub fn reset_current_habit(&mut self) {
        if !self.counter.switch {
            self.build_habits[self.counter.build_counter].reset();
        } else {
            self.avoid_habits[self.counter.avoid_counter].reset();
        }
        self.toggle_normal_mode();
    }

    pub fn toggle_current_habit(&mut self) {
        if !self.counter.switch {
            self.build_habits[self.counter.build_counter].toggle_complete(&self.current_day);
        } else {
            self.avoid_habits[self.counter.avoid_counter].toggle_complete(&self.current_day);
        }
    }

    pub fn get_selected_habit(&self) -> Habit {
        if !self.counter.switch {
            self.build_habits[self.counter.build_counter].clone()
        } else {
            self.avoid_habits[self.counter.avoid_counter].clone()
        }
    }

    fn all_habits(&self) -> impl Iterator<Item = &Habit> {
        self.build_habits.iter().chain(self.avoid_habits.iter())
    }

    pub fn count_completed_on(&self, date: chrono::NaiveDate) -> usize {
        self.all_habits()
            .filter(|h| h.days_completed.contains(&date))
            .count()
    }

    pub fn completion_rate_for_date(&self, date: chrono::NaiveDate) -> f32 {
        let total = self.build_habits.len() + self.avoid_habits.len();
        if total == 0 {
            return 0.0;
        }
        self.count_completed_on(date) as f32 / total as f32
    }

    fn display_gauge(&self, progress: f32) -> String {
        let segments = [
            "▱▱▱▱▱▱▱▱▱▱", // 0%
            "▰▱▱▱▱▱▱▱▱▱", // 10%
            "▰▰▱▱▱▱▱▱▱▱", // 20%
            "▰▰▰▱▱▱▱▱▱▱", // 30%
            "▰▰▰▰▱▱▱▱▱▱", // 40%
            "▰▰▰▰▰▱▱▱▱▱", // 50%
            "▰▰▰▰▰▰▱▱▱▱", // 60%
            "▰▰▰▰▰▰▰▱▱▱", // 70%
            "▰▰▰▰▰▰▰▰▱▱", // 80%
            "▰▰▰▰▰▰▰▰▰▱", // 90%
            "▰▰▰▰▰▰▰▰▰▰", // 100%
        ];
        let index = ((progress / 10.0) as usize).min(10);
        format!("{} {:.1}%", segments[index], progress)
    }

    pub fn check_todays_progress(&self, day: &Day) -> String {
        let total = self.build_habits.len() + self.avoid_habits.len();
        if total == 0 {
            return format!("{}  ({}/{})", self.display_gauge(0.0), 0, total);
        }

        let date = day.resolve_date();
        let completed = self.count_completed_on(date);
        let progress = (completed as f32 / total as f32) * 100.0;
        format!(
            "{}  ({}/{})",
            self.display_gauge(progress),
            completed,
            total
        )
    }

    pub fn check_weeks_progress(&self) -> String {
        let total_habits = self.build_habits.len() + self.avoid_habits.len();
        if total_habits == 0 {
            return format!("{}  ({}/{})", self.display_gauge(0.0), 0, 0);
        }

        let today = Utc::now();
        let date = today.date_naive();
        let days_since_monday = today.weekday().num_days_from_monday();
        let week_start = date - Duration::days(days_since_monday as i64);

        let total_possible = total_habits * 7;
        let completed: usize = (0..7)
            .map(|i| {
                let check_date = week_start + Duration::days(i);
                self.count_completed_on(check_date)
            })
            .sum();

        let progress = (completed as f32) / (total_possible as f32) * 100.0;
        format!(
            "{}  ({}/{})",
            self.display_gauge(progress),
            completed,
            total_possible
        )
    }
}
