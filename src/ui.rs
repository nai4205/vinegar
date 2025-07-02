use crate::app::{App, AppMode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(frame.area());

    let main_block = Block::bordered()
        .title("Tasks")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let tasks: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| ListItem::new(task.as_str()))
        .collect();

    let task_list = List::new(tasks)
        .block(main_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(ratatui::style::Modifier::ITALIC))
        .highlight_symbol(">> ");

    frame.render_widget(task_list, chunks[0]);

    let input_block = Block::bordered()
        .title("Add Task")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let input_paragraph =
        Paragraph::new(app.input.as_str())
            .block(input_block)
            .style(match app.mode {
                AppMode::Editing => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            });

    frame.render_widget(input_paragraph, chunks[1]);
    if app.mode == AppMode::Editing {
        frame.set_cursor_position((chunks[1].x + app.input.len() as u16 + 1, chunks[1].y + 1));
    }
}
