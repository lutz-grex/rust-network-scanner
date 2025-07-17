use rocket::{serde::json::Json, Route};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthStatus {
    status: &'static str,
    status_code: u16
}


#[get("/")]
pub async fn scan() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: "OK",
        status_code: 200
    })
}



pub fn routes() -> Vec<Route> {
    routes![scan]
}