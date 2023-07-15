// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sat_data;
use sat_data::json_from_serial;

use serial2::SerialPort;
use std::path::PathBuf;
use std::thread;
use std::str;
use std::sync::Arc;
use serde_json::Value;
use serde::Serialize;
use tauri::Window;

#[derive(Clone, Serialize)]
struct Payload {
  message: String,
}

fn get_available_ports() -> String {
    let available_ports: Vec<PathBuf> = SerialPort::available_ports().unwrap();
    let string_ports: Vec<String> = available_ports
        .into_iter()
        .map(|path| path.to_string_lossy().into_owned())
        .collect();
    return string_ports.join(",");
}

fn main() {
    tauri::Builder::default()
        .on_page_load(|window: Window, _| {
            println!("Available Ports: {}", get_available_ports());
            let window_: Window = window.clone();
            let window__: Window = window.clone();

            let window: Arc<Window> = Arc::new(window);
            let window_clone: Arc<Window> = Arc::clone(&window);

            window.listen("connect", move |event| {
                let port: String = event.payload().unwrap().replace("\"", "");
                println!("Got connect with payload: {}", port);
                let port: SerialPort = SerialPort::open(port, 115200).unwrap();
                let mut buffer = [0; 192];

                let window = Arc::clone(&window_clone);
                thread::spawn(move || {
                    loop {
                        let size: usize = port.read(&mut buffer).unwrap();
                        if size == 0 {
                            continue;
                        }
                        println!("{:?}", str::from_utf8(&buffer));
                        let sensor_data: String = match str::from_utf8(&buffer) {
                            Ok(x) => x.trim_matches(char::from(0)).to_string(),
                            _ => continue,
                        };
                        buffer = [0; 192];
                        let json_sensor_data: Value = json_from_serial(&sensor_data);
                        let reply: Payload = Payload {
                            message: json_sensor_data.to_string(),
                        };

                        window
                            .emit("rx", Some(reply))
                            .expect("failed to emit");
                    }
                });
            });

            window_.listen("page-load", move |_| {
                println!("Got page-load");
                let reply: Payload = Payload {
                    message: get_available_ports(),
                };

                println!("Sending available-ports with payload: {}", reply.message);
                window__
                    .emit("available-ports", Some(reply))
                    .expect("failed to emit");
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
