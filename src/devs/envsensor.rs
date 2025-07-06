use crate::devs::SensorReading;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EnvSensor {
    id: u64,
    pub name: Option<String>,
    pub temp: Option<SensorReading>,
    pub humidity: Option<SensorReading>,
    pub battery: Option<SensorReading>,
    pub uptime: Option<u32>,
    pub last_active: SystemTime,
}
impl EnvSensor {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            name: None,
            temp: None,
            humidity: None,
            battery: None,
            uptime: None,
            last_active: SystemTime::now(),
        }
    }
    pub fn temp(&self) -> Option<f32> {
        Some(self.temp?.to_float())
    }
    pub fn humidity(&self) -> Option<f32> {
        Some(self.humidity?.to_float())
    }

    pub fn battery(&self) -> Option<f32> {
        Some(self.battery?.to_float())
    }
}
impl super::Dev for EnvSensor {
    fn dev_id(&self) -> String {
        format!("{:#08x}", self.id)
    }
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn last_active(&self) -> SystemTime {
        self.last_active
    }

    fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }
    fn dev_type(&self) -> &'static str {
        "Environmental Sensor"
    }
}
