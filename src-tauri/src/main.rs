// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serial2::SerialPort;
use std::path::PathBuf;
use std::thread;
use std::str;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
struct SensorData {
    mq4: bool,
    mq7: bool,
    mq135: bool,
    latitude: f64,
    longitude: f64,
    altitude: f64,
    direction: f64,
    speed: f64,
    bmp_temperature: f64,
    pressure: u32,
    dht_temperature: f64,
    humidity: f64,
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn convert_sensor_data_to_json(input: &str) -> Result<Value, serde_json::Error> {
    let mut sensor_data: SensorData = SensorData {
        mq4: false,
        mq7: false,
        mq135: false,
        latitude: 0.0,
        longitude: 0.0,
        altitude: 0.0,
        direction: 0.0,
        speed: 0.0,
        bmp_temperature: 0.0,
        pressure: 0,
        dht_temperature: 0.0,
        humidity: 0.0,
        w: 2.0,
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };

    for entry in input.trim().split(';') {
        let parts: Vec<&str> = entry.split(':').collect();
        if parts.len() != 2 {
            continue;
        }
        match parts[0] {
            "MQ4" => sensor_data.mq4 = parts[1].parse().unwrap_or(false),
            "MQ7" => sensor_data.mq7 = parts[1].parse().unwrap_or(false),
            "MQ135" => sensor_data.mq135 = parts[1].parse().unwrap_or(false),
            "LAT" => sensor_data.latitude = parts[1].parse().unwrap_or(0.0),
            "LON" => sensor_data.longitude = parts[1].parse().unwrap_or(0.0),
            "ALT" => sensor_data.altitude = parts[1].parse().unwrap_or(0.0),
            "DIR" => sensor_data.direction = parts[1].parse().unwrap_or(0.0),
            "SPD" => sensor_data.speed = parts[1].parse().unwrap_or(0.0),
            "B-T" => sensor_data.bmp_temperature = parts[1].parse().unwrap_or(0.0),
            "P" => sensor_data.pressure = parts[1].parse().unwrap_or(0),
            "D-T" => sensor_data.dht_temperature = parts[1].parse().unwrap_or(0.0),
            "H" => sensor_data.humidity = parts[1].parse().unwrap_or(0.0),
            "W" => sensor_data.w = parts[1].parse().unwrap_or(2.0),
            "X" => sensor_data.x = parts[1].parse().unwrap_or(2.0),
            "Y" => sensor_data.y = parts[1].parse().unwrap_or(2.0),
            "Z" => sensor_data.z = parts[1].parse().unwrap_or(2.0),
            _ => (),
        }
    }

    let json_data = json!(sensor_data);
    Ok(json_data)
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
        .on_page_load(|window, _| {
            println!("Available Ports: {}", get_available_ports());
            let window_: tauri::Window = window.clone();
            let window__: tauri::Window = window.clone();

            let window = Arc::new(window);
            let window_clone = Arc::clone(&window);

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
                        let sensor_data: String = str::from_utf8(&buffer).unwrap().trim_matches(char::from(0)).to_string();
                        buffer = [0; 192];
                        let json_sensor_data = convert_sensor_data_to_json(&sensor_data).unwrap();
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
