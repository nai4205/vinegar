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
