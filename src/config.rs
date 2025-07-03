use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub keys: Keybindings,
    #[serde(default)]
    pub layout: LayoutConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Keybindings {
    pub quit: KeyEvent,
    pub add_task: KeyEvent,
    pub delete_task: KeyEvent,
    pub edit_task: KeyEvent,
    pub deselect: KeyEvent,
    pub toggle_expand: KeyEvent,
    pub select_next: KeyEvent,
    pub select_previous: KeyEvent,
}

#[derive(Debug, Deserialize)]
pub struct LayoutConfig {
    pub direction: LayoutDirection,
    pub constraints: Vec<u16>,
}

#[derive(Debug, Deserialize)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct ThemeConfig {
    pub colors: ColorsConfig,
    pub other: OtherConfig,
    pub icons: IconsConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ColorsConfig {
    pub main_fg: String,
    pub input_fg: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct OtherConfig {
    pub highlight_mod: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct IconsConfig {
    pub expanded: String,
    pub collapsed: String,
    pub highlight_symbol: String,
}

impl Default for IconsConfig {
    fn default() -> Self {
        Self {
            expanded: "▼".to_string(),
            collapsed: "▶".to_string(),
            highlight_symbol: "> ".to_string(),
        }
    }
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            add_task: KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
            delete_task: KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE),
            edit_task: KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
            deselect: KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
            toggle_expand: KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            select_next: KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            select_previous: KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
        }
    }
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            constraints: vec![80, 20],
        }
    }
}

impl Default for ColorsConfig {
    fn default() -> Self {
        Self {
            main_fg: "White".to_string(),
            input_fg: "Yellow".to_string(),
        }
    }
}

impl Default for OtherConfig {
    fn default() -> Self {
        Self {
            highlight_mod: "Bold".to_string(),
        }
    }
}

pub fn load_config() -> Config {
    let path = Path::new("config.toml");
    if path.exists() {
        let config_str = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&config_str).expect("Failed to parse config file")
    } else {
        Config::default()
    }
}
