//! # ImpulseNoise – rare “salt-&-pepper” bursts
//!
//! **Sources**  
//! • A single heavy ion from a solar flare dumps huge charge.  
//! • HV discharge in the dynode chain.  
//! • Cosmic muon triggers a big pulse in a deep-pixel sensor.
//!
//! We flip a biased coin (`prob`). On heads we add or subtract an amplitude
//! drawn *uniformly* between `min_amp … max_amp`; otherwise we leave the
//! value untouched.

use rand::Rng;
use super::Noise;

pub struct ImpulseNoise {
    pub prob:     f64, // 0 – 1   (np. 0.01 = 1 %)
    pub min_amp:  f64, // minimalny skok
    pub max_amp:  f64, // maksymalny skok
}

impl Noise for ImpulseNoise {
    fn generate(&mut self, base: f64) -> f64 {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < self.prob {
            let amp = rng.gen_range(self.min_amp..=self.max_amp);
            base + if rng.gen::<bool>() { amp } else { -amp }
        } else {
            base
        }
    }
}
