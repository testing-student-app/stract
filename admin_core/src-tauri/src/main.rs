#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::env;

fn main() {
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

                            std::process::Command::new("./go-server")
                                .arg("-addr")
                                .arg(":8081")
                                .current_dir(target_dir)
                                .spawn()
                                .expect("failed to execute go-server");

                            Ok("{status: 'started'}".to_string())
                        },
                        callback,
                        error,
                    ),
                },
            }
        })
        .build()
        .run();
}
