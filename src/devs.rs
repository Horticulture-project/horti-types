use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

use crate::{
    devices_connected::DevicesConnected,
    devs::hb::{DevStatus, DevType},
};
pub trait Dev {
    fn dev_id(&self) -> String {
        format!("{:08x}", self.dev_sn())
    }
    fn dev_sn(&self) -> u64;
    fn name(&self) -> Option<&str>;
    fn last_active(&self) -> DateTime<Utc>;
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
    fn fwver(&self) -> Option<[u8; 4]> {
        None
    }
    fn fwver_name(&self) -> Option<String> {
        None
    }
    fn status(&self) -> crate::devs::hb::DevStatus {
        crate::devs::hb::DevStatus::Unknown(0)
    }
    fn has_sensors(&self) -> bool {
        false
    }
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
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub struct NameChange {
    name: Option<String>,
    description: Option<String>,
}
impl NameChange {
    pub fn new(_devid: u64, name: String) -> Self {
        Self {
            name: Some(name),
            description: None,
        }
    }

    pub fn new_with_description(
        _devid: u64,
        name: Option<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            name: name,
            description,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DescriptionChange {
    description: String,
}
impl DescriptionChange {
    pub fn new(_devid: u64, description: &str) -> Self {
        Self {
            description: description.to_string(),
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct SensorReading {
    pub h: i32,
    pub l: i32,
}
impl SensorReading {
    fn to_float(&self) -> f32 {
        self.h as f32 + (self.l as f32 * 0.000001)
    }
}
impl Dev for Device {
    fn dev_sn(&self) -> u64 {
        match self {
            Device::Soil(sensor) => sensor.dev_sn(),
            Device::Env(sensor) => sensor.dev_sn(),
            Device::Router(router) => router.dev_sn(),
            Device::Led(panel) => panel.dev_sn(),
            Device::TeLys(telys) => telys.dev_sn(),
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

    fn last_active(&self) -> DateTime<Utc> {
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Hash, Eq)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub struct DevInfo {
    pub dev_sn: u64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub rloc16: u16,
    pub status: DevStatus,
    pub last_active: DateTime<Utc>,
    pub dev_type: DevType,
    pub fwver: Option<u32>,
    pub fwver_name: Option<String>,
    pub uptime: Option<i64>,
    pub connected_devices: Vec<DevicesConnected>,
}
impl DevInfo {
    pub fn uptime(&self) -> Option<u32> {
        self.uptime.map(|u| u as u32)
    }
    pub fn set_fwver(&mut self, fwver: u32) {
        self.fwver = Some(fwver);
    }
    pub fn set_fwtag(&mut self, fwtag: String) {
        self.fwver_name = Some(fwtag);
    }
    pub fn status(&self) -> DevStatus {
        self.status.map_active(self.last_active())
    }
    pub fn set_connected_devices(&mut self, connected_devices: Vec<DevicesConnected>) {
        self.connected_devices = connected_devices;
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
     pub fn unknown_device(rloc: i32) -> Self {
        let rloc = rloc as u16;
        Self {
            dev_sn: rloc as u64,
            name: Some(format!("{:04x}", rloc)),
            description: Some("No status".to_string()),
            rloc16: rloc,
            status: DevStatus::Offline,
            last_active: Utc::now(),
            dev_type: DevType::Unknown(255),
            fwver: None,
            fwver_name: None,
            uptime: None,
            connected_devices: vec![],
        }
    }
}

impl Dev for DevInfo {
    fn dev_sn(&self) -> u64 {
        self.dev_sn
    }
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    fn last_active(&self) -> DateTime<Utc> {
        self.last_active
    }
    fn dev_type(&self) -> &'static str {
        self.dev_type.into()
    }
    fn fwver(&self) -> Option<[u8; 4]> {
        self.fwver.map(|v| v.to_be_bytes())
    }
    fn fwver_name(&self) -> Option<String> {
        self.fwver_name.clone()
    }
}
