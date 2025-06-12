use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}
pub enum CurrentlyEditing {
    Key,
    Value,
    None,
}
pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: CurrentlyEditing,
}
impl App {
    pub fn new() -> Self {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: CurrentlyEditing::None,
        }
    }
    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone())
            .unwrap();
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = CurrentlyEditing::None;
    }
    pub fn toggle_editing(&mut self) {
        match self.currently_editing {
            CurrentlyEditing::Key => self.currently_editing = CurrentlyEditing::Value,
            CurrentlyEditing::Value => self.currently_editing = CurrentlyEditing::Key,
            CurrentlyEditing::None => self.currently_editing = CurrentlyEditing::Key,
        };
    }
    pub fn print_json(&mut self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
