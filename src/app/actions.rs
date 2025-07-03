use super::{App, AppMode};
use crate::event::AppEvent;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> color_eyre::Result<()> {
    match app.mode {
        AppMode::Normal => {
            if key_event == app.config.keys.quit {
                app.events.send(AppEvent::Quit);
            } else if key_event == app.config.keys.add_task {
                app.mode = AppMode::Editing;
            } else if key_event == app.config.keys.delete_task {
                app.events.send(AppEvent::DeleteTask);
            } else if key_event == app.config.keys.edit_task {
                if let Some(selected) = app.task_list_state.selected() {
                    let tasks_to_display = app.get_tasks_to_display();
                    if let Some(selected_task_info) = tasks_to_display.get(selected) {
                        let task_path = selected_task_info.1.clone();
                        let mut task_ref = &app.tasks[task_path[0]];
                        for &index in task_path.iter().skip(1) {
                            task_ref = &task_ref.subtasks[index];
                        }
                        app.input = task_ref.name.clone();
                        app.mode = AppMode::EditingTask { path: task_path };
                    }
                }
            } else if key_event == app.config.keys.toggle_expand {
                toggle_expand_task(app);
            } else if key_event == app.config.keys.select_previous {
                select_previous_task(app);
            } else if key_event == app.config.keys.select_next {
                select_next_task(app);
            } else if key_event == app.config.keys.deselect {
                app.task_list_state.select(None);
            }
        }
        AppMode::Editing => match key_event.code {
            KeyCode::Enter => app.events.send(AppEvent::AddTask),
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Esc => {
                app.mode = AppMode::Normal;
            }
            _ => {}
        },
        AppMode::EditingTask { .. } => match key_event.code {
            KeyCode::Enter => app.events.send(AppEvent::UpdateTask),
            KeyCode::Char(c) => app.input.push(c),
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Esc => app.mode = AppMode::Normal,
            _ => {}
        },
    }
    Ok(())
}

fn toggle_expand_task(app: &mut App) {
    if let Some(selected_index) = app.task_list_state.selected() {
        let tasks_to_display = app.get_tasks_to_display();
        if let Some(selected_task_info) = tasks_to_display.get(selected_index) {
            let task_path = &selected_task_info.1;
            let mut task_ref = &mut app.tasks[task_path[0]];
            for &index in task_path.iter().skip(1) {
                task_ref = &mut task_ref.subtasks[index];
            }
            task_ref.expanded = !task_ref.expanded;
        }
    }
}

fn select_next_task(app: &mut App) {
    let task_count = app.get_tasks_to_display().len();
    if task_count == 0 {
        return;
    }
    let i = match app.task_list_state.selected() {
        Some(i) => {
            if i >= task_count - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    };
    app.task_list_state.select(Some(i));
}

fn select_previous_task(app: &mut App) {
    let task_count = app.get_tasks_to_display().len();
    if task_count == 0 {
        return;
    }
    let i = match app.task_list_state.selected() {
        Some(i) => {
            if i == 0 {
                task_count - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };
    app.task_list_state.select(Some(i));
}
