//! # DriftNoise – ageing and slow environmental drift
//!
//! **What happens in reality?**
//! • A silicon sensor accumulates crystal defects – its efficiency drifts
//!   slowly over weeks or months.  
//! • A photomultiplier’s gain follows temperature.  
//! • Bias HV supplies sag a few ppm / hour.
//!
//! **Model**  
//! We add a small constant offset every tick (`drift_per_step`) to mimic
//! monotonic ageing, plus a very small Gaussian `random_std` so it doesn’t
//! look perfectly linear.

use rand_distr::{Normal, Distribution};   // ← DODAJ `Distribution`
use super::Noise;


/// Slow monotonic trend + random flicker.
pub struct DriftNoise {
    pub drift_per_step: f64, // constant bias added each reading
    pub random_std:     f64, // σ of additional random term
    cumulative:         f64, // internal state (Σ drift)
}

impl DriftNoise {
    pub fn new(drift_per_step: f64, random_std: f64) -> Self {
        Self { drift_per_step, random_std, cumulative: 0.0 }
    }
}

impl Noise for DriftNoise {
    fn generate(&mut self, base: f64) -> f64 {
        // deterministic ageing
        self.cumulative += self.drift_per_step;

        // small extra flicker
        let gauss = Normal::new(0.0, self.random_std)
            .expect("σ>0").sample(&mut rand::thread_rng());

        base + self.cumulative + gauss
    }
}
