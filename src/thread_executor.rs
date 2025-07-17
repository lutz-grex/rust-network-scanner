use std::{error::Error, net::IpAddr, sync::Arc, time::Duration};

use futures::future::join_all;
use tokio::sync::Semaphore;
use indicatif::ProgressBar;

use crate::{cli::extractor::parse_port_input, models::connection::Connection, network::{self, connection_scanner::fetch_connection_details}};



pub async fn thread_fetch_connection_details(target: &Vec<IpAddr>, ports: &str, timeout: u64, concurrency: usize, banner: bool) -> Result<Vec<Connection>, Box<dyn Error + Send + Sync>> {
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let timeout_duration = Duration::from_millis(timeout);
    let ports_set = parse_port_input(&ports);

    let scan_process: u64 = (target.len() * ports_set.len()).try_into().unwrap();
    let bar = ProgressBar::new(scan_process);


    let scan_futures = target.iter().flat_map(|ip| {
        let semaphore = semaphore.clone();
        let ip = *ip;
        let bar = bar.clone();

        ports_set.iter().map(move |port| {
            let semaphore = semaphore.clone();
            let port = *port;
            let timeout_duration = timeout_duration.clone();
            let banner = banner.clone();
            let bar = bar.clone();

            async move {
                let _permit = semaphore.acquire().await
                    .map_err(|e| format!("Semaphore Error, {}", e.to_string()))?;

                
                let result = fetch_connection_details(&ip, port, banner, &timeout_duration).await;
                bar.inc(1);
                result
            }
        })
    }).collect::<Vec<_>>();

    let scan_results = join_all(scan_futures).await;

    let mut scan_success: Vec<Connection> = scan_results
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    scan_success.sort_by(|a, b| {
        a.status.cmp(&b.status).then_with(|| a.port.cmp(&b.port))
    });

    Ok(scan_success)
}