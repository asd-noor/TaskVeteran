#[derive(PartialEq)]
pub struct ToDoListItem {
    pub label: String,
    pub completed: bool,
}

impl ToDoListItem {
    pub fn new(label: &str) -> Self {
        ToDoListItem {
            label: String::from(label),
            completed: false,
        }
    }
}
