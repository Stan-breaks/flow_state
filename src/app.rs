use std::{
    collections::HashSet,
    error::Error,
    fs::{create_dir_all, read_to_string, write},
};

use chrono::{Datelike, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

pub enum CurrentScreen {
    Today,
    Stats,
}
pub struct Counter {
    pub index: usize,
    pub build_counter: usize,
    pub avoid_counter: usize,
    pub switch: bool,
}

pub enum ScreenMode {
    Normal,
    Adding,
    Editing,
    Deleting,
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
pub enum HabitPattern {
    Chaotic,
    Struggling,
    Developing,
    Established,
    Mastered,
}
impl HabitPattern {
    pub fn string(&self) -> &'static str {
        match self {
            HabitPattern::Chaotic => "Chaotic 🌪️",
            HabitPattern::Struggling => "Struggling  😤",
            HabitPattern::Developing => "Developing 🌱",
            HabitPattern::Established => "Established ⚖️",
            HabitPattern::Mastered => "Mastered 🎯",
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
impl Habit {
    pub fn check_status(&self) -> HabitStatus {
        let today = Utc::now().date_naive();
        for i in self.days_completed.iter() {
            if &today == i {
                return HabitStatus::Complete;
            }
        }
        HabitStatus::InComplete
    }
    pub fn toggle_complete(&mut self) {
        let today = Utc::now();
        let day_completed = today.date_naive();
        if !self.days_completed.insert(day_completed.clone()) {
            self.days_completed.remove(&day_completed);
        }
    }
    pub fn check_pattern(&self) -> HabitPattern {
        let days_since_creation = Utc::now()
            .date_naive()
            .signed_duration_since(self.created)
            .num_days()
            .max(1);
        let check_ins = self.days_completed.len();
        let pattern = ((check_ins as f32 / days_since_creation as f32 * 5.0).round() as u32)
            .max(1)
            .min(5);
        match pattern {
            2 => HabitPattern::Struggling,
            3 => HabitPattern::Developing,
            4 => HabitPattern::Established,
            5 => HabitPattern::Mastered,
            _ => HabitPattern::Chaotic,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct HabitsData {
    habits: Vec<Habit>,
}
pub struct App {
    pub habits: Vec<Habit>,
    pub counter: Counter,
    pub current_screen: CurrentScreen,
    pub screen_mode: ScreenMode,
    pub current_habit: Habit,
}
impl App {
    pub fn new() -> Self {
        App {
            habits: Vec::default(),
            counter: Counter {
                index: 0,
                build_counter: 0,
                avoid_counter: 0,
                switch: false,
            },
            current_screen: CurrentScreen::Today,
            screen_mode: ScreenMode::Normal,
            current_habit: Habit {
                name: String::default(),
                habit_type: HabitType::Build,
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
            habits: self.habits.clone(),
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
            self.habits = habits_data.habits;
        } else {
            self.populate_dummy_data();
        }
        Ok(())
    }
    fn populate_dummy_data(&mut self) {
        self.habits = vec![
            Habit {
                name: "Morning run".to_string(),
                habit_type: HabitType::Build,
                days_completed: HashSet::new(),
                created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            },
            Habit {
                name: "Read 10 pages".to_string(),
                habit_type: HabitType::Build,
                days_completed: HashSet::new(),
                created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            },
            Habit {
                name: "Social media scrolling".to_string(),
                habit_type: HabitType::Avoid,
                days_completed: HashSet::new(),
                created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            },
            Habit {
                name: "Late-night snacking".to_string(),
                habit_type: HabitType::Avoid,
                days_completed: HashSet::new(),
                created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            },
        ];
    }
    pub fn increment_habits_counter(&mut self) {
        let build_len = self
            .habits
            .iter()
            .filter(|habit| habit.habit_type == HabitType::Build)
            .collect::<Vec<&Habit>>()
            .len();
        let avoid_len = self.habits.len() - build_len;
        if !self.counter.switch && self.counter.build_counter <= build_len {
            self.counter.build_counter += 1;
        }
        if !self.counter.switch && self.counter.build_counter == build_len + 1 {
            self.counter.switch = true;
            self.counter.build_counter = 0;
        }
        if self.counter.switch && self.counter.avoid_counter < avoid_len {
            self.counter.avoid_counter += 1;
        }
        self.counter.index += 1;
    }
    pub fn decrement_habits_counter(&mut self) {
        let build_len = self
            .habits
            .iter()
            .filter(|habit| habit.habit_type == HabitType::Build)
            .collect::<Vec<&Habit>>()
            .len();
        if self.counter.switch && self.counter.avoid_counter > 0 {
            self.counter.avoid_counter -= 1;
        }
        if self.counter.switch && self.counter.avoid_counter == 0 {
            self.counter.switch = false;
            self.counter.build_counter = build_len + 1;
        }
        if !self.counter.switch && self.counter.build_counter > 0 {
            self.counter.build_counter -= 1;
        }
        self.counter.index -= 1;
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
        let length = self.habits.len();
        if length == 0 {
            return format!("{}  ({}/{})", self.display_gauge(0.0), 0, length);
        }

        let today = Utc::now();
        let today = today.date_naive();

        let mut counter = 0;
        for habit in self.habits.iter() {
            if habit.days_completed.contains(&today) {
                counter += 1;
            }
        }
        let progress = (counter as f32 / length as f32) * 100.0;
        format!("{}  ({}/{})", self.display_gauge(progress), counter, length)
    }
    pub fn check_weeks_progress(&self) -> String {
        let total_habits = self.habits.len();
        if total_habits == 0 {
            return format!("{}  ({}/{})", self.display_gauge(0.0), 0, 0);
        }
        let today = Utc::now();
        let date = today.date_naive();
        let day_since_monday = today.weekday().num_days_from_monday();
        let week_start = date - Duration::days(day_since_monday as i64);

        let total_possible = total_habits * 7;
        let mut counter = 0;
        for i in 0..7 {
            let check_date = week_start + Duration::days(i);
            for j in self.habits.iter() {
                if j.days_completed.contains(&check_date) {
                    counter += 1
                }
            }
        }
        let progress = (counter as f32) / (total_possible as f32) * 100 as f32;

        format!(
            "{}  ({}/{})",
            self.display_gauge(progress),
            counter,
            total_possible
        )
    }
    pub fn toggle_edit_mode(&mut self, habit: Habit) {
        match self.screen_mode {
            ScreenMode::Normal => {
                self.screen_mode = ScreenMode::Editing;
                self.current_habit = habit;
            }
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
            _ => {
                self.screen_mode = ScreenMode::Normal;
                self.current_habit = Habit {
                    name: String::default(),
                    habit_type: HabitType::Build,
                    days_completed: HashSet::default(),
                    created: NaiveDate::default(),
                }
            }
        }
    }
    pub fn toggle_delete_mode(&mut self) {
        match self.screen_mode {
            ScreenMode::Deleting => {}
            _ => {
                self.screen_mode = ScreenMode::Deleting;
                if !self.counter.switch && self.counter.build_counter > 0 {
                    let build_habit = self
                        .habits
                        .iter()
                        .filter(|habit| habit.habit_type == HabitType::Build)
                        .collect::<Vec<&Habit>>()[self.counter.build_counter - 1];
                    let mut index = 0;
                    for i in 0..self.habits.len() {
                        if &self.habits[i] == build_habit {
                            index = i;
                            break;
                        }
                    }
                    self.current_habit = self.habits[index].clone();
                }
                if self.counter.switch && self.counter.avoid_counter > 0 {
                    let avoid_habit = self
                        .habits
                        .iter()
                        .filter(|habit| habit.habit_type == HabitType::Avoid)
                        .collect::<Vec<&Habit>>()[self.counter.avoid_counter - 1];
                    let mut index = 0;
                    for i in 0..self.habits.len() {
                        if &self.habits[i] == avoid_habit {
                            index = i;
                            break;
                        }
                    }
                    self.current_habit = self.habits[index].clone();
                }
            }
        }
    }
    pub fn toggle_habit_type(&mut self) {
        match self.current_habit.habit_type {
            HabitType::Build => self.current_habit.habit_type = HabitType::Avoid,
            HabitType::Avoid => self.current_habit.habit_type = HabitType::Build,
        }
    }
    pub fn edit_habit(&mut self, index: usize) {
        self.habits[index].name = self.current_habit.name.clone();
        self.habits[index].habit_type = self.current_habit.habit_type.clone();
        self.toggle_normal_mode();
    }
    pub fn add_habit(&mut self) {
        self.current_habit.created = Utc::now().date_naive();
        self.habits.push(self.current_habit.clone());
        self.toggle_normal_mode();
    }
}
