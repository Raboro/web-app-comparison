use askama::Template;

use crate::Todo;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

#[derive(Template)]
#[template(path = "todos.html")]
pub struct Todos {
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFound;
