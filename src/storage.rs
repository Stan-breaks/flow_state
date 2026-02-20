use std::{
    collections::HashSet,
    error::Error,
    fs::{create_dir_all, read_to_string, write},
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::habit::{Habit, HabitType};

#[derive(Serialize, Deserialize, Clone)]
struct HabitsData {
    build_habits: Vec<Habit>,
    avoid_habits: Vec<Habit>,
}

pub fn save_habits(
    build_habits: &[Habit],
    avoid_habits: &[Habit],
) -> Result<(), Box<dyn Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("flow_state");
    create_dir_all(&config_dir)?;

    let habits_data = HabitsData {
        build_habits: build_habits.to_vec(),
        avoid_habits: avoid_habits.to_vec(),
    };
    let toml_string = toml::to_string(&habits_data)?;
    write(config_dir.join("habits.toml"), toml_string)?;
    Ok(())
}

pub fn load_habits() -> Result<(Vec<Habit>, Vec<Habit>), Box<dyn Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("flow_state");
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
        },
        Habit {
            name: "Read 10 pages".to_string(),
            habit_type: HabitType::Build,
            days_completed: HashSet::new(),
            created: NaiveDate::from_ymd_opt(2025, 06, 12).unwrap(),
        },
    ];
    let avoid_habits = vec![
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
    (build_habits, avoid_habits)
}
