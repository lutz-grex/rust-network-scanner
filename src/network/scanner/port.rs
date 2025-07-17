use std::error::Error;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time;
use std::net::{IpAddr};

use crate::models::connection::ConnectionStatus;



pub struct PortScanResult {
    pub status: ConnectionStatus,
    pub latency_ms: Option<u128>,
}

pub async fn port_scan(address: &str, timeout_duration: &Duration) -> Result<PortScanResult, Box<dyn Error + Send + Sync>> {

    let start = Instant::now();

    let scan_result = time::timeout(*timeout_duration, TcpStream::connect(&address)).await;
    
    let latency = start.elapsed().as_millis();


    let connection_status = match scan_result {
        Ok(Ok(_stream)) => ConnectionStatus::OPEN,
        Ok(Err(_)) => ConnectionStatus::CLOSED,
        Err(_) => ConnectionStatus::TIMEOUT
    };


    Ok(PortScanResult {
        status: connection_status,
        latency_ms: if connection_status == ConnectionStatus::OPEN {Some(latency)} else {None}
    })
}