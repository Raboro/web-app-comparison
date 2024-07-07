use crate::templates;

#[get("/")]
pub fn index() -> templates::IndexTemplate {
    templates::IndexTemplate {}
}
