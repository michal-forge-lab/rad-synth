// infra/config.rs
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub mqtt_url: String,
    pub sensor: SensorCfg,
    pub interval_sec: u64,
}

#[derive(Deserialize, Debug)]
pub struct SensorCfg {
    pub id: String,
    pub baseline_msv: f64,
    pub jitter_msv:   f64,
    pub spike_prob:   f64,
    pub spike_mul:    f64,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
            .map_err(Into::into)
    }
}
