use crate::devs::SensorReading;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeLys {
    pub name: Option<String>,
    pub device_sn: u64,
    pub temp: Option<SensorReading>,
    pub light: Option<SensorReading>,
    pub battery: Option<SensorReading>,
    pub uptime: Option<u32>,
    pub last_active: SystemTime,
}

impl TeLys {
    pub fn new(id: u64) -> Self {
        Self {
            name: None,
            device_sn: id,
            temp: None,
            light: None,
            battery: None,
            uptime: None,
            last_active: SystemTime::now(),
        }
    }

    pub fn temp(&self) -> Option<f32> {
        Some(self.temp?.to_float())
    }

    pub fn light(&self) -> Option<f32> {
        Some(self.light?.to_float())
    }

    pub fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }

    pub fn battery(&self) -> Option<f32> {
        Some(self.battery?.to_float())
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn dev_id(&self) -> String {
        format!("{:#08x}", self.device_sn)
    }
    pub fn last_active(&self) -> SystemTime {
        self.last_active
    }
}
