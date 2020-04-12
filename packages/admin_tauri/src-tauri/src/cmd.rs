use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    NewFile { callback: String, error: String },
    OpenFile,
    Save { data: String },
    SaveAs { data: String },
}
