use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct SensorConfig {
    pub sensor: Sensor,
    pub anomaly: Anomaly,
    pub pattern: Pattern,
    pub kafka: Kafka,
}

#[derive(Debug, Deserialize)]
pub struct Sensor {
    pub id: String,
    pub r#type: String,
    pub unit: String,
    pub distribution: String,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub mean: Option<f32>,
    pub std_dev: Option<f32>,
    pub alpha: Option<f32>,
    pub beta: Option<f32>,
    pub interval_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct Kafka {
    pub brokers: String,
    pub topic: String,
}

#[derive(Debug, Deserialize)]
pub struct Anomaly {
    pub generate: bool,
    pub margin: f32,
    pub prob: f64
}

#[derive(Debug, Deserialize)]
pub struct Pattern {
    pub generate: bool,
    pub interval: String,
    pub amplitude: f32,
    pub repeat: f32
}


pub fn load_config() -> Option<SensorConfig> {
    let path = "src/config.yaml";

    let config_content = fs::read_to_string(&path).expect("Failed to read file");

    let sensor_config =serde_yaml::from_str(&config_content).expect("Failed to parse yaml file");

    sensor_config
}