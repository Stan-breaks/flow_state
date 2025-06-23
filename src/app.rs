use std::{
    collections::HashSet,
    error::Error,
    fs::{create_dir_all, read_to_string, write},
};

use chrono::{Datelike, Duration, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};

pub enum CurrentScreen {
    Today,
    Stats,
}

pub enum ScreenMode {
    Normal,
    Adding,
    Editing,
}

pub enum HabitStatus {
    Complete,
    InComplete,
}
impl HabitStatus {
    pub fn emoji(&self) -> &'static str {
        match self {
            HabitStatus::Complete => "✅",
            HabitStatus::InComplete => "⚪",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Completed {
    date: NaiveDate,
    day: Weekday,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Habit {
    pub name: String,
    pub days_completed: HashSet<Completed>,
    pub created: NaiveDate,
}
impl Habit {
    pub fn check_status(&self) -> HabitStatus {
        let today = Utc::now().date_naive();
        for i in self.days_completed.iter() {
            if today == i.date {
                return HabitStatus::Complete;
            }
        }
        HabitStatus::InComplete
    }
    pub fn toggle_complete(&mut self) {
        let today = Utc::now();
        let day_completed = Completed {
            date: today.date_naive(),
            day: today.weekday(),
        };
        if !self.days_completed.insert(day_completed.clone()) {
            self.days_completed.remove(&day_completed);
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct HabitsData {
    build_habits: Vec<Habit>,
    avoid_habits: Vec<Habit>,
}
pub struct App {
    pub build_habits: Vec<Habit>,
    pub avoid_habits: Vec<Habit>,
    pub habits_counter: usize,
    pub current_screen: CurrentScreen,
    pub screen_mode: ScreenMode,
    pub current_habit: Habit,
}
impl App {
    pub fn new() -> Self {
        App {
            build_habits: Vec::default(),
            avoid_habits: Vec::default(),
            habits_counter: 0,
            current_screen: CurrentScreen::Today,
            screen_mode: ScreenMode::Normal,
            current_habit: Habit {
                name: String::default(),
                days_completed: HashSet::default(),
                created: NaiveDate::default(),
            },
        }
    }
    pub fn toggle_page(&mut self) {
        match &self.current_screen {
            CurrentScreen::Today => self.current_screen = CurrentScreen::Stats,
            CurrentScreen::Stats => self.current_screen = CurrentScreen::Today,
        };
    }
    pub fn save_habits(&mut self) -> color_eyre::Result<(), Box<dyn Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("flow_state");
        let habits_data = HabitsData {
            build_habits: self.build_habits.clone(),
            avoid_habits: self.avoid_habits.clone(),
        };
        let toml_string = toml::to_string(&habits_data)?;
        write(config_dir.join("habits.toml"), toml_string)?;
        Ok(())
    }

    pub fn load_habits(&mut self) -> color_eyre::Result<(), Box<dyn Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("flow_state");
        create_dir_all(&config_dir)?;

        let habits_file = config_dir.join("habits.toml");

        if habits_file.exists() {
            let content = read_to_string(habits_file)?;
            let habits_data: HabitsData = toml::from_str(&content)?;
            self.build_habits = habits_data.build_habits;
            self.avoid_habits = habits_data.avoid_habits;
        } else {
            self.populate_dummy_data();
        }
        Ok(())
    }
    fn populate_dummy_data(&mut self) {
        self.build_habits = vec![
            Habit {
                name: "Morning run".to_string(),
                days_completed: HashSet::new(),
                created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            },
            Habit {
                name: "Read 10 pages".to_string(),
                days_completed: HashSet::new(),
                created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            },
        ];

        self.avoid_habits = vec![
            Habit {
                name: "Social media scrolling".to_string(),
                days_completed: HashSet::new(),
                created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            },
            Habit {
                name: "Late-night snacking".to_string(),
                days_completed: HashSet::new(),
                created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            },
        ];
    }
    pub fn increment_habits_counter(&mut self) {
        if self.habits_counter < self.avoid_habits.len() + self.build_habits.len() {
            self.habits_counter += 1;
        }
    }
    pub fn decrement_habits_counter(&mut self) {
        if self.habits_counter > 0 {
            self.habits_counter -= 1;
        }
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
        format!("{} {:.1}%", segments[index], progress,)
    }
    pub fn check_todays_progress(&self) -> String {
        let length = self.build_habits.len() + self.avoid_habits.len();
        if length == 0 {
            return format!("{}  ({}/{})", self.display_gauge(0.0), 0, length);
        }

        let today = Utc::now();
        let today = Completed {
            date: today.date_naive(),
            day: today.weekday(),
        };

        let mut counter = 0;
        for habit in self.build_habits.iter() {
            if habit.days_completed.contains(&today) {
                counter += 1;
            }
        }
        for habit in self.avoid_habits.iter() {
            if habit.days_completed.contains(&today) {
                counter += 1;
            }
        }

        let progress = (counter as f32 / length as f32) * 100.0;
        format!("{}  ({}/{})", self.display_gauge(progress), counter, length)
    }
    pub fn check_weeks_progress(&self) -> String {
        let total_habits = self.build_habits.len() + self.avoid_habits.len();
        if total_habits == 0 {
            return format!("{}  ({}/{})", self.display_gauge(0.0), 0, 0);
        }
        let today = Utc::now();
        let today = Completed {
            date: today.date_naive(),
            day: today.weekday(),
        };
        let day_since_monday = today.day.num_days_from_monday();
        let week_start = today.date - Duration::days(day_since_monday as i64);

        let total_possible = total_habits * 7;
        let mut counter = 0;
        for i in 0..7 {
            let check_date = week_start + Duration::days(i);
            let check_datetime = check_date.and_hms_opt(0, 0, 0).unwrap();
            let check_day = Completed {
                date: check_date,
                day: check_datetime.weekday(),
            };
            for j in self.build_habits.iter() {
                if j.days_completed.contains(&check_day) {
                    counter += 1
                }
            }
            for j in self.avoid_habits.iter() {
                if j.days_completed.contains(&check_day) {
                    counter += 1
                }
            }
        }
        let progress = (counter as f32) / (total_possible as f32);

        format!(
            "{}  ({}/{})",
            self.display_gauge(progress),
            counter,
            total_possible
        )
    }
    pub fn toggle_edit_mode(&mut self) {
        match self.screen_mode {
            ScreenMode::Normal => self.screen_mode = ScreenMode::Editing,
            _ => {}
        }
    }
    pub fn toggle_add_mode(&mut self) {
        match self.screen_mode {
            ScreenMode::Normal => self.screen_mode = ScreenMode::Adding,
            _ => {}
        }
    }
    pub fn toggle_normal_mode(&mut self) {
        match self.screen_mode {
            ScreenMode::Normal => {}
            _ => self.screen_mode = ScreenMode::Normal,
        }
    }
    pub fn add_build_habit(&mut self) {
        self.build_habits.push(self.current_habit.clone());
    }
    pub fn add_avoid_habit(&mut self) {
        self.avoid_habits.push(self.current_habit.clone());
    }
}
