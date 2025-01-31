extern crate rand;

use rand::{rngs::ThreadRng, Rng};
use rand_distr::{Distribution, Normal, LogNormal, Gamma, Beta, Exp};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::{Duration, SystemTime};
use chrono::prelude::*;
use tokio::time;
use crate::sensor::{Anomaly, Pattern, Sensor, SensorConfig};

pub fn generate_data(sensor: &Sensor, rng: &mut ThreadRng, anomaly: &Anomaly) -> f32 {
    let value = match sensor.distribution.as_str() {
        "uniform" => rng.gen_range(sensor.min_value.unwrap()..=sensor.max_value.unwrap()),
        "normal" => {
            let normal = Normal::new(sensor.mean.unwrap(),
                sensor.std_dev.unwrap()).expect("Failed to make Normal Distribution");
            normal.sample(rng)
        },
        "lognormal" => {
            let lognormal = LogNormal::new(sensor.mean.unwrap(),
                 sensor.std_dev.unwrap()).expect("Failed to make LogNormal Distribution");
            lognormal.sample(rng)
        }
        "exponential" => {
            let exp = Exp::new(sensor.mean.unwrap())
                .expect("Failed to make Exponential Distribution");
            exp.sample(rng)
        }
        "beta" => {
            let beta = Beta::new(sensor.alpha.unwrap(), 
                sensor.beta.unwrap()).expect("Failed to make Beta distribution");
            beta.sample(rng)
        }
        "gamma" => {
            let gamma = Gamma::new(sensor.alpha.unwrap(),
                sensor.beta.unwrap()).expect("Failed to make Gamma distribution");
            gamma.sample(rng)
        }
        _ => panic!("Unexpected Failure"),
    };

    if rng.gen_bool(anomaly.prob) & anomaly.generate == true {
        let min = -1.0 * anomaly.margin;
        let anomaly_offset = rng.gen_range(min..=anomaly.margin);
        println!("Generated sensor value: {}", value + anomaly_offset);
        return value + anomaly_offset as f32;
    }

    println!("Generated sensor value: {}", value);

    return value;
}

pub async fn stream_sensor_data(sensor: &Sensor, producer: &FutureProducer, anomaly: &Anomaly, pattern: &Pattern, config: &SensorConfig) {
    let mut rng = rand::thread_rng();

    loop {        

        let start = SystemTime::now();
        let now = Local::now();

        let value = generate_data(sensor, &mut rng, anomaly);

        let diurnal_adj = match pattern.interval.as_str() {
            "day" => {
                let int = now.day();
                pattern.amplitude * f32::sin(2.0 * std::f32::consts::PI * int as f32 / pattern.repeat)
            },
            "hour" => {
                let int = now.hour();
                pattern.amplitude * f32::sin(2.0 * std::f32::consts::PI * int as f32 / pattern.repeat)
            },
            "minute" => {
                let int = now.minute();
                pattern.amplitude * f32::sin(2.0 * std::f32::consts::PI * int as f32 / pattern.repeat)
            },
            "second" => {
                let int = now.second();
                pattern.amplitude * f32::sin(2.0 * std::f32::consts::PI * int as f32 / pattern.repeat)
            },
            _ => panic!("Unidentified interval")
        };

        let final_value = value + diurnal_adj;

        let now = Local::now();
        let datetime = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:09}", now.year(), now.month(),
             now.day(), now.hour(), now.minute(), now.nanosecond());

        let end = SystemTime::now();
        let latency = end.duration_since(start)
            .expect("Invalid time");

        let record_str: String = format!("{}\t{}\t{:.6}\t{}\t{}\t{:04}",
             sensor.id, datetime, &final_value, sensor.unit, sensor.r#type, latency.as_micros());
        
        let record = FutureRecord::to(&config.kafka.topic)
            .payload(&record_str)
            .key("temp_data");

        producer.send(record, Duration::from_secs(0)).await.unwrap();

        time::sleep(Duration::from_millis(sensor.interval_ms)).await;
    }
}