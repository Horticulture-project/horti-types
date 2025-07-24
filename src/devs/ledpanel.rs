use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::devices_connected::DevicesConnectedTypes;
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LedPanel {
    id: u64,
    pub name: Option<String>,
    pub uptime: Option<u32>,
    pub last_active: DateTime<Utc>,
    pub fwver: Option<u32>,
    pub fwver_name: Option<String>,
    pub status: crate::devs::hb::DevStatus,
    connected_devices: Vec<(DevicesConnectedTypes, u16)>,
}
impl LedPanel {
    pub fn new(
        id: u64,
        name: Option<String>,
        uptime: Option<u32>,
        last_active: DateTime<Utc>,
        fwver: Option<u32>,
        fwver_name: Option<String>,
        connected_devices: Vec<(DevicesConnectedTypes, u16)>,
        status: crate::devs::hb::DevStatus,
    ) -> Self {
        Self {
            id,
            name,
            uptime,
            last_active,
            fwver,
            fwver_name,
            status,
            connected_devices,
        }
    }
    pub fn get_connected_devices(&self) -> &[(DevicesConnectedTypes, u16)] {
        &self.connected_devices
    }
}
impl super::Dev for LedPanel {
    fn dev_sn(&self) -> u64 {
        self.id
    }
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }

    fn last_active(&self) -> DateTime<Utc> {
        self.last_active
    }
    fn dev_type(&self) -> &'static str {
        "LED Panel"
    }

    fn fwver(&self) -> Option<[u8; 4]> {
        self.fwver.map(|v| v.to_be_bytes())
    }
    fn fwver_name(&self) -> Option<String> {
        self.fwver_name.clone()
    }
    fn status(&self) -> crate::devs::hb::DevStatus {
        self.status.map_active(self.last_active)
    }
}
