// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sat_data;

use serial2::SerialPort;
use std::path::PathBuf;
use std::thread;
use std::str;
use std::sync::Arc;
use serde_json::Value;
use serde::Serialize;
use tauri::Window;

use crate::sat_data::SatelliteData;
use crate::sat_data::height_from_pressure;
use crate::sat_data::json_from_satellite_data;
use crate::sat_data::satellite_data_from_serial;

#[derive(Clone, Serialize)]
struct Telemetry {
  sat_data: String,
  height_p: f32,
}

#[derive(Clone, Serialize)]
struct Message {
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
                let serial_port: SerialPort;
                if let Ok(sp) = SerialPort::open(&port, 115200) {
                    serial_port = sp;
                } else {
                    window_clone
                        .emit("error", Message {message: format!("Fue imposible conectarse a {}", &port)})
                        .expect("failed to emit");
                    return;
                }

                let mut buffer = [0; 192];

                let window = Arc::clone(&window_clone);
                thread::spawn(move || {
                    loop {
                        let size: usize = serial_port.read(&mut buffer).unwrap();
                        if size == 0 {
                            continue;
                        }
                        println!("{:?}", str::from_utf8(&buffer));
                        let serial_data: String = match str::from_utf8(&buffer) {
                            Ok(x) => x.trim_matches(char::from(0)).to_string(),
                            _ => continue,
                        };
                        buffer = [0; 192];

                        let sat_data: SatelliteData = satellite_data_from_serial(&serial_data);
                        let height_by_pressure: f32 = height_from_pressure(sat_data.pressure);
                        let json_sat_data: Value = json_from_satellite_data(sat_data);
                        let reply: Telemetry = Telemetry {
                            sat_data: json_sat_data.to_string(),
                            height_p: height_by_pressure,
                        };

                        window
                            .emit("rx", reply)
                            .expect("failed to emit");
                    }
                });
            });

            window_.listen("page-load", move |_| {
                println!("Got page-load");
                let reply: Message = Message {
                    message: get_available_ports(),
                };

                println!("Sending available-ports with payload: {}", reply.message);
                window__
                    .emit("available-ports", reply)
                    .expect("failed to emit");
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
