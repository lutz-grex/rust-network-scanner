use crate::models::cve_entry::CveEntry;
use crate::models::status::{ConnectionStatus, RequestStatus};
use crate::models::web_services::WebService;
use crate::network::scanner::banner::BannerScanResult;
use crate::network::scanner::port::PortScanResult;
use serde::Serialize;
use std::net::IpAddr;

#[derive(Serialize)]
pub struct Connection {
    #[serde(rename = "ip")]
    pub target: String,
    #[serde(default)]
    pub port: u16,

    pub connection_status: ConnectionStatus,
    pub request_status: RequestStatus,

    pub server: Option<String>,
    pub web_service: Option<WebService>,
    pub banner: Option<String>,
    pub latency_ms: Option<u128>,
    pub cve: Option<Vec<CveEntry>>,
}

impl Connection {
    pub fn from_results(
        target: &IpAddr,
        port: u16,
        port_scan: Option<PortScanResult>,
        banner_scan: Option<BannerScanResult>,
        cve: Option<Vec<CveEntry>>,
    ) -> Self {
        let (banner, server, web_service) = banner_scan
            .map(|b| (Some(b.banner), Some(b.server), Some(b.web_service)))
            .unwrap_or((None, None, None));

        let (connection_status, latency_ms) = port_scan
            .map(|p| (p.status, p.latency_ms))
            .unwrap_or((ConnectionStatus::CLOSED, None));

        let request_status = match connection_status {
            ConnectionStatus::OPEN => RequestStatus::SUCCESS,
            _ => RequestStatus::FAILED,
        };

        Connection {
            target: target.to_string(),
            port,
            connection_status,
            latency_ms,
            banner,
            server,
            web_service,
            request_status,
            cve,
        }
    }
}
