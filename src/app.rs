pub enum CurrentScreen {
    Today,
    Manage,
    Stats,
}

#[derive(Default)]
pub struct Habit {
    pub name: String,
    pub frequency: String,
    pub created: String,
}
pub struct App {
    pub build_habit: Vec<Habit>,
    pub avoid_habit: Vec<Habit>,
    pub current_screen: CurrentScreen,
}
impl App {
    pub fn new() -> Self {
        App {
            build_habit: Vec::default(),
            avoid_habit: Vec::default(),
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
    pub fn populate_dummy_data(&mut self) {
        self.build_habit = vec![
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

        self.avoid_habit = vec![
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
