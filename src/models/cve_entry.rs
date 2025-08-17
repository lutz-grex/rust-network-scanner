use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct CveEntry {
    pub title: String,
    pub score: f32,
    pub severity: String,
    pub description: String,
    pub href: String,
}
