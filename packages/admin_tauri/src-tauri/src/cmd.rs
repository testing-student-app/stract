use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  NewFile {},
  OpenFile { callback: String, error: String },
  Save { data: String },
  SaveAs { data: String },
}
