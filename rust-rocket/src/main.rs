#[macro_use]
extern crate rocket;
use rocket::{form::Form, fs::FileServer, State};

mod templates;

struct Todos {
    todos: Vec<String>,
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
    templates::TodosTemplate {
        todos: &todos.todos,
    }
}

#[post("/todo", data = "<todo_form>")]
fn post_todo(todo_form: Form<TodoForm<'_>>) {
    println!("{}", todo_form.todo);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_todos, post_todo])
        .mount("/static", FileServer::from("static"))
        .manage(Todos {
            todos: vec!["Shower".to_string(), "Exercise".to_string()],
        })
}
