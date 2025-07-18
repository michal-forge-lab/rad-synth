use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

use sensor_sim::{
    domain::{
        engine::Engine,
        sensors::{
            gamma::GammaSensor,
            uv::UVSensor,
            neutron::NeutronSensor,
        },
        noise::{gaussian::GaussianNoise, impulse::ImpulseNoise},
    },
    infra::{config::Settings, mqtt::{connect, publish_json}},
};

#[tokio::main]
async fn main() -> Result<()> {
    // 1️⃣  Konfiguracja
    let settings = Settings::new()?;

    // 2️⃣  Inicjalizacja sensorów
    let mut sensors: Vec<Box<dyn sensor_sim::domain::sensors::Sensor>> = vec![
        Box::new(GammaSensor::from_cfg(&settings.sensor)),
        Box::new(UVSensor::new("uv", 6.0, GaussianNoise::new(0.4))),
        Box::new(NeutronSensor::new("neutron", 1_200.0,
                    ImpulseNoise { prob: 0.01, min_amp: 80.0, max_amp: 250.0 })),
    ];

    // 3️⃣  Uruchom silnik
    let mut engine = Engine::new(sensors);

    // 4️⃣  MQTT
    let client = connect(&settings.mqtt_url, "sensor_sim").await?;

    // 5️⃣  Pętla
    loop {
        for (name, value) in engine.tick() {
            println!("{:<10} → {:>8.4}  ({})", name, value, Utc::now());

            if let Err(e) = publish_json(
                &client,
                &format!("sensor/{}/reading", name),
                &json!({ "ts": Utc::now(), "value": value }),
            ).await {
                eprintln!("MQTT publish error: {e}");
            }
        }
        sleep(Duration::from_secs(settings.interval_sec)).await;
    }
}
