use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Default)]
pub enum ConnectionStatus {
    OPEN,
    #[default]
    CLOSED,
    TIMEOUT,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub enum RequestStatus {
    SUCCESS,
    FAILED,
}
