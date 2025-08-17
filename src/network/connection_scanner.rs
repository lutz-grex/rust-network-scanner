use std::net::IpAddr;
use std::time::Duration;

use crate::models::connection::Connection;
use crate::network::util::string_formatter;
use crate::network::requests::cve::request_cve;
use crate::network::scanner::banner::scan_banner;
use crate::network::scanner::port::port_scan;
use futures::future::join;

pub async fn fetch_connection_details(
    target: &IpAddr,
    port: u16,
    include_cve: bool,
    timeout: &Duration,
) -> Result<Connection, anyhow::Error> {
    let address = string_formatter::build_address(&target.to_string(), port);

    let port_scan_future = port_scan(&address, timeout);
    let banner_scan_future = scan_banner(&address, timeout);

    let (port_scan_result, banner_scan_result) = join(port_scan_future, banner_scan_future).await;

    let port_data = port_scan_result.ok();
    let banner_data = banner_scan_result.ok();

    let cves = if include_cve {
        match &banner_data {
            Some(banner) => request_cve(&banner.server, 4.0).await?,
            None => None,
        }
    } else {
        None
    };

    Ok(Connection::from_results(
        &target.clone(),
        port,
        port_data,
        banner_data,
        cves,
    ))
}
