use std::time::Duration;

use rdkafka::{producer::FutureProducer, ClientConfig};
use tokio::time;

pub mod sensor;
pub mod simulate;
pub mod stream;

#[tokio::main]
async fn main() {
    println!("Loading the sensor config file...");

    let config = sensor::load_config().unwrap();

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &config.kafka.brokers)
        .create()
        .expect("Producer Creation Error!");

    simulate::stream_sensor_data(&config.sensor, &producer, &config.anomaly, &config.pattern, &config).await;
}
