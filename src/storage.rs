use std::{
    collections::HashSet,
    fs::{create_dir_all, read_to_string, write},
    io,
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    app::AppError,
    habit::{Habit, HabitType},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct NotificationSettings {
    pub enable: bool,
    pub hour: usize,
    pub minute: usize,
    pub low_threshold: usize,
    pub high_threshold: usize,
    /// Hours past midnight that still count as "yesterday" (e.g. 2 lets a
    /// night owl's day run until 2am). 0 = day resets at local midnight.
    #[serde(default)]
    pub day_cutoff_hour: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct HabitsData {
    build_habits: Vec<Habit>,
    avoid_habits: Vec<Habit>,
}

pub fn save_habits(build_habits: &[Habit], avoid_habits: &[Habit]) -> Result<(), AppError> {
    let config_dir = match dirs::config_dir() {
        Some(path) => Ok(path.join("flow_state")),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "config directory not found",
        )),
    }?;

    create_dir_all(&config_dir)?;

    let habits_data = HabitsData {
        build_habits: build_habits.to_vec(),
        avoid_habits: avoid_habits.to_vec(),
    };
    let toml_string = toml::to_string(&habits_data)?;
    write(config_dir.join("habits.toml"), toml_string)?;
    Ok(())
}

pub fn load_habits() -> Result<(Vec<Habit>, Vec<Habit>), AppError> {
    let config_dir = match dirs::config_dir(){
        Some(path)=>Ok(path.join("flow_state")) ,
        None => Err(io::Error::new(io::ErrorKind::NotFound, "config directory not found"))
    }?;
    create_dir_all(&config_dir)?;

    let habits_file = config_dir.join("habits.toml");

    if habits_file.exists() {
        let content = read_to_string(habits_file)?;
        let habits_data: HabitsData = toml::from_str(&content)?;
        Ok((habits_data.build_habits, habits_data.avoid_habits))
    } else {
        Ok(populate_dummy_data())
    }
}

fn populate_dummy_data() -> (Vec<Habit>, Vec<Habit>) {
    let build_habits = vec![
        Habit {
            name: "Morning run".to_string(),
            habit_type: HabitType::Build,
            days_completed: HashSet::new(),
            created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            holidays: Vec::new(),
        },
        Habit {
            name: "Read 10 pages".to_string(),
            habit_type: HabitType::Build,
            days_completed: HashSet::new(),
            created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            holidays: Vec::new(),
        },
    ];
    let avoid_habits = vec![
        Habit {
            name: "Social media scrolling".to_string(),
            habit_type: HabitType::Avoid,
            days_completed: HashSet::new(),
            created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            holidays: Vec::new(),
        },
        Habit {
            name: "Late-night snacking".to_string(),
            habit_type: HabitType::Avoid,
            days_completed: HashSet::new(),
            created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
            holidays: Vec::new(),
        },
    ];
    (build_habits, avoid_habits)
}

pub fn load_notification_settings() -> Result<NotificationSettings, AppError> {
    let config_dir = match dirs::config_dir(){
        Some(path)=>Ok(path.join("flow_state")) ,
        None => Err(io::Error::new(io::ErrorKind::NotFound, "config directory not found"))
    }?;

    let notification_file = config_dir.join("notification.toml");

    if notification_file.exists() {
        let content = read_to_string(notification_file)?;
        let notification_data: NotificationSettings = toml::from_str(&content)?;
        Ok(notification_data)
    } else {
        Ok(default_notification_settings())
    }
}

fn default_notification_settings() -> NotificationSettings {
    NotificationSettings {
        enable: false,
        hour: 0,
        minute: 0,
        low_threshold: 20,
        high_threshold: 80,
        day_cutoff_hour: 0,
    }
}
