use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub keys: Keybindings,
}

#[derive(Debug, Deserialize)]
pub struct Keybindings {
    pub quit: KeyEvent,
    pub add_task: KeyEvent,
    pub edit_task: KeyEvent,
    pub deselect: KeyEvent,
    pub toggle_expand: KeyEvent,
    pub select_next: KeyEvent,
    pub select_previous: KeyEvent,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            add_task: KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
            edit_task: KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
            deselect: KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
            toggle_expand: KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            select_next: KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            select_previous: KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
        }
    }
}

pub fn load_config() -> Config {
    let path = Path::new("config.toml");
    if path.exists() {
        let config_str = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&config_str).expect("Failed to parse config file")
    } else {
        Config {
            keys: Keybindings::default(),
        }
    }
}
