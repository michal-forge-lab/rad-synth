pub mod gaussian;
pub mod impulse;
pub mod drift;

/// Trait wspólny dla wszystkich modeli szumu.
pub trait Noise: Send + Sync {
    /// Zwraca zaburzoną wartość na podstawie wartości bazowej.
    fn generate(&mut self, base_value: f64) -> f64;
}

/// Helper – wyłączanie szumu (np. w testach).
pub struct NoNoise;
impl Noise for NoNoise {
    fn generate(&mut self, base_value: f64) -> f64 { base_value }
}
