use crate::{
    models::{Todo, Todos},
    templates,
};
use rocket::{form::Form, State};
use std::str::FromStr;
use uuid::Uuid;

#[derive(FromForm)]
pub struct TodoForm<'a> {
    #[field(validate = len(1..))]
    todo: &'a str,
}

#[post("/", data = "<todo_form>")]
pub fn post_todo(todo_form: Form<TodoForm<'_>>, todos: &State<Todos>) -> templates::TodosTemplate {
    let mut todos_guard = todos.todos.write().unwrap();
    todos_guard.push(Todo::new(todo_form.todo.to_string()));
    templates::TodosTemplate {
        todos: todos_guard.clone(),
    }
}

#[delete("/<id>")]
pub fn delete_todo(id: &str, todos: &State<Todos>) -> templates::TodosTemplate {
    let uuid = Uuid::from_str(id);
    if uuid.is_err() {
        return templates::TodosTemplate {
            todos: todos.todos.write().unwrap().clone(),
        };
    }
    let mut todos_guard = todos.todos.write().unwrap();
    todos_guard.retain(|todo| todo.id != uuid.clone().unwrap());
    templates::TodosTemplate {
        todos: todos_guard.clone(),
    }
}
