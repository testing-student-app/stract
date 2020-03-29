use serde::Serialize;

mod shell;

#[derive(Serialize)]
pub struct ServerReply {
    pub status: String,
}
