#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;

mod templates;

#[get("/")]
fn index() -> templates::IndexTemplate {
    templates::IndexTemplate {}
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from("static"))
}
