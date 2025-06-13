use std::{
    error::Error,
    fs::{create_dir_all, read_to_string, write},
};

use serde::{Deserialize, Serialize};

pub enum CurrentScreen {
    Today,
    Manage,
    Stats,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Habit {
    pub name: String,
    pub frequency: String,
    pub created: String,
}
#[derive(Serialize, Deserialize, Clone)]
struct HabitsData {
    build_habits: Vec<Habit>,
    avoid_habits: Vec<Habit>,
}
pub struct App {
    pub build_habits: Vec<Habit>,
    pub avoid_habits: Vec<Habit>,
    pub current_screen: CurrentScreen,
}
impl App {
    pub fn new() -> Self {
        App {
            build_habits: Vec::default(),
            avoid_habits: Vec::default(),
            current_screen: CurrentScreen::Today,
        }
    }
    pub fn toggle_page(&mut self) {
        match &self.current_screen {
            CurrentScreen::Today => self.current_screen = CurrentScreen::Manage,
            CurrentScreen::Manage => self.current_screen = CurrentScreen::Stats,
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
                frequency: "Daily".to_string(),
                created: "2025-06-13".to_string(),
            },
            Habit {
                name: "Read 10 pages".to_string(),
                frequency: "Daily".to_string(),
                created: "2025-06-12".to_string(),
            },
        ];

        self.avoid_habits = vec![
            Habit {
                name: "Social media scrolling".to_string(),
                frequency: "Daily".to_string(),
                created: "2025-06-10".to_string(),
            },
            Habit {
                name: "Late-night snacking".to_string(),
                frequency: "Weekly".to_string(),
                created: "2025-06-09".to_string(),
            },
        ];
    }
}
