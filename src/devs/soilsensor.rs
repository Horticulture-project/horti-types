use crate::devs::SensorReading;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoilSensor {
    pub name: Option<String>,
    pub device_sn: u64,
    pub soil_moisture: Option<SensorReading>,
    pub temp: Option<SensorReading>,
    pub humidity: Option<SensorReading>,
    pub light: Option<SensorReading>,
    pub battery: Option<SensorReading>,
    pub uptime: Option<u32>,
    pub last_active: SystemTime,
}

impl SoilSensor {
    pub fn new(id: u64) -> Self {
        Self {
            name: None,
            device_sn: id,
            soil_moisture: None,
            temp: None,
            humidity: None,
            light: None,
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
    pub fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }
    pub fn battery(&self) -> Option<f32> {
        Some(self.battery?.to_float())
    }
}
impl super::Dev for SoilSensor {
    fn dev_id(&self) -> String {
        format!("{:#08x}", self.device_sn)
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
    fn display_name(&self) -> String {
        match self.name() {
            Some(name) => name.to_string(),
            None => self.dev_id(),
        }
    }
}
