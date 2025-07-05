// use crate::coap_backend::MeasurementType as MT;
// use crate::coap_backend::SensorChannel;
// use crate::storage::Measurement;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
pub mod envsensor;
pub mod hb;
pub mod ledpanel;
pub mod router;
pub mod soilsensor;
pub mod telys;
use envsensor::EnvSensor;
use ledpanel::LedPanel;
use router::Router;
use soilsensor::SoilSensor;
use telys::TeLys;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Device {
    Soil(SoilSensor),
    Env(EnvSensor),
    Router(Router),
    Led(LedPanel),
    TeLys(TeLys),
}
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct SensorReading {
    pub h: i32,
    pub l: i32,
    pub timestamp: SystemTime,
}
impl SensorReading {
    fn to_float(&self) -> f32 {
        self.h as f32 + (self.l as f32 * 0.000001)
    }
}
