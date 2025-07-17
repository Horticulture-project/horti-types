use crate::devs::SensorReading;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeLys {
    pub name: Option<String>,
    pub device_sn: u64,
    pub temp: Option<SensorReading>,
    pub light: Option<SensorReading>,
    pub battery: Option<SensorReading>,
    pub uptime: Option<u32>,
    pub last_active: DateTime<Utc>,
    pub fwver: Option<u32>,
    pub fwver_name: Option<String>,
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
            last_active: Utc::now(),
            fwver: None,
            fwver_name: None,
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
}
impl super::Dev for TeLys {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    fn dev_sn(&self) -> u64 {
        self.device_sn
    }

    fn last_active(&self) -> DateTime<Utc> {
        self.last_active
    }
    fn dev_type(&self) -> &'static str {
        "TeLys"
    }

    fn fwver(&self) -> Option<[u8; 4]> {
        self.fwver.map(|v| v.to_be_bytes())
    }
    fn fwver_name(&self) -> Option<String> {
        self.fwver_name.clone()
    }
}
