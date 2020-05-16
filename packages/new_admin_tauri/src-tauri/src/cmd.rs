use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerReply {
  pub good_ok: bool,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  OpenFile {
    callback: String,
    error: String,
  },
  Save {
    callback: String,
    error: String,
    path: String,
    data: toml::value::Table,
  },
  SaveAs {
    callback: String,
    error: String,
    data: toml::value::Table,
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
