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
pub trait Dev {
    fn dev_id(&self) -> String;
    fn name(&self) -> Option<&str>;
    fn last_active(&self) -> SystemTime;
    fn uptime(&self) -> Option<std::time::Duration> {
        None
    }
    fn display_name(&self) -> String {
        match self.name() {
            Some(name) => name.to_string(),
            None => self.dev_id(),
        }
    }
    fn dev_type(&self) -> &'static str;
}
pub trait Battery {
    fn battery(&self) -> Option<f32> {
        None
    }
    fn bat_pct(&self) -> Option<f32> {
        self.battery().map(|b| (3.3 - 2.5) / (b - 2.5) * 100.0)
    }
}
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
impl Dev for Device {
    fn dev_id(&self) -> String {
        match self {
            Device::Soil(sensor) => sensor.dev_id(),
            Device::Env(sensor) => sensor.dev_id(),
            Device::Router(router) => router.dev_id(),
            Device::Led(panel) => panel.dev_id(),
            Device::TeLys(telys) => telys.dev_id(),
        }
    }

    fn name(&self) -> Option<&str> {
        match self {
            Device::Soil(sensor) => sensor.name(),
            Device::Env(sensor) => sensor.name(),
            Device::Router(router) => router.name(),
            Device::Led(panel) => panel.name(),
            Device::TeLys(telys) => telys.name(),
        }
    }

    fn last_active(&self) -> SystemTime {
        match self {
            Device::Soil(sensor) => sensor.last_active(),
            Device::Env(sensor) => sensor.last_active(),
            Device::Router(router) => router.last_active(),
            Device::Led(panel) => panel.last_active(),
            Device::TeLys(telys) => telys.last_active(),
        }
    }
    fn uptime(&self) -> Option<std::time::Duration> {
        match self {
            Device::Soil(sensor) => sensor.uptime(),
            Device::Env(sensor) => sensor.uptime(),
            Device::Router(router) => router.uptime(),
            Device::Led(panel) => panel.uptime(),
            Device::TeLys(telys) => telys.uptime(),
        }
    }
    fn display_name(&self) -> String {
        match self {
            Device::Soil(sensor) => sensor.display_name(),
            Device::Env(sensor) => sensor.display_name(),
            Device::Router(router) => router.display_name(),
            Device::Led(panel) => panel.display_name(),
            Device::TeLys(telys) => telys.display_name(),
        }
    }
    fn dev_type(&self) -> &'static str {
        match self {
            Device::Soil(_) => "Soil Sensor",
            Device::Env(_) => "Environmental Sensor",
            Device::Router(_) => "Router",
            Device::Led(_) => "LED Panel",
            Device::TeLys(_) => "TeLys",
        }
    }
}
