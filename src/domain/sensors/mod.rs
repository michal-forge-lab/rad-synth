pub mod gamma;
pub mod uv;
pub mod neutron;

pub trait Sensor: Send {
    fn tick(&mut self) -> f64;
    fn name(&self) -> &str;
}
