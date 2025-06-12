pub enum CurrentScreen {
    Today,
    Manage,
    Stats,
}

#[derive(Default)]
pub struct Habit {
    name: String,
    habit_type: String,
    frequency: String,
    created: String,
}
pub struct App {
    pub build_habit: Habit,
    pub avoid_habit: Habit,
    pub current_screen: CurrentScreen,
}
impl App {
    pub fn new() -> Self {
        App {
            build_habit: Habit::default(),
            avoid_habit: Habit::default(),
            current_screen: CurrentScreen::Today,
        }
    }
    pub fn toggle_page(&mut self) {
        match &self.current_screen {
            CurrentScreen::Today => self.current_screen = CurrentScreen::Manage,
            CurrentScreen::Manage => self.current_screen = CurrentScreen::Stats,
            CurrentScreen::Stats => self.current_screen = CurrentScreen::Today,
            _ => {}
        };
    }
}
