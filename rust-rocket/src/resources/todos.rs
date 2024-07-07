use rocket::State;

use crate::{models::Todos, templates};

#[get("/")]
pub fn get_todos(todos: &State<Todos>) -> templates::Todos {
    let todos_guard = todos.todos.read().unwrap().to_vec();
    templates::Todos {
        todos: todos_guard.clone(),
    }
}