use crate::app::{App, AppMode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(frame.area());

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
    let (input_title, input_style) = match app.mode {
        AppMode::Editing => (
            "Add Task (Press Enter to submit)",
            Style::default().fg(Color::Yellow),
        ),
        AppMode::EditingTask { .. } => (
            "Edit Task (Press Enter to submit)",
            Style::default().fg(Color::Yellow),
        ),
        AppMode::Normal => (
            "Press 'a' to add, 'd' to deselect, 'e' to edit, 'j'/'k' to navigate, Enter to expand",
            Style::default(),
        ),
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
