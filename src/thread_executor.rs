use std::{collections::HashSet, net::IpAddr, sync::Arc, time::Duration};

use anyhow::anyhow;
use futures::future::join_all;
use indicatif::ProgressBar;
use tokio::sync::Semaphore;

use crate::{
    models::connection::Connection, network::connection_scanner::fetch_connection_details,
};

pub async fn thread_fetch_connection_details(
    target: &Vec<IpAddr>,
    ports: &HashSet<u16>,
    timeout: u64,
    concurrency: usize,
    include_cve: bool,
) -> Result<Vec<Connection>, anyhow::Error> {
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let timeout_duration = Duration::from_millis(timeout);

    let scan_process: u64 = (target.len() * ports.len())
        .try_into()
        .map_err(|_| anyhow!("Invalid scan process count"))?;
    let bar = ProgressBar::new(scan_process);

    let scan_futures = target
        .iter()
        .flat_map(|ip| {
            let semaphore = semaphore.clone();
            let ip = *ip;
            let bar = bar.clone();

            ports.iter().map(move |port| {
                let semaphore = semaphore.clone();
                let port = *port;
                let timeout_duration = timeout_duration.clone();
                let include_cve = include_cve.clone();
                let bar = bar.clone();

                async move {
                    let _permit = semaphore
                        .acquire()
                        .await
                        .map_err(|e| anyhow!("Semaphore Error, {}", e.to_string()))?;

                    let result =
                        fetch_connection_details(&ip, port, include_cve, &timeout_duration).await;
                    bar.inc(1);
                    result
                }
            })
        })
        .collect::<Vec<_>>();

    let scan_results = join_all(scan_futures).await;

    let mut scan_success: Vec<Connection> =
        scan_results.into_iter().filter_map(Result::ok).collect();

    scan_success.sort_by(|a, b| {
        a.connection_status
            .cmp(&b.connection_status)
            .then_with(|| a.port.cmp(&b.port))
    });

    Ok(scan_success)
}
