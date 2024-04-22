use std::collections::HashMap;

use ratatui::widgets::ListState;
use serde_json::Result;

// MAIN SCREEN

pub enum MainMenuItemStatus {
    Selected,
    Unselected,
}

pub enum MainMenuItemCode {
    SyncDB,
    Credentials,
    Exit,
}

pub struct MainMenuItem {
    pub text: String,
    pub code: MainMenuItemCode,
    pub status: MainMenuItemStatus,
}

pub struct MainMenu {
    pub items: Vec<MainMenuItem>,
    pub list_state: ListState,
    last_selected: Option<MainMenuItem>,
}

pub struct MainScreen {
    pub menu: MainMenu,
}

// APP

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,              // the currently being edited json key.
    pub value_input: String,            // the currently being edited json value.
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub main_screen: MainScreen,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            main_screen: MainScreen::new(),
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}

impl MainScreen {
    pub fn new() -> MainScreen {
        MainScreen {
            menu: MainMenu::new(),
        }
    }
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        MainMenu {
            list_state,
            items: vec![
                MainMenuItem::new(
                    "✨ Synchronise Databases",
                    MainMenuItemCode::SyncDB,
                    MainMenuItemStatus::Selected,
                ),
                MainMenuItem::new(
                    "Credentials",
                    MainMenuItemCode::Credentials,
                    MainMenuItemStatus::Unselected,
                ),
                MainMenuItem::new(
                    "Exit",
                    MainMenuItemCode::Exit,
                    MainMenuItemStatus::Unselected,
                ),
            ],
            last_selected: None,
        }
    }

    pub fn next(&mut self) {
        let current_selection = self.list_state.selected().unwrap();
        let next_selection = (current_selection + 1) % self.items.len();
        self.list_state.select(Some(next_selection));
        self.toggle_select_status(&current_selection, &next_selection);
        self.last_selected = Some(self.items[next_selection].clone());
    }

    pub fn previous(&mut self) {
        let current_selection = self.list_state.selected().unwrap();
        if let Some(selected) = self.list_state.selected() {
            let prev = selected.saturating_sub(1);
            self.list_state.select(Some(prev));
        }
        let previous_selection = if current_selection == 0 {
            self.items.len() - 1
        } else {
            current_selection - 1
        };
        self.toggle_select_status(&current_selection, &previous_selection);
        self.list_state.select(Some(previous_selection));
    }

    pub fn selected(&self) -> &MainMenuItem {
        let current_selection = self.list_state.selected().unwrap();
        &self.items[current_selection]
    }

    fn toggle_select_status(&mut self, &current_selection: &usize, &next_selection: &usize) {
        self.items[current_selection].status = MainMenuItemStatus::Unselected;
        self.items[next_selection].status = MainMenuItemStatus::Selected;
        self.items[current_selection].text =  self.add_delete_icon_to_item(&self.items[current_selection]);
        self.items[next_selection].text = self.add_delete_icon_to_item(&self.items[next_selection]);
    }

    fn add_delete_icon_to_item(&self, item: &MainMenuItem) -> String {
        match item.status {
            MainMenuItemStatus::Selected => format!("✨ {}", item.text),
            MainMenuItemStatus::Unselected => {
                format!("{}", item.text.trim_start_matches("✨ ").to_string())
            }
        }
    }
}

impl MainMenuItem {
    pub fn new(text: &str, code: MainMenuItemCode, status: MainMenuItemStatus) -> MainMenuItem {
        MainMenuItem {
            text: text.to_string(),
            code,
            status,
        }
    }

    pub fn clone(&self) -> MainMenuItem {
        MainMenuItem {
            text: self.text.clone(),
            code: self.code.clone(),
            status: self.status.clone(),
        }
    }
}

impl MainMenuItemCode {
    pub fn clone(&self) -> MainMenuItemCode {
        match self {
            MainMenuItemCode::SyncDB => MainMenuItemCode::SyncDB,
            MainMenuItemCode::Credentials => MainMenuItemCode::Credentials,
            MainMenuItemCode::Exit => MainMenuItemCode::Exit,
        }
    }
}

impl MainMenuItemStatus {
    pub fn clone(&self) -> MainMenuItemStatus {
        match self {
            MainMenuItemStatus::Selected => MainMenuItemStatus::Selected,
            MainMenuItemStatus::Unselected => MainMenuItemStatus::Unselected,
        }
    }
}
