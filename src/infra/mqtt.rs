//! # MQTT wrapper
//!
//! A stupid-thin façade over *rumqttc* so the domain layer never imports
//! networking crates.  Two primitives only:
//! • `connect(url, client_id)`  → `AsyncClient`  
//! • `publish_json(&client, topic, &impl Serialize)` – QoS 1 JSON message.


use std::time::Duration;
use anyhow::Result;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use serde::Serialize;         
use tokio::task;

pub async fn connect(url: &str, client_id: &str) -> Result<AsyncClient> {
    let (host, port) = url
        .trim_start_matches("tcp://")
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("MQTT_URL format tcp://host:port"))?;
    let port: u16 = port.parse()?;

    let mut opts = MqttOptions::new(client_id, host, port);
    opts.set_keep_alive(Duration::from_secs(5));

    let (client, mut evloop): (AsyncClient, EventLoop) = AsyncClient::new(opts, 10);

    task::spawn(async move {
        loop {
            if let Err(e) = evloop.poll().await {
                eprintln!("MQTT eventloop error: {e}");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    });

    Ok(client)
}

pub async fn publish_json<T: Serialize>(
    client: &AsyncClient,
    topic: &str,
    value: &T,
) -> Result<()> {
    let payload = serde_json::to_vec(value)?;
    client
        .publish(topic, QoS::AtLeastOnce, false, payload)
        .await?;
    Ok(())
}
