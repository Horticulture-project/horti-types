use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeLys {
    name: Option<String>,
    device_sn: u64,
    battery: Option<f32>,
    uptime: Option<u32>,
    last_active: DateTime<Utc>,
    fwver: Option<u32>,
    fwver_name: Option<String>,
    #[serde(default)]
    status: crate::devs::hb::DevStatus,
}

impl TeLys {
    pub fn new(
        device_sn: u64,
        name: Option<String>,
        uptime: Option<u32>,
        last_active: DateTime<Utc>,
        fwver: Option<u32>,
        fwver_name: Option<String>,
        status: crate::devs::hb::DevStatus,
        battery: Option<f32>,
    ) -> Self {
        Self {
            name,
            device_sn,
            uptime,
            last_active,
            fwver,
            fwver_name,
            status,
            battery,
        }
    }

    pub fn set_battery(&mut self, battery: Option<f32>) {
        self.battery = battery;
    }
    pub fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }
    pub fn battery(&self) -> Option<f32> {
        self.battery
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
    fn status(&self) -> crate::devs::hb::DevStatus {
        self.status
    }
}
