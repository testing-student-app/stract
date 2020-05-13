use tauri::Handle;

#[derive(Serialize)]
pub struct State {
  pub name: String,
  pub payload: String,
}

pub fn notify_state<T: 'static>(handle: &Handle<T>, name: String) {
  notify_state_with_payload(handle, name, String::from(""))
}

pub fn notify_state_with_payload<T: 'static>(handle: &Handle<T>, name: String, payload: String) {
  let reply = State { name, payload };

  tauri::event::emit(
    handle,
    String::from("state"),
    Option::from(serde_json::to_string(&reply).unwrap()),
  );
}
