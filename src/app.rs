use crate::event::{AppEvent, Event, EventHandler};
use crate::ui;
use ratatui::widgets::ListState;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub subtasks: Vec<Task>,
    pub expanded: bool,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            subtasks: Vec::new(),
            expanded: false,
        }
    }
}

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
}

#[derive(Debug, PartialEq)]
pub enum AppMode {
    Normal,
    Editing,
    EditingTask { path: Vec<usize> },
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            tasks: Vec::new(),
            input: String::new(),
            mode: AppMode::Normal,
            task_list_state: ListState::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| ui::ui(frame, &mut self))?; // Pass mutable self
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        self.handle_key_events(key_event)?
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
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match self.mode {
            AppMode::Normal => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    self.events.send(AppEvent::Quit)
                }
                KeyCode::Char('a') => {
                    self.mode = AppMode::Editing;
                }
                KeyCode::Char('e') => {
                    if let Some(selected) = self.task_list_state.selected() {
                        let tasks_to_display = self.get_tasks_to_display();
                        if let Some(selected_task_info) = tasks_to_display.get(selected) {
                            let task_path = selected_task_info.1.clone();
                            let mut task_ref = &self.tasks[task_path[0]];
                            for &index in task_path.iter().skip(1) {
                                task_ref = &task_ref.subtasks[index];
                            }
                            self.input = task_ref.name.clone();
                            self.mode = AppMode::EditingTask { path: task_path };
                        }
                    }
                }
                KeyCode::Enter => self.toggle_expand_task(),
                KeyCode::Char('k') => self.select_previous_task(),
                KeyCode::Char('j') => self.select_next_task(),
                _ => {}
            },
            AppMode::Editing => match key_event.code {
                KeyCode::Enter => self.events.send(AppEvent::AddTask),
                KeyCode::Char(c) => {
                    self.input.push(c);
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Esc => {
                    self.mode = AppMode::Normal;
                }
                _ => {}
            },
            AppMode::EditingTask { .. } => match key_event.code {
                KeyCode::Enter => self.events.send(AppEvent::UpdateTask),
                KeyCode::Char(c) => self.input.push(c),
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Esc => self.mode = AppMode::Normal,
                _ => {}
            },
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    ///Some -> takes a special type: option. option can be something or nothing,
    fn toggle_expand_task(&mut self) {
        if let Some(selected_index) = self.task_list_state.selected() {
            let tasks_to_display = self.get_tasks_to_display();
            if let Some(selected_task_info) = tasks_to_display.get(selected_index) {
                let task_path = &selected_task_info.1;
                let mut task_ref = &mut self.tasks[task_path[0]];
                for &index in task_path.iter().skip(1) {
                    task_ref = &mut task_ref.subtasks[index];
                }
                task_ref.expanded = !task_ref.expanded;
            }
        }
    }

    fn select_next_task(&mut self) {
        let task_count = self.get_tasks_to_display().len();
        if task_count == 0 {
            return;
        }
        let i = match self.task_list_state.selected() {
            Some(i) => {
                if i >= task_count - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.task_list_state.select(Some(i));
    }

    fn select_previous_task(&mut self) {
        let task_count = self.get_tasks_to_display().len();
        if task_count == 0 {
            return;
        }
        let i = match self.task_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    task_count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.task_list_state.select(Some(i));
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
        let display_name = format!("{}{}", prefix, task.name);
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
