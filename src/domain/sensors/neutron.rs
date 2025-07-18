//! # NeutronSensor – high-energy neutron count-rate
//!
//! **Origin**   >90 % of high-altitude radiation dose is neutrons produced
//! when primary cosmic rays spallate in the atmosphere.
//!
//! • `base_cps` – counts s⁻¹ at cruise altitude (1 200 cps ≈ 10 µSv h⁻¹).  
//! • Impulse noise fits sporadic mini-showers or aircraft thunderstorms.
//!
//! Unit used here is **cps**; convert to µSv h⁻¹ offline if needed.


use crate::domain::noise::Noise;
use super::Sensor;

pub struct NeutronSensor<N: Noise> {
    label:    String,
    base_cps: f64,
    noise:    N,
}

impl<N: Noise> NeutronSensor<N> {
    pub fn new(label: impl Into<String>, base_cps: f64, noise: N) -> Self {
        Self { label: label.into(), base_cps, noise }
    }
}

impl<N: Noise> Sensor for NeutronSensor<N> {
    fn tick(&mut self) -> f64 {
        self.noise.generate(self.base_cps)
    }
    fn name(&self) -> &str { &self.label }
}
