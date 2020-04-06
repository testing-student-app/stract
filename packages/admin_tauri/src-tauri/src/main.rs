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
use tauri::Handle;

#[derive(Serialize)]
pub struct State {
    pub name: String,
    pub payload: String,
}

fn main() {
    let mut setup = false;
    tauri::AppBuilder::new()
        .splashscreen_html("
            <body>
                <div class='text-center'>
                    <div class='spinner-border'></div>
                </div>
                <style>
                    body {
                      height: 100vh;
                    }
                    .text-center {
                      height: 100vh;
                      text-align: center !important;
                      display: flex;
                      justify-content: center;
                      align-items: center;
                    }
                    @-webkit-keyframes spinner-border {
                      to {
                        transform: rotate(360deg);
                      }
                    }
                    @keyframes spinner-border {
                      to {
                        transform: rotate(360deg);
                      }
                    }
                    .spinner-border {
                      display: inline-block;
                      width: 2rem;
                      height: 2rem;
                      vertical-align: text-bottom;
                      border: 0.25em solid currentColor;
                      border-right-color: transparent;
                      border-radius: 50%;
                      -webkit-animation: spinner-border 0.75s linear infinite;
                      animation: spinner-border 0.75s linear infinite;
                    }
                </style>
            </body>
        ")
        .setup(move |webview, _| {
            if !setup {
                setup = true;

                let handle = webview.handle();

                if cfg!(debug_assertions) { inject_tauri(&handle); }

                let reload_handle = webview.handle();
                tauri::event::listen("reload".to_string(), move |port| {
                    let reload_handle_clone = reload_handle.clone();
                    std::thread::spawn(move || {
                        let ten_millis = std::time::Duration::from_millis(100);
                        std::thread::sleep(ten_millis);
                        spawn_go_server(
                            &reload_handle_clone,
                            port.unwrap().parse::<u16>().unwrap() + 1,
                        );
                    });
                });

                spawn_go_server(&handle, 8081);
            }
        })
        .build()
        .run();
}

fn notify_state<T: 'static>(handle: &Handle<T>, name: String) {
    notify_state_with_payload(handle, name, String::from(""))
}

fn notify_state_with_payload<T: 'static>(handle: &Handle<T>, name: String, payload: String) {
    let reply = State { name, payload };

    tauri::event::emit(
        handle,
        String::from("state"),
        Option::from(serde_json::to_string(&reply).unwrap()),
    );
}

fn spawn_go_server<T: 'static>(handle: &Handle<T>, port: u16) {
    notify_state_with_payload(&handle, String::from("server_loaded"), false.to_string());
    let target_exe = env::current_exe().unwrap();
    let target_dir = target_exe.parent().unwrap();
    let process = command::spawn_command(
        shell::go_server_execname(),
        target_dir,
        vec!["-addr", &format!(":{}", port)],
    );

    let mut webview_started = false;

    let pid = shell::pidof("go-server");
    if process.is_ok() && pid.is_ok() && !webview_started {
        webview_started = true;
        tauri::close_splashscreen(&handle).unwrap();
        notify_state_with_payload(&handle, String::from("server_port"), port.to_string());
        notify_state_with_payload(&handle, String::from("server_loaded"), true.to_string());
    } else {
        notify_state_with_payload(&handle, String::from("server_loaded"), false.to_string());
        startup_eval(&handle, port);
    }
}

fn startup_eval<T: 'static>(handle: &Handle<T>, old_port: u16) {
    handle
        .dispatch(move |webview| {
            webview.eval(&format!(
                "
      window.__STRACT_RELOAD = function () {{
        window.tauri.emit('reload', {})
        window.location.reload()
      }}
    ",
                old_port
            ))
        })
        .expect("failed to inject reload");
}

fn inject_tauri<T: 'static>(handle: &Handle<T>) {
    handle
        .dispatch(move |webview| {
            webview.eval(include_str!(concat!(env!("TAURI_DIR"), "/tauri.js")))
        })
        .expect("failed to inject tauri.js");
}
