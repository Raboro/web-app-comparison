#[macro_use]
extern crate rocket;
use std::sync::RwLock;

use models::{Todo, Todos};
use resources::{
    base::{index, not_found},
    todo::{delete_todo, get_all_todos, post_todo},
};
use rocket::fs::FileServer;

mod models;
mod resources;
mod templates;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/todo", routes![post_todo, delete_todo, get_all_todos])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![not_found])
        .manage(Todos {
            todos: RwLock::new(vec![
                Todo::new("Shower".to_string()),
                Todo::new("Exercise".to_string()),
            ]),
        })
}
