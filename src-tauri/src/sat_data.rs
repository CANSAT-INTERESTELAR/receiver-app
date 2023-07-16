use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
pub struct SatelliteData {
    pub mq4: bool,
    pub mq7: bool,
    pub mq135: bool,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub direction: f64,
    pub speed: f64,
    pub bmp_temperature: f64,
    pub pressure: u32,
    pub dht_temperature: f64,
    pub humidity: f64,
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

// Input pressure in Pascals
pub fn height_from_pressure(pressure: u32) -> f32 {
    return 44330.0*(1.0-(pressure as f32/101325.0).powf(1.0/5.255))
}