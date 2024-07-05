#[macro_use]
extern crate rocket;
use std::{str::FromStr, sync::RwLock};

use models::{Todo, Todos};
use rocket::{form::Form, fs::FileServer, State};
use uuid::Uuid;

mod templates;
mod models;

#[derive(FromForm)]
struct TodoForm<'a> {
    #[field(validate = len(1..))]
    todo: &'a str,
}

#[get("/")]
fn index() -> templates::IndexTemplate {
    templates::IndexTemplate {}
}

#[get("/todos")]
fn get_todos(todos: &State<Todos>) -> templates::TodosTemplate {
    let todos_guard = todos.todos.read().unwrap().to_vec();
    templates::TodosTemplate {
        todos: todos_guard.clone(),
    }
}

#[post("/todo", data = "<todo_form>")]
fn post_todo(todo_form: Form<TodoForm<'_>>, todos: &State<Todos>) -> templates::TodosTemplate {
    let mut todos_guard = todos.todos.write().unwrap();
    todos_guard.push(Todo::new(todo_form.todo.to_string()));
    templates::TodosTemplate {
        todos: todos_guard.clone(),
    }
}

#[delete("/todo/<id>")]
fn delete_todo(id: &str, todos: &State<Todos>) -> templates::TodosTemplate {
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_todos, post_todo, delete_todo])
        .mount("/static", FileServer::from("static"))
        .manage(Todos {
            todos: RwLock::new(vec![
                Todo::new("Shower".to_string()),
                Todo::new("Exercise".to_string()),
            ]),
        })
}
