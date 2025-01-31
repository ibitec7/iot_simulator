use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub async fn send_to_kafka(producer: &FutureProducer, topic: &str, value: &f32) {
    let value_str = value.to_string();
    let record = FutureRecord::to(topic)
        .payload(&value_str)
        .key("Sensor");

    producer.send(record, Duration::from_secs(0)).await.unwrap();
}