use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::Modifier;

pub fn parse_modifier(modifier_str: &str) -> Modifier {
    match modifier_str.to_lowercase().as_str() {
        "bold" => Modifier::BOLD,
        "italic" => Modifier::ITALIC,
        "underline" => Modifier::UNDERLINED,
        "slow_blink" => Modifier::SLOW_BLINK,
        "rapid_blink" => Modifier::RAPID_BLINK,
        "reversed" => Modifier::REVERSED,
        "dim" => Modifier::DIM,
        "crossed_out" => Modifier::CROSSED_OUT,
        // Use Modifier::empty() for ratatui v0.26.0+
        // If using an older version, this might be Modifier::NONE
        _ => Modifier::empty(), // Return no modifier if the string is unrecognized
    }
}

pub fn format_key_event(key_event: KeyEvent) -> String {
    let mut s = String::new();
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
        s.push_str("Ctrl+");
    }
    if key_event.modifiers.contains(KeyModifiers::ALT) {
        s.push_str("Alt+");
    }
    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
        s.push_str("Shift+");
    }
    match key_event.code {
        KeyCode::Char(c) => s.push(c),
        _ => s.push_str(&format!("{:?}", key_event.code)),
    }
    s
}
