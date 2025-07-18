use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RadiationReading {
    pub id: String,
    pub sensor_id: String,
    pub timestamp: DateTime<Utc>,
    pub value: f64,
}

impl RadiationReading {
    pub fn new(sensor_id: &str, value: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sensor_id: sensor_id.to_owned(),
            timestamp: Utc::now(),
            value,
        }
    }
}
