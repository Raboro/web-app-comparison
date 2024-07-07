use crate::templates;

#[get("/")]
pub fn index() -> templates::Index {
    templates::Index {}
}

#[catch(404)]
pub fn not_found() -> templates::NotFound {
    templates::NotFound
}
