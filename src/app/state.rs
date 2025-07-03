#[derive(Debug, PartialEq)]
pub enum AppMode {
    Normal,
    Editing,
    EditingTask { path: Vec<usize> },
}
