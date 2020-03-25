#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::env;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
struct ServerReply {
    status: String,
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            use cmd::Cmd::*;

            match serde_json::from_str(arg) {
                Err(_) => {}
                Ok(command) => match command {
                    LoadServer { callback, error } => tauri::execute_promise(
                        _webview,
                        move || {
                            let target_exe = env::current_exe()?;
                            let target_dir = target_exe.parent().unwrap();
                            let port: u16 = 8081;

                            std::process::Command::new("./go-server")
                                .arg("-addr")
                                .arg(format!(":{}", port))
                                .current_dir(target_dir)
                                .spawn()
                                .expect("failed to execute go-server");

                            let reply = ServerReply {
                                status: String::from("started"),
                                port,
                            };

                            Ok(serde_json::to_string(&reply).unwrap())
                        },
                        callback,
                        error,
                    ),
                },
            }
        })
        .build()
        .run();

    Ok(())
}
