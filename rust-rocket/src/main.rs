#[macro_use]
extern crate rocket;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate {}
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
