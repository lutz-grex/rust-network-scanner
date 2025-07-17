use std::time::Duration;

use futures::{future::join_all, TryFutureExt};
use rocket::{http::{ext::IntoCollection, Status}, serde::json::Json, Route};
use serde::Deserialize;

use crate::{cli::extractor::parse_ip_addr_input, models::connection::Connection, thread_executor::thread_fetch_connection_details};



#[derive(FromForm)]
pub struct ScanParams<'r> {
    target: &'r str,
    ports: &'r str,
    concurrency: Option<usize>,
    timeout_ms: Option<u64>,
    banner: Option<bool>,
}

#[get("/scan?<params..>")]
pub async fn scan<'r>(params: ScanParams<'r>) -> Result<Json<Vec<Connection>>, (rocket::http::Status, String)> {
    let timeout_ms = params.timeout_ms.unwrap_or(1000);
    let banner = params.banner.unwrap_or(false);
    let concurrency = params.concurrency.unwrap_or(100);
    let ips = parse_ip_addr_input(params.target);

    let connections = thread_fetch_connection_details(&ips, &params.ports, timeout_ms, concurrency, banner)
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