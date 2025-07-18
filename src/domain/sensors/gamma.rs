//! # GammaSensor – synthetic γ-dose-rate probe
//!
//! **What it imitates**  
//! • A Geiger-Müller tube or GM pancake measuring μSv / h.  
//! • Baseline = cosmic and terrestrial background.  
//! • `jitter` = counting statistics + electronics noise.  
//! • `spike_prob / spike_mul` = sealed source moved nearby, or a solar
//!   particle event.
//!
//! Unit: **mSv h⁻¹** (modify when you need μSv h⁻¹ – just scale the baseline).

use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use crate::domain::sensors::Sensor;

pub struct GammaSensor {
    id: String,
    baseline: f64,
    jitter: f64,
    spike_prob: f64,
    spike_mul: f64,
    rng: SmallRng,
}

impl GammaSensor {
    pub fn from_cfg(cfg: &crate::infra::config::SensorCfg) -> Self {
        Self {
            id: cfg.id.clone(),
            baseline: cfg.baseline_msv,
            jitter: cfg.jitter_msv,
            spike_prob: cfg.spike_prob,
            spike_mul: cfg.spike_mul,
            rng: SmallRng::from_entropy(),
        }
    }
}

impl Sensor for GammaSensor {
    fn tick(&mut self) -> f64 {
        let normal = Normal::new(0.0, self.jitter).unwrap();
        let mut val = self.baseline + normal.sample(&mut self.rng);
        if self.rng.gen_bool(self.spike_prob) {
            val *= self.spike_mul;
        }
        val
    }

    fn name(&self) -> &str {
        &self.id
    }
}
