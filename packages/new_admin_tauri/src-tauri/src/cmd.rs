use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerReply {
  pub good_ok: bool,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  NewFile {},
  OpenFile {
    callback: String,
    error: String,
  },
  Save {
    data: String,
  },
  SaveAs {
    data: String,
  },
  StartServer {
    callback: String,
    error: String,
    port: String,
  },
  CheckServer {
    callback: String,
    error: String,
    port: String,
  },
}
