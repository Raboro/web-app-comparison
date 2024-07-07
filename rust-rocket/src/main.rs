#[macro_use]
extern crate rocket;
use std::sync::RwLock;

use models::{Todo, Todos};
use resources::{
    base::index,
    todo::{delete_todo, post_todo}, todos::get_todos,
};
use rocket::fs::FileServer;

mod models;
mod resources;
mod templates;

#[catch(404)]
fn not_found() -> templates::NotFoundTemplate {
    templates::NotFoundTemplate
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/todo", routes![post_todo, delete_todo])
        .mount("/todos", routes![get_todos])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![not_found])
        .manage(Todos {
            todos: RwLock::new(vec![
                Todo::new("Shower".to_string()),
                Todo::new("Exercise".to_string()),
            ]),
        })
}
