#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
mod command;
mod notify;

#[macro_use]
extern crate serde_derive;
extern crate netstat;
extern crate serde_json;

use std::env;
use tauri::Handle;

fn main() {
    let mut setup = false;
    tauri::AppBuilder::new()
        .splashscreen_html(&SPLASHSCREEN_HTML)
        .setup(move |webview, _| {
            if !setup {
                setup = true;

                let handle = webview.handle();

                if cfg!(debug_assertions) {
                    inject_tauri(&handle);
                }

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
                tauri::close_splashscreen(&handle).unwrap();
            }
        })
        .build()
        .run();
}

fn check_if_port_exist(port: &u16) -> bool {
    let af_flags = netstat::AddressFamilyFlags::IPV4;
    let proto_flags = netstat::ProtocolFlags::TCP;
    let is_running = netstat::iterate_sockets_info(af_flags, proto_flags)
        .unwrap()
        .filter(
            |socket_info: &Result<netstat::SocketInfo, netstat::Error>| {
                let socket_info = socket_info.clone();
                return match socket_info.unwrap().protocol_socket_info {
                    netstat::ProtocolSocketInfo::Tcp(tcp_socket_info) => {
                        if tcp_socket_info.local_port == *port {
                            return true;
                        }
                        // ignore everything else
                        false
                    }
                    netstat::ProtocolSocketInfo::Udp(_) => {
                        // we dont care about udp
                        false
                    }
                };
            },
        )
        .collect::<Vec<Result<netstat::SocketInfo, netstat::Error>>>();
    let is_running = if is_running.len() != 0 { true } else { false };
    is_running
}

#[cfg(target_os = "linux")]
fn go_server_execname() -> String {
    String::from("./go-server")
}

#[cfg(target_os = "windows")]
fn go_server_execname() -> String {
    String::from("go-server.exe")
}

fn notify_server<T: 'static>(handle: &Handle<T>, port: u16, is_loaded: bool) {
    notify::notify_state_with_payload(&handle, String::from("server_port"), port.to_string());
    notify::notify_state_with_payload(
        &handle,
        String::from("server_loaded"),
        is_loaded.to_string(),
    );
}

fn spawn_go_server<T: 'static>(handle: &Handle<T>, port: u16) {
    notify_server(&handle, port, false);

    let target_exe = env::current_exe().unwrap();
    let target_dir = target_exe.parent().unwrap();
    command::spawn_command(
        go_server_execname(),
        target_dir,
        vec!["-addr", &format!(":{}", port)],
    )
    .expect("Failed to start guijs server");

    let duration = std::time::Duration::from_millis(1000);
    std::thread::sleep(duration);
    let is_running = check_if_port_exist(&port);
    let mut webview_started = false;

    if is_running && !webview_started {
        webview_started = true;
        notify_server(&handle, port, true);
    } else {
        notify_server(&handle, port, false);
        startup_eval(&handle, port);
    }
}

fn startup_eval<T: 'static>(handle: &Handle<T>, old_port: u16) {
    handle
        .dispatch(move |webview| {
            webview.eval(&format!("window.tauri.emit('reload', {})", old_port))
        })
        .expect("failed to inject reload");
    handle
        .dispatch(move |webview| webview.eval("window.location.reload()"))
        .expect("failed to inject reload");
}

fn inject_tauri<T: 'static>(handle: &Handle<T>) {
    handle
        .dispatch(move |webview| {
            webview.eval(include_str!(concat!(env!("TAURI_DIR"), "/tauri.js")))
        })
        .expect("failed to inject tauri.js");
}

const SPLASHSCREEN_HTML: &str = "
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
";
