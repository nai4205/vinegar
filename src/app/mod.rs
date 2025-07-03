pub mod actions;
pub mod state;
pub mod task;
pub mod task_utils;

use crate::config::Config;
use crate::event::{AppEvent, Event, EventHandler};
use crate::ui;
use ratatui::widgets::ListState;
use ratatui::DefaultTerminal;
use state::AppMode;
use task::Task;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    pub tasks: Vec<Task>,
    pub input: String,
    pub mode: AppMode,
    pub task_list_state: ListState,
    pub config: Config,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(config: Config) -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            tasks: Vec::new(),
            input: String::new(),
            mode: AppMode::Normal,
            task_list_state: ListState::default(),
            config,
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| ui::ui(frame, &mut self))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        actions::handle_key_events(key_event, &mut self)?
                    }
                }
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::AddTask => {
                        if let Some(selected_index) = self.task_list_state.selected() {
                            let tasks_to_display = self.get_tasks_to_display();
                            if let Some(selected_task_info) = tasks_to_display.get(selected_index) {
                                let task_path = &selected_task_info.1;
                                if let Some(task) =
                                    task_utils::get_task_mut(&mut self.tasks, task_path)
                                {
                                    task.subtasks
                                        .push(Task::new(self.input.drain(..).collect()));
                                    task.expanded = true;
                                }
                            }
                        } else {
                            self.tasks.push(Task::new(self.input.drain(..).collect()));
                        }
                        self.mode = AppMode::Normal;
                    }
                    AppEvent::UpdateTask => {
                        if let AppMode::EditingTask { path } = &self.mode {
                            if let Some(task) = task_utils::get_task_mut(&mut self.tasks, path) {
                                task.name = self.input.drain(..).collect();
                            }
                        }
                        self.mode = AppMode::Normal;
                    }
                    AppEvent::DeleteTask => {
                        if let Some(selected_index) = self.task_list_state.selected() {
                            let tasks_to_display = self.get_tasks_to_display();
                            if let Some(selected_task_info) =
                                tasks_to_display.get(selected_index).cloned()
                            {
                                let task_path = selected_task_info.1;

                                if task_path.len() > 1 {
                                    let parent_path = &task_path[..task_path.len() - 1];
                                    let task_index = *task_path.last().unwrap();
                                    if let Some(parent_task) =
                                        task_utils::get_task_mut(&mut self.tasks, parent_path)
                                    {
                                        parent_task.subtasks.remove(task_index);
                                    }
                                } else {
                                    let task_index = task_path[0];
                                    self.tasks.remove(task_index);
                                }

                                let task_count = self.get_tasks_to_display().len();
                                if task_count == 0 {
                                    self.task_list_state.select(None);
                                } else if selected_index >= task_count {
                                    self.task_list_state.select(Some(task_count - 1));
                                }
                            }
                        }
                    }
                },
            }
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn get_tasks_to_display(&self) -> Vec<(String, Vec<usize>)> {
        let mut display_tasks = Vec::new();
        for (i, task) in self.tasks.iter().enumerate() {
            self.add_task_to_display(&mut display_tasks, task, vec![i], 0);
        }
        display_tasks
    }

    fn add_task_to_display(
        &self,
        display_tasks: &mut Vec<(String, Vec<usize>)>,
        task: &Task,
        path: Vec<usize>,
        depth: usize,
    ) {
        let prefix = " ".repeat(depth * 2);
        let display_name = format!("{}☐ {}", prefix, task.name);
        display_tasks.push((display_name, path.clone()));

        if task.expanded {
            for (i, subtask) in task.subtasks.iter().enumerate() {
                let mut sub_path = path.clone();
                sub_path.push(i);
                self.add_task_to_display(display_tasks, subtask, sub_path, depth + 1);
            }
        }
    }
}
