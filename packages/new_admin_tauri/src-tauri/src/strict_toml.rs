use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct InternalTomlFile {
  pub path: String,
  pub contents: toml::Value,
}

pub fn create_with_nanoids(mut table: toml::Value) -> toml::Value {
  let data = table
    .as_table()
    .unwrap()
    .get(&"questions".to_string())
    .unwrap()
    .as_array()
    .unwrap();
  let new_data = data
    .iter()
    .map(|v: &toml::value::Value| {
      let id = nanoid!();
      let mut v_cloned = v.clone();
      v_cloned
        .as_table_mut()
        .unwrap()
        .insert("id".to_string(), toml::Value::String(id));
      v_cloned.as_table().unwrap().clone()
    })
    .collect::<Vec<toml::value::Table>>();
  let new_table = table.as_table_mut().unwrap();
  new_table.remove(&"questions".to_string());
  new_table.insert("questions".to_string(), toml::Value::from(new_data));
  toml::Value::from(new_table.clone())
}

pub fn remove_nanoids(mut table: toml::Value) -> toml::Value {
  let data = table
    .as_table()
    .unwrap()
    .get(&"questions".to_string())
    .unwrap()
    .as_array()
    .unwrap();
  let new_data = data
    .iter()
    .map(|v: &toml::value::Value| {
      let mut v_cloned = v.clone();
      v_cloned.as_table_mut().unwrap().remove("id");
      v_cloned.as_table().unwrap().clone()
    })
    .collect::<Vec<toml::value::Table>>();
  let new_table = table.as_table_mut().unwrap();
  new_table.remove(&"questions".to_string());
  new_table.insert("questions".to_string(), toml::Value::from(new_data));
  toml::Value::from(new_table.clone())
}
