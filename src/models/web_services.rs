use rocket::serde::Serialize;

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
