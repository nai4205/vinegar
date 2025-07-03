use crate::app::task::Task;

/// Finds a mutable reference to a task in the tree using its path.
pub fn get_task_mut<'a>(tasks: &'a mut [Task], path: &[usize]) -> Option<&'a mut Task> {
    if path.is_empty() {
        return None;
    }

    // Since we start with a slice, we can't just index it without a bounds check.
    // We get the first part of the path and the rest of it.
    let (first_index, rest_of_path) = path.split_first()?; // Safely get the head and tail of the path

    let mut current_task = tasks.get_mut(*first_index)?; // Safely get the first task

    for &index in rest_of_path {
        current_task = current_task.subtasks.get_mut(index)?; // Safely get subtasks
    }

    Some(current_task)
}
