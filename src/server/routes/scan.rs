use std::time::Duration;

use futures::{future::join_all, TryFutureExt};
use rocket::{http::{ext::IntoCollection, Status}, serde::json::Json, Route};
use serde::Deserialize;

use crate::{cli::extractor::{parse_ip_addr_input, parse_port_input}, models::connection::Connection, server::helper::validate::Validatable, thread_executor::thread_fetch_connection_details};



#[derive(FromForm)]
pub struct ScanParams<'r> {
    target: &'r str,
    ports: &'r str,
    concurrency: Option<usize>,
    timeout: Option<u64>,
    cve: Option<bool>,
}


impl<'a> Validatable for ScanParams<'a> {
    fn validate(&self) -> Result<(),  (rocket::http::Status, String)> {

        println!("{:?}", self.target);

        if self.target.trim().is_empty() {
            return Err((Status::BadRequest, "Invalid input for target".into()));
        }

        if self.ports.trim().is_empty() {
            return Err((Status::BadRequest, "Invalid input for port".into()));
        }

        Ok(())
    }
}


#[get("/scan?<params..>")]
pub async fn scan<'r>(params: ScanParams<'r>) -> Result<Json<Vec<Connection>>, (rocket::http::Status, String)> {

    params.validate()?;

    let timeout_ms = params.timeout.unwrap_or(1000);
    let include_cve = params.cve.unwrap_or(false);
    let concurrency = params.concurrency.unwrap_or(100);
    let ips = parse_ip_addr_input(params.target)
        .map_err(|e| (Status::BadRequest, e.to_string()))?;
    let ports = parse_port_input(&params.ports)
        .map_err(|e| (Status::BadRequest, e.to_string()))?;

    let connections = thread_fetch_connection_details(&ips, &ports, timeout_ms, concurrency, include_cve)
        .await
        .map_err(|e| (Status::InternalServerError, format!("Scan failed: {e}")))?;

    Ok(Json(connections))  
}



pub fn routes() -> Vec<Route> {
    routes![scan]
}

/*


    let connection_futures = ports.iter()
        .map(|p| {
            let target = params.target.clone();

            async move {
                match port_scan(&target, *p, banner, &timeout_duration).await {
                    Ok(con) => Some(con),
                    Err(_) => None
                }
            }
        });

    let connections: Vec<Connection> = join_all(connection_futures).await.into_iter().filter_map(|c| c).collect();


*/