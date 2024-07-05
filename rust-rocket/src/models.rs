use core::fmt;
use std::sync::RwLock;

use uuid::Uuid;

pub struct Todos {
    pub todos: RwLock<Vec<Todo>>,
}

#[derive(Clone)]
pub struct Todo {
    pub name: String,
    pub id: Uuid,
}

impl Todo {
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: Uuid::new_v4(),
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.id)
    }
}