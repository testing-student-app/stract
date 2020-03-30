#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
mod command;
mod shell;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::io::{BufRead, BufReader};
use std::process::Command;

use stract_admin::{shell, ServerReply};
use tauri::Handle;

#[derive(Serialize)]
pub struct State {
    pub name: String,
    pub payload: String,
}

fn main() {
    let mut setup = false;
    let mut first_start = true;
    tauri::AppBuilder::new()
        .setup(move |webview, _| {
            if !setup {
                setup = true;
                let handle = webview.handle();

                let reload_handle = webview.handle();
                tauri::event::listen("reload".to_string(), move |port: u16| {
                    first_start = false;
                    let reload_handle_clone = reload_handle.clone();
                    std::thread::spawn(move || {
                        let ten_millis = std::time::Duration::from_millis(100);
                        std::thread::sleep(ten_millis);
                        spawn_go_server(&handle, port + 1);
                    });
                });

                if first_start {
                    spawn_go_server(&handle);
                }
            }
        })
        .build()
        .run();
}

fn notify_state<T: 'static>(handle: &Handle<T>, name: String) {
    notify_state_with_payload(handle, name, String::from(""))
}

fn notify_state_with_payload<T: 'static, P>(handle: &Handle<T>, name: String, payload: P) {
    let reply = State { name, payload };

    tauri::event::emit(
        handle,
        String::from("state"),
        serde_json::to_string(&reply).unwrap(),
    );
}

fn spawn_go_server<T: 'static>(handle: &Handle<T>, port: u16) {
    notify_state_with_payload(&handle, String::from("server_loaded"), false);
    let target_exe = env::current_exe().unwrap();
    let target_dir = target_exe.parent().unwrap();
    let stdout = command::spawn_command(
        orchestrator_command(),
        Some(target_dir),
        vec!["-addr", format!(":{}", port)],
    )
    .expect("Failed to start go server")
    .stdout
    .expect("Failed to get go server stdout");
    let reader = BufReader::new(stdout);

    let mut webview_started = false;
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            println!("{}", line);
            if !line.contains("address already in use") && !webview_started {
                webview_started = true;
                notify_state_with_payload(&handle, String::from("server_port"), port);
                notify_state_with_payload(&handle, String::from("server_loaded"), true);
            } else {
                notify_state_with_payload(&handle, String::from("server_loaded"), false);
                startup_eval(&handle, port);
            }
        });
}

fn startup_eval<T: 'static>(handle: &Handle<T>, old_port: u16) {
    handle
        .dispatch(|webview| {
            webview
                .eval(format!(
                    "
      window.__STRACT_RELOAD = function () {{
        window.tauri.emit('reload', {})
        window.location.reload()
      }}
    ",
                    old_port
                ))
                .expect("failed to eval location.replace rewrite");
            webview.eval(include_str!(concat!(env!("TAURI_DIR"), "/tauri.js")))
        })
        .expect("failed to eval tauri entry point");
}
