use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
pub struct SatelliteData {
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

pub fn satellite_data_from_serial(input: &str) -> SatelliteData {
    let mut sat_data: SatelliteData = SatelliteData {
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

    // For "LAT:0.000;LON:1.111"
    // Entry will be LAT:0.000
    // Parts[0] will be LAT
    // Parts[1] will be 0.000
    for entry in input.trim().split(';') {
        let parts: Vec<&str> = entry.split(':').collect();
        // Avoid parsing entries with multiple ":"
        if parts.len() != 2 {
            continue;
        }

        match parts[0] {
            "MQ4" => sat_data.mq4 = parts[1].parse().unwrap_or(false),
            "MQ7" => sat_data.mq7 = parts[1].parse().unwrap_or(false),
            "MQ135" => sat_data.mq135 = parts[1].parse().unwrap_or(false),
            "LAT" => sat_data.latitude = parts[1].parse().unwrap_or(0.0),
            "LON" => sat_data.longitude = parts[1].parse().unwrap_or(0.0),
            "ALT" => sat_data.altitude = parts[1].parse().unwrap_or(0.0),
            "DIR" => sat_data.direction = parts[1].parse().unwrap_or(0.0),
            "SPD" => sat_data.speed = parts[1].parse().unwrap_or(0.0),
            "B-T" => sat_data.bmp_temperature = parts[1].parse().unwrap_or(0.0),
            "P" => sat_data.pressure = parts[1].parse().unwrap_or(0),
            "D-T" => sat_data.dht_temperature = parts[1].parse().unwrap_or(0.0),
            "H" => sat_data.humidity = parts[1].parse().unwrap_or(0.0),
            "W" => sat_data.w = parts[1].parse().unwrap_or(2.0),
            "X" => sat_data.x = parts[1].parse().unwrap_or(2.0),
            "Y" => sat_data.y = parts[1].parse().unwrap_or(2.0),
            "Z" => sat_data.z = parts[1].parse().unwrap_or(2.0),
            _ => (),
        }
    }

    sat_data
}

pub fn json_from_satellite_data(sat_data: SatelliteData) -> Value {
    json!(sat_data)
}

pub fn json_from_serial(input: &str) -> Value {
    json_from_satellite_data(satellite_data_from_serial(input))
}