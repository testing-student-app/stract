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
use std::fs;
use tauri::Handle;
use toml::Value;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => match command {
          NewFile {} => Ok(()),
          OpenFile { callback, error } => {
            tauri::execute_promise(
              _webview,
              move || {
                let response: tauri::api::dialog::Response =
                  if let Ok(response) = tauri::api::dialog::select(None, None) {
                    response
                  } else {
                    return Err("Canceled".into());
                  };

                let file_path: String = match response {
                  tauri::api::dialog::Response::Okay(file_path) => file_path,
                  tauri::api::dialog::Response::OkayMultiple(file_paths) => file_paths[0].clone(),
                  tauri::api::dialog::Response::Cancel => return Err("Canceled".into()),
                };

                let file =
                  fs::read_to_string(file_path).expect("Something went wrong reading the file");

                let data = file.parse::<Value>().unwrap();

                Ok(serde_json::to_string(&data).unwrap())
              },
              callback,
              error,
            );
            Ok(())
          }
          Save { data } => Ok(()),
          SaveAs { data } => Ok(()),
          StartServer {
            callback,
            error,
            port,
          } => {
            tauri::execute_promise(
              _webview,
              move || {
                let port = port.parse::<u16>().unwrap();
                let good_ok = spawn_go_server(port);
                let serverReply = cmd::ServerReply { good_ok };
                if !good_ok {
                  return Err("failed".into());
                }
                Ok(serde_json::to_string(&serverReply).unwrap())
              },
              callback,
              error,
            );
            Ok(())
          }
          CheckServer {
            callback,
            error,
            port,
          } => {
            tauri::execute_promise(
              _webview,
              move || {
                let port = port.parse::<u16>().unwrap();
                let good_ok = check_if_port_exist(&port);
                let serverReply = cmd::ServerReply { good_ok };
                if good_ok {
                  return Ok(serde_json::to_string(&serverReply).unwrap());
                }
                Err("failed".into())
              },
              callback,
              error,
            );
            Ok(())
          }
        },
      }
    })
    .build()
    .run();
}

fn spawn_go_server(port: u16) -> bool {
  let target_exe = env::current_exe().unwrap();
  let target_dir = target_exe.parent().unwrap();
  if let Err(_) = command::spawn_command(
    go_server_execname(),
    target_dir,
    vec!["-addr", &format!(":{}", port)],
  ) {
    return false;
  };

  let duration = std::time::Duration::from_millis(1000);
  std::thread::sleep(duration);
  // let is_running = check_if_port_exist(&port);

  true
}

#[cfg(target_os = "linux")]
fn go_server_execname() -> String {
  String::from("./go-server")
}

#[cfg(target_os = "windows")]
fn go_server_execname() -> String {
  String::from("go-server.exe")
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
