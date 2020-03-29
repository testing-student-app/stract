use serde::Serialize;

pub mod shell;

#[derive(Serialize)]
pub struct ServerReply {
    pub status: String,
}
