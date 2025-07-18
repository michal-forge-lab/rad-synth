//! # GaussianNoise – white / thermal detector noise
//!
//! **Physics in one minute**
//! 1. A radiation detector (Geiger tube, PIN diode, scintillator+PMT…) is
//!    bombarded by *many* uncorrelated micro-events: thermal carriers,
//!    environmental γ-quanta, pickup in the front-end electronics.
//! 2. The Central Limit Theorem tells us that the sum of ≫10 000 tiny,
//!    independent contributions approaches a *normal* distribution.
//! 3. We therefore generate `N(0, σ)` and add it to the baseline to emulate
//!    the “every-sample” flicker you see on real hardware.
//
//! Typical values  
//! • σ ≈ 1 – 10 % of the baseline for PIN diodes  
//! • σ << 1 % for integrating (current-mode) ion chambers.

use rand_distr::{Distribution, Normal};
use super::Noise;

/// Zero-mean white noise with given σ (std-dev).
pub struct GaussianNoise {
    distribution: Normal<f64>,
}

impl GaussianNoise {
    /// `std_dev` in *same units* as the sensor output
    pub fn new(std_dev: f64) -> Self {
        Self {
            distribution: Normal::new(0.0, std_dev)
                .expect("σ>0 ⇒ Normal::new never fails"),
        }
    }
}

impl Noise for GaussianNoise {
    fn generate(&mut self, base: f64) -> f64 {
        base + self.distribution.sample(&mut rand::thread_rng())
    }
}
