//! # Engine – deterministic simulator driver
//!
//! On every call to `tick()` it:
//! 1. mutably borrows each `Sensor`,
//! 2. asks for a new reading via `Sensor::tick()`,  
//! 3. returns a flat `Vec<(name, value)>` ready for logging, MQTT, etc.
//!
//! The engine itself holds **no timebase** – the caller (binary or test)
//! decides whether “tick” means 1 ms or 15 minutes.


use crate::domain::sensors::Sensor;

pub struct Engine {
    sensors: Vec<Box<dyn Sensor>>,
}

impl Engine {
    pub fn new(sensors: Vec<Box<dyn Sensor>>) -> Self {
        Self { sensors }
    }

    pub fn tick(&mut self) -> Vec<(String, f64)> {
        self.sensors
            .iter_mut()
            .map(|s| (s.name().to_owned(), s.tick()))
            .collect()
    }
}
