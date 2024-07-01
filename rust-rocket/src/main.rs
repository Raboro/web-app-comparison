#[macro_use]
extern crate rocket;
use rocket::{fs::FileServer, State};

mod templates;

struct Todos {
    todos: Vec<String>
}

#[get("/")]
fn index() -> templates::IndexTemplate {
    templates::IndexTemplate {}
}

#[get("/todos")]
fn get_todos(todos: &State<Todos>) -> templates::TodosTemplate {
    templates::TodosTemplate { todos: &todos.todos }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_todos])
        .mount("/static", FileServer::from("static"))
        .manage(Todos {todos: vec!["Shower".to_string(), "Exercise".to_string()]})
}
