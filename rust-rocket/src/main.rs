#[macro_use]
extern crate rocket;
use std::sync::RwLock;

use rocket::{form::Form, fs::FileServer, State};

mod templates;

struct Todos {
    todos: RwLock<Vec<String>>,
}

#[derive(FromForm)]
struct TodoForm<'a> {
    #[field(validate = len(1..))]
    todo: &'a str
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
fn post_todo(todo_form: Form<TodoForm<'_>>, todos: &State<Todos>) -> templates::TodosTemplate{
    let mut todos_guard = todos.todos.write().unwrap();
    todos_guard.push(todo_form.todo.to_string());
    templates::TodosTemplate {
        todos: todos_guard.clone()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_todos, post_todo])
        .mount("/static", FileServer::from("static"))
        .manage(Todos {
            todos: RwLock::new(vec!["Shower".to_string(), "Exercise".to_string()]),
        })
}
