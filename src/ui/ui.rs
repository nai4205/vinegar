use crate::app::{state::AppMode, App};
use crate::config::LayoutDirection;
use crate::ui::utils::{format_key_event, parse_modifier};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
    Frame,
};
use std::str::FromStr;

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

    // --- Apply Nested Theme from Config ---
    let theme = &app.config.theme;
    let main_fg = Color::from_str(&theme.colors.main_fg).unwrap_or(Color::White);
    let input_fg = Color::from_str(&theme.colors.input_fg).unwrap_or(Color::Yellow);
    let highlight_mod = parse_modifier(&theme.other.highlight_mod);
    let highlight_symbol = &theme.icons.highlight_symbol; // Get the highlight symbol

    // Main task list
    let main_block = Block::bordered()
        .title("Tasks")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let tasks_to_display = app.get_tasks_to_display();
    let tasks: Vec<ListItem> = tasks_to_display
        .iter()
        .map(|(display_name, _)| ListItem::new(display_name.as_str().to_string()))
        .collect();

    let task_list = List::new(tasks)
        .block(main_block)
        .style(Style::default().fg(main_fg)) // Use themed color
        .highlight_style(Style::default().add_modifier(highlight_mod)) // Use themed modifier
        .highlight_symbol(highlight_symbol);

    frame.render_stateful_widget(task_list, chunks[0], &mut app.task_list_state);

    // Input/Editing block
    let help_text;
    let (input_title, mut input_style) = match app.mode {
        AppMode::Editing => (
            "Add Task (Press Enter to submit)",
            Style::default().fg(input_fg), // Use themed color
        ),
        AppMode::EditingTask { .. } => (
            "Edit Task (Press Enter to submit)",
            Style::default().fg(input_fg), // Use themed color
        ),
        AppMode::Normal => {
            let keybindings = &app.config.keys;
            help_text = format!(
                "Press '{}' to add, '{}' to deselect, '{}' to edit, '{} to delete', '{}/{}' to navigate, {} to expand, '{}' to quit",
                format_key_event(keybindings.add_task),
                format_key_event(keybindings.deselect),
                format_key_event(keybindings.edit_task),
                format_key_event(keybindings.delete_task),
                format_key_event(keybindings.select_previous),
                format_key_event(keybindings.select_next),
                format_key_event(keybindings.toggle_expand),
                format_key_event(keybindings.quit),
            );
            (help_text.as_str(), Style::default())
        }
    };

    // In Normal mode, the style is default, let's make sure it has the right foreground
    if app.mode == AppMode::Normal {
        input_style = input_style.fg(main_fg);
    }

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
