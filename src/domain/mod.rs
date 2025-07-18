
pub mod noise;      // gaussian, drift, impulse  + trait Noise
pub mod sensors;    // gamma, uv, neutron        + trait Sensor
pub mod engine;     // tick() → Vec<(name,value)>

/// wygodny alias
pub use engine::Engine;

/* ------- re-eksport sensorów ----------------------------------- */
pub use sensors::{
    Sensor,
    gamma::GammaSensor,
    uv::UVSensor,
    neutron::NeutronSensor,
};

/* ------- re-eksport modeli szumu ------------------------------- */
pub use noise::{
    Noise,
    gaussian::GaussianNoise,
    drift::DriftNoise,
    impulse::ImpulseNoise,
};
