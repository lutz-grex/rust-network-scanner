
pub trait Validatable {
    fn validate(&self) -> Result<(),  (rocket::http::Status, String)>;
}