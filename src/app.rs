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
    Reset,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Day {
    Today,
    Yesterday,
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
#[derive(PartialEq)]
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
    pub fn check_status(&self, day: Day) -> HabitStatus {
        let mut today = Utc::now().date_naive();
        if day == Day::Yesterday {
            today = today - Duration::days(1);
        }
        for i in self.days_completed.iter() {
            if &today == i {
                return HabitStatus::Complete;
            }
        }
        HabitStatus::InComplete
    }
    pub fn toggle_complete(&mut self, day: Day) {
        let today = Utc::now();
        let mut day_completed = today.date_naive();
        if day == Day::Yesterday {
            day_completed = day_completed - Duration::days(1);
        }

        if !self.days_completed.insert(day_completed.clone()) {
            self.days_completed.remove(&day_completed);
        }
    }
    pub fn reset(&mut self) {
        let today = Utc::now().date_naive();
        self.days_completed.clear();
        self.created = today;
    }
    pub fn check_raw_pattern(&self) -> i32 {
        let days_since_creation = Utc::now()
            .date_naive()
            .signed_duration_since(self.created)
            .num_days()
            .max(1);
        let check_ins = self.days_completed.len();
        ((check_ins as f32 / days_since_creation as f32 * 5.0).round() as i32)
            .max(0)
            .min(5)
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
            5 => {
                if days_since_creation < 7 {
                    HabitPattern::Developing
                } else {
                    HabitPattern::Mastered
                }
            }
            _ => HabitPattern::Chaotic,
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
    pub counter: Counter,
    pub current_screen: CurrentScreen,
    pub screen_mode: ScreenMode,
    pub current_habit: Habit,
    pub current_day: Day,
}
impl App {
    pub fn new() -> Self {
        App {
            build_habits: Vec::default(),
            avoid_habits: Vec::default(),
            counter: Counter {
                index: 0,
                build_counter: 0,
                avoid_counter: 0,
                switch: false,
            },
            current_screen: CurrentScreen::Today,
            screen_mode: ScreenMode::Normal,
            current_day: Day::Today,
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
        ];
        self.avoid_habits = vec![
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
        let build_len = self.build_habits.len();
        let avoid_len = self.avoid_habits.len();
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
        let build_len = self.build_habits.len();
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
    pub fn check_todays_progress(&self, day: Day) -> String {
        let length = self.build_habits.len() + self.avoid_habits.len();
        if length == 0 {
            return format!("{}  ({}/{})", self.display_gauge(0.0), 0, length);
        }

        let mut today = Utc::now().date_naive();
        if day == Day::Yesterday {
            today = today - Duration::days(1);
        }

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
        let date = today.date_naive();
        let day_since_monday = today.weekday().num_days_from_monday();
        let week_start = date - Duration::days(day_since_monday as i64);

        let total_possible = total_habits * 7;
        let mut counter = 0;
        for i in 0..7 {
            let check_date = week_start + Duration::days(i);
            for j in self.build_habits.iter() {
                if j.days_completed.contains(&check_date) {
                    counter += 1
                }
            }
            for j in self.avoid_habits.iter() {
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
            _ => self.screen_mode = ScreenMode::Deleting,
        }
    }
    pub fn toggle_reset_mode(&mut self) {
        match self.screen_mode {
            ScreenMode::Reset => {}
            _ => self.screen_mode = ScreenMode::Reset,
        }
    }
    pub fn toggle_day(&mut self) {
        match self.current_day {
            Day::Today => self.current_day = Day::Yesterday,
            Day::Yesterday => self.current_day = Day::Today,
        }
    }
    pub fn toggle_habit_type(&mut self) {
        match self.current_habit.habit_type {
            HabitType::Build => self.current_habit.habit_type = HabitType::Avoid,
            HabitType::Avoid => self.current_habit.habit_type = HabitType::Build,
        }
    }
    pub fn edit_build_habit(&mut self, index: usize) {
        self.build_habits[index].name = self.current_habit.name.clone();
        self.build_habits[index].habit_type = self.current_habit.habit_type.clone();
        self.toggle_normal_mode();
    }
    pub fn edit_avoid_habit(&mut self, index: usize) {
        self.avoid_habits[index].name = self.current_habit.name.clone();
        self.avoid_habits[index].habit_type = self.current_habit.habit_type.clone();
        self.toggle_normal_mode();
    }
    pub fn add_habit(&mut self) {
        self.current_habit.created = Utc::now().date_naive();
        match self.current_habit.habit_type {
            HabitType::Build => self.build_habits.push(self.current_habit.clone()),
            HabitType::Avoid => self.avoid_habits.push(self.current_habit.clone()),
        }
        self.toggle_normal_mode();
    }
    pub fn find_best_build_habit(&self) -> Habit {
        let mut best_score = self.build_habits[0].check_raw_pattern();
        let mut best_index = 0;
        for i in 1..self.build_habits.len() {
            let score = self.build_habits[i].check_raw_pattern();
            if best_score < score {
                best_index = i;
                best_score = score;
            }
        }
        self.build_habits[best_index].clone()
    }
    pub fn find_best_avoid_habit(&self) -> Habit {
        let mut best_score = self.avoid_habits[0].check_raw_pattern();
        let mut best_index = 0;
        for i in 1..self.avoid_habits.len() {
            let score = self.avoid_habits[i].check_raw_pattern();
            if best_score < score {
                best_index = i;
                best_score = score;
            }
        }
        self.avoid_habits[best_index].clone()
    }
    pub fn find_worst_build_habit(&self) -> Habit {
        let mut worst_score = self.build_habits[0].check_raw_pattern();
        let mut worst_index = 0;
        for i in 1..self.build_habits.len() {
            let score = self.build_habits[i].check_raw_pattern();
            if worst_score > score {
                worst_score = score;
                worst_index = i
            }
        }
        self.build_habits[worst_index].clone()
    }
    pub fn find_worst_avoid_habit(&self) -> Habit {
        let mut worst_score = self.avoid_habits[0].check_raw_pattern();
        let mut worst_index = 0;
        for i in 1..self.avoid_habits.len() {
            let score = self.avoid_habits[i].check_raw_pattern();
            if worst_score > score {
                worst_score = score;
                worst_index = i
            }
        }
        self.avoid_habits[worst_index].clone()
    }
}
