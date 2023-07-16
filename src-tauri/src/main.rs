// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sat_data;

use serial2::SerialPort;
use tauri::AppHandle;
use tauri::Manager;
use std::path::PathBuf;
use std::thread;
use std::str;
use serde_json::Value;
use serde::Serialize;
use tauri::Window;
use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::sat_data::SatelliteData;
use crate::sat_data::height_from_pressure;
use crate::sat_data::json_from_satellite_data;
use crate::sat_data::satellite_data_from_serial;

#[derive(Clone, Serialize)]
struct Telemetry {
    timestamp: String,
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

#[tauri::command]
fn updateports(app: AppHandle) {
    let reply: Message = Message {
        message: get_available_ports(),
    };

    let window: Window = app.get_window("main").expect("failed to get main window");

    window
        .emit("available-ports", reply)
        .expect("failed to emit");
}

fn remove_char(string: String, remove: char) -> String {
    let mut s: String = string;
    s.retain(|c| c != remove);
    s
}

fn send_error(window: &Window, error: String) {
    window
        .emit("error", Message {message: error})
        .expect("failed to emit");
}

#[tauri::command]
fn connect(app: AppHandle, port: String) {
    let window: Window = app.get_window("main").expect("failed to get main window");
    let port: String = port.replace("\"", "");
    let serial_port: SerialPort;
    if let Ok(sp) = SerialPort::open(&port, 115200) {
        serial_port = sp;
    } else {
        send_error(&window, format!("Error de conexión a {}.", &port));
        return;
    }

    thread::spawn(move || {
        let start_log_utc: &str = &Utc::now().to_rfc3339();
        let mut buffer: [u8; 192] = [0; 192];
        let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(window.app_handle().path_resolver().app_local_data_dir().unwrap().join(remove_char(format!("{}.log", start_log_utc), ':')))
                .unwrap();

        loop {
            if let Ok(size) = serial_port.read(&mut buffer) {
                if size == 0 {
                    continue;
                }
            } else {
                send_error(&window, "Se desconectó el puerto serial.".to_string());
                return;
            }

            println!("{:?}", str::from_utf8(&buffer));

            // Get a String with all the '\0's removed (the buffer is all '\0' initially)
            let serial_data: String = match str::from_utf8(&buffer) {
                Ok(x) => x.trim_matches(char::from(0)).to_string(),
                _ => continue,
            };
            buffer = [0; 192];

            let sat_data: SatelliteData = satellite_data_from_serial(&serial_data);
            let height_by_pressure: f32 = height_from_pressure(sat_data.pressure);
            let json_sat_data: Value = json_from_satellite_data(sat_data);
            let now_utc: &str = &Utc::now().to_rfc3339();

            let telemetry: Telemetry = Telemetry {
                timestamp: now_utc.to_string(),
                sat_data: json_sat_data.to_string(),
                height_p: height_by_pressure,
            };

            if let Err(e) = writeln!(file, "{}", serde_json::to_string(&telemetry).unwrap()) {
                send_error(&window, e.to_string());
            }

            window
                .emit("rx", telemetry)
                .expect("failed to emit");
        }
    });
}

fn main() {
    tauri::Builder::default()
        .on_page_load(|_, _| {
            println!("Available Ports: {}", get_available_ports());
        })
        .invoke_handler(tauri::generate_handler![updateports, connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
