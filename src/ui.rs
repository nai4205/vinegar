use crate::app::{App, AppMode};
use crate::config::LayoutDirection;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
    Frame,
};

fn format_key_event(key_event: KeyEvent) -> String {
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

pub fn ui(frame: &mut Frame, app: &mut App) {
    //=> are match arms, below is matching the variable direction to see if it is horizontal or
    //vertical then setting it to those directions
    let direction = match app.config.layout.direction {
        LayoutDirection::Horizontal => Direction::Horizontal,
        LayoutDirection::Vertical => Direction::Vertical,
    };

    let constraints: Vec<Constraint> = app
        .config
        .layout
        .constraints
        .iter()
        .map(|&p| Constraint::Percentage(p))
        .collect();
    /*
    The |&p| is called a closure, the p represents one item
    from the iterator at a time, the & dereferences the p to a number from the constraints
    in the config, then converts it to a percentage.
    So above converts, [80,20] into [Constraint::Percentage(80), Constraint::Percentage(20)]
    Which can be used by ratatui
    */
    let chunks = Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(frame.area());
    //The values defined above; direction and constraint are used here

    // Main task list
    let main_block = Block::bordered()
        .title("Tasks")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let tasks_to_display = app.get_tasks_to_display();
    let tasks: Vec<ListItem> = tasks_to_display
        .iter()
        .map(|(display_name, _)| ListItem::new(format!("{}", display_name.as_str())))
        .collect();

    let task_list = List::new(tasks)
        .block(main_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(task_list, chunks[0], &mut app.task_list_state);

    // Input/Editing block
    let help_text;
    let (input_title, input_style) = match app.mode {
        AppMode::Editing => (
            "Add Task (Press Enter to submit)",
            Style::default().fg(Color::Yellow),
        ),
        AppMode::EditingTask { .. } => (
            "Edit Task (Press Enter to submit)",
            Style::default().fg(Color::Yellow),
        ),
        AppMode::Normal => {
            let keybindings = &app.config.keys;
            help_text = format!(
                "Press '{}' to add, '{}' to deselect, '{}' to edit, '{}/{}' to navigate, {} to expand, '{}' to quit",
                format_key_event(keybindings.add_task),
                format_key_event(keybindings.deselect),
                format_key_event(keybindings.edit_task),
                format_key_event(keybindings.select_previous),
                format_key_event(keybindings.select_next),
                format_key_event(keybindings.toggle_expand),
                format_key_event(keybindings.quit),
            );
            (help_text.as_str(), Style::default())
        }
    };

    let input_block = Block::bordered()
        .title(input_title)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let input_paragraph = Paragraph::new(app.input.as_str())
        .block(input_block)
        .style(input_style);

    frame.render_widget(input_paragraph, chunks[1]);

    // Set cursor position only when in an editing mode
    if let AppMode::Editing | AppMode::EditingTask { .. } = app.mode {
        frame.set_cursor_position((chunks[1].x + app.input.len() as u16 + 1, chunks[1].y + 1));
    }
}
