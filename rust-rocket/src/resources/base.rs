use crate::templates;

#[get("/")]
pub fn index() -> templates::Index {
    templates::Index {}
}
