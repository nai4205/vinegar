pub mod actions;
pub mod state;
pub mod task;

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
            terminal.draw(|frame| ui::ui(frame, &mut self))?; // Pass mutable self
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
                                let mut task_ref = &mut self.tasks[task_path[0]];
                                for &index in task_path.iter().skip(1) {
                                    task_ref = &mut task_ref.subtasks[index];
                                }
                                task_ref
                                    .subtasks
                                    .push(Task::new(self.input.drain(..).collect()));
                                task_ref.expanded = true;
                            }
                        } else {
                            self.tasks.push(Task::new(self.input.drain(..).collect()));
                        }
                        self.mode = AppMode::Normal;
                    }
                    AppEvent::UpdateTask => {
                        if let AppMode::EditingTask { path } = &self.mode {
                            let mut task_ref = &mut self.tasks[path[0]];
                            for &index in path.iter().skip(1) {
                                task_ref = &mut task_ref.subtasks[index];
                            }
                            task_ref.name = self.input.drain(..).collect();
                        }
                        self.mode = AppMode::Normal;
                    }
                    AppEvent::DeleteTask => {
                        if let Some(selected_index) = self.task_list_state.selected() {
                            let tasks_to_display = self.get_tasks_to_display();
                            if let Some(selected_task_info) = tasks_to_display.get(selected_index) {
                                let task_path = &selected_task_info.1;
                                let mut task_ref = &mut self.tasks[task_path[0]];
                                for &index in task_path.iter().skip(1) {
                                    task_ref = &mut task_ref.subtasks[index];
                                }
                                task_ref.subtasks.pop();
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
