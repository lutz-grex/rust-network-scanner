use std::net::IpAddr;
use std::time::Duration;
use std::error::Error;

use futures::future::join;

use crate::models::connection::{Connection, ConnectionStatus, RequestStatus};
use crate::network::helper::build_address;
use crate::network::requests::cve::request_cve;
use crate::network::scanner::banner::scan_banner;
use crate::network::scanner::port::port_scan;


pub async fn fetch_connection_details(target: &IpAddr, port: u16, include_cve: bool, timeout: &Duration) -> Result<Connection, Box<dyn Error + Send + Sync>> {
    let address = build_address(&target.to_string(), port);

    let port_scan_future = port_scan(&address, timeout);
    let banner_scan_future = scan_banner(&address, timeout);


    let (port_scan_result, banner_scan_result) = join(port_scan_future, banner_scan_future).await;



    let port_scan_data = port_scan_result.unwrap_or_default();
    let banner_scan_data = banner_scan_result.unwrap_or_default();

    #[cfg(debug_assertions)] {
        println!("{}", banner_scan_data.server);
    }


    let cves = match include_cve {
        true => request_cve(&banner_scan_data.server, 4.0).await,
        false => Ok(None)
    }?;
    
   


    Ok(Connection { 
        target: target.to_string(), 
        port: port, 
        banner: banner_scan_data.banner, 
        server: banner_scan_data.server,
        web_service: banner_scan_data.web_service,
        status: port_scan_data.status, 
        latency_ms: port_scan_data.latency_ms,
        request_status: if port_scan_data.status == ConnectionStatus::OPEN {RequestStatus::SUCCESS} else {RequestStatus::FAILED},
        cve: cves
    })
    
}