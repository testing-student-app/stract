#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod command;
mod notify;
mod strict_toml;

#[macro_use]
extern crate serde_derive;
extern crate netstat;
extern crate serde_json;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml::Value;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => match command {
          OpenFile { callback, error } => {
            tauri::execute_promise(
              _webview,
              move || {
                let response: tauri::api::dialog::Response = if let Ok(response) =
                  tauri::api::dialog::select(Some("toml".to_string()), None)
                {
                  response
                } else {
                  return Err("Canceled".into());
                };

                let file_path: String = match response {
                  tauri::api::dialog::Response::Okay(file_path) => file_path,
                  tauri::api::dialog::Response::OkayMultiple(file_paths) => file_paths[0].clone(),
                  tauri::api::dialog::Response::Cancel => return Err("Canceled".into()),
                };

                let file = fs::read_to_string(file_path.clone())
                  .expect("Something went wrong reading the file");

                let data = file.parse::<Value>().unwrap();

                let internal_toml_file = strict_toml::InternalTomlFile {
                  path: file_path.clone(),
                  contents: strict_toml::create_with_nanoids(data),
                };

                Ok(serde_json::to_string(&internal_toml_file).unwrap())
              },
              callback,
              error,
            );
            Ok(())
          }
          Save {
            callback,
            error,
            path,
            data,
          } => {
            tauri::execute_promise(
              _webview,
              move || {
                let os_path = Path::new(&path);
                let display = os_path.display();

                let mut file = match File::create(&os_path) {
                  Err(why) => return Err("Could not create file!".into()),
                  Ok(file) => file,
                };

                match file.write_all(
                  toml::ser::to_string_pretty(&strict_toml::remove_nanoids(toml::Value::from(
                    data.clone(),
                  )))
                  .unwrap()
                  .as_bytes(),
                ) {
                  Err(why) => {
                    return Err(
                      format!(
                        "couldn't write to {}: {}",
                        display,
                        why.raw_os_error().unwrap()
                      )
                      .into(),
                    )
                  }
                  Ok(_) => println!("successfully wrote to {}", display),
                };

                let internal_toml_file = strict_toml::InternalTomlFile {
                  path,
                  contents: toml::Value::from(data),
                };

                Ok(serde_json::to_string(&internal_toml_file).unwrap())
              },
              callback,
              error,
            );
            Ok(())
          }
          SaveAs {
            callback,
            error,
            data,
          } => {
            tauri::execute_promise(
              _webview,
              move || {
                let response: tauri::api::dialog::Response = if let Ok(response) =
                  tauri::api::dialog::save_file(Some("toml".to_string()), None)
                {
                  response
                } else {
                  return Err("Canceled".into());
                };

                let file_path: String = match response {
                  tauri::api::dialog::Response::Okay(file_path) => file_path,
                  tauri::api::dialog::Response::OkayMultiple(file_paths) => file_paths[0].clone(),
                  tauri::api::dialog::Response::Cancel => return Err("Canceled".into()),
                };

                let os_path = Path::new(&file_path);
                let display = os_path.display();

                let mut file = match File::create(&os_path) {
                  Err(why) => return Err("Could not create file!".into()),
                  Ok(file) => file,
                };

                match file.write_all(
                  toml::ser::to_string_pretty(&strict_toml::remove_nanoids(toml::Value::from(
                    data.clone(),
                  )))
                  .unwrap()
                  .as_bytes(),
                ) {
                  Err(why) => {
                    return Err(
                      format!(
                        "couldn't write to {}: {}",
                        display,
                        why.raw_os_error().unwrap()
                      )
                      .into(),
                    )
                  }
                  Ok(_) => println!("successfully wrote to {}", display),
                };

                let internal_toml_file = strict_toml::InternalTomlFile {
                  path: file_path.clone(),
                  contents: toml::Value::from(data),
                };

                Ok(serde_json::to_string(&internal_toml_file).unwrap())
              },
              callback,
              error,
            );
            Ok(())
          }
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
                let server_reply = cmd::ServerReply { good_ok };
                if !good_ok {
                  return Err("failed".into());
                }
                Ok(serde_json::to_string(&server_reply).unwrap())
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
                let server_reply = cmd::ServerReply { good_ok };
                if good_ok {
                  return Ok(serde_json::to_string(&server_reply).unwrap());
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
