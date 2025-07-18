//! # UVSensor – space UV flux monitor
//!
//! **Typical hardware**  SiC photodiode on a cubesat, or UV-A/B index
//! module on a weather station.
//!
//! • `base_uv` is either the UV-Index (0-11+) **or** radiometric power
//!   μW cm⁻² – you decide in config.  
//! • Noise dominated by photon shot noise ⇒ Gaussian model works well.
//!
//! Good test ranges: index 6 ≈ strong summer sun, index 10+ ≈ tropical noon.


use crate::domain::noise::Noise;     // krótsza ścieżka po re-eksporcie
use super::Sensor;

pub struct UVSensor<N: Noise> {
    label:   String,
    base_uv: f64,
    noise:   N,
}

impl<N: Noise> UVSensor<N> {
    pub fn new(label: impl Into<String>, base_uv: f64, noise: N) -> Self {
        Self { label: label.into(), base_uv, noise }
    }
}

impl<N: Noise> Sensor for UVSensor<N> {
    fn tick(&mut self) -> f64 {
        self.noise.generate(self.base_uv)
    }
    fn name(&self) -> &str { &self.label }
}
