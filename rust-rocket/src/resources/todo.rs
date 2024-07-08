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

#[get("/")]
pub fn get_all_todos(todos: &State<Todos>) -> templates::Todos {
    let todos_guard = todos.todos.read().unwrap().to_vec();
    templates::Todos {
        todos: todos_guard.clone(),
    }
}

#[post("/", data = "<todo_form>")]
pub fn post_todo(todo_form: Form<TodoForm<'_>>, todos: &State<Todos>) -> templates::Todos {
    let mut todos_guard = todos.todos.write().unwrap();
    todos_guard.push(Todo::new(todo_form.todo.to_string()));
    templates::Todos {
        todos: todos_guard.clone(),
    }
}

#[delete("/<id>")]
pub fn delete_todo(id: &str, todos: &State<Todos>) -> templates::Todos {
    let uuid = Uuid::from_str(id);
    if uuid.is_err() {
        return templates::Todos {
            todos: todos.todos.write().unwrap().clone(),
        };
    }
    let mut todos_guard = todos.todos.write().unwrap();
    todos_guard.retain(|todo| todo.id != uuid.clone().unwrap());
    templates::Todos {
        todos: todos_guard.clone(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use rocket::{http::Status, local::blocking::Client};

    use crate::models::Todos;

    use super::{delete_todo, get_all_todos, post_todo};

    fn rocket() -> rocket::Rocket<rocket::Build> {
        rocket::build()
            .manage(Todos {
                todos: RwLock::new(Vec::new()),
            })
            .mount("/", routes![get_all_todos, post_todo, delete_todo])
    }

    #[test]
    fn get_todos_should_get_all_todos() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get(uri!(super::get_all_todos)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            "<ul id=\"taskList\" class=\"list-disc\">\n    \n</ul>"
        );
    }

    #[test]
    fn test_post_todo() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client
            .post("/")
            .header(rocket::http::ContentType::Form)
            .body("todo=Test%20Todo")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("Test Todo"));
    }

    #[test]
    fn test_delete_todo() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client
            .post("/")
            .header(rocket::http::ContentType::Form)
            .body("todo=Test%20Todo")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        println!("{}", body);

        let start = body.find("hx-delete=\"/todo/").unwrap() + 17;
        let end = start + 36;
        let id = &body[start..end];

        let response = client.delete(format!("/{}", id)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(!body.contains(id));
    }
}
