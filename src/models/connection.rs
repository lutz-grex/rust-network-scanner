use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Connection {
    #[serde(rename = "ip")]              // JSON-Feld heißt "ip" statt "target"
    pub target: String,

    #[serde(default)]                   // Wenn Port mal nicht angegeben ist, Default = 0
    pub port: u16,

    pub server: String,

    pub web_service: WebService,

    pub banner: String,

    pub status: ConnectionStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u128>,            // Antwortzeit in ms

    pub request_status: RequestStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve: Option<Vec<CveEntry>>
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Default)]
pub enum ConnectionStatus {
    OPEN,
    #[default]
    CLOSED,
    TIMEOUT
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub enum RequestStatus {
    SUCCESS,
    FAILED
}

#[derive(Serialize)]
pub struct CveEntry {
    pub title: String,
    pub score: f32,
    pub severity: String,
    pub description: String,
    pub href: String,
}

#[derive(Serialize, Default, Debug, Clone)]
pub enum WebService {
    #[default]
    NONE,
    HTTP,
    SSH, 
    FTP,
    SMTP,
    REDIS,
    IMAP,
    POP3,
}