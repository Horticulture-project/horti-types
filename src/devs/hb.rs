use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::fmt::Display;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HeartBeatZephyr {
    pub id: i64,
    pub fwver: u32,
    pub status: u8,
    pub devtype: u8,
    pub rloc16: u16,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone, Copy)]
#[serde(tag = "kind")]
pub struct HeartBeat {
    #[serde(
        serialize_with = "serialize_u64_as_string",
        deserialize_with = "deserialize_string_as_u64"
    )]
    pub id: u64,
    #[serde(rename = "firmware")]
    pub fwver: u32,
    #[serde(rename = "type")]
    pub devtype: DevType,
    pub rloc16: u16,
    pub status: DevStatus,
    pub uptime: u32,
    pub infobits: u16,
}
impl HeartBeat {
    pub fn from_payload(payload: Vec<u8>) -> Option<HeartBeat> {
        if payload.len() == 16 {
            Some(HeartBeat {
                id: u64::from_le_bytes([
                    payload[0], payload[1], payload[2], payload[3], payload[4], payload[5],
                    payload[6], payload[7], // fixme
                ]),
                fwver: u32::from_le_bytes([payload[8], payload[9], payload[10], payload[11]]),
                rloc16: u16::from_le_bytes([payload[14], payload[15]]),
                status: DevStatus::from(payload[12]),
                devtype: DevType::from(payload[13]),
                uptime: 0,
                infobits: 0,
            })
        } else {
            None
        }
    }
    pub fn from_payload_zephyr(payload: Vec<u8>, id: u64) -> Option<HeartBeat> {
        if payload.len() >= 14 {
            let fwver = u32::from_le_bytes(payload[0..4].try_into().ok()?);
            let uptime = u32::from_le_bytes(payload[4..8].try_into().ok()?);
            let rloc16 = u16::from_le_bytes(payload[8..10].try_into().ok()?);
            let infobits = u16::from_le_bytes(payload[10..12].try_into().ok()?);
            let status = DevStatus::from(payload[12]);
            let devtype = DevType::from(payload[13]);
            Some(HeartBeat {
                id,
                fwver,
                status,
                devtype,
                rloc16,
                uptime,
                infobits,
            })
        } else {
            None
        }
    }
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn fwver(mut self, fwver: u32) -> Self {
        self.fwver = fwver;
        self
    }

    pub fn devtype(mut self, devtype: u8) -> Self {
        self.devtype = devtype.into();
        self
    }

    pub fn status(mut self, status: u8) -> Self {
        self.status = status.into();
        self
    }

    pub fn rloc16(mut self, rloc16: u16) -> Self {
        self.rloc16 = rloc16.into();
        self
    }
    pub fn uptime(mut self, uptime: std::time::Duration) -> Self {
        self.uptime = uptime.as_secs() as u32;
        self
    }
}
impl std::fmt::Display for HeartBeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HeartBeat SN:{:x}, FW: {:#010x}, Type: {}, Status: {}, Rloc: {:#06x}, Uptime: {}",
            self.id, self.fwver, self.devtype, self.status, self.rloc16, self.uptime
        )
    }
}

// enum dev_type // From zephyr code
#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum DevType {
    BorderRouter,
    HortiLed,
    HortiPlantSensor,
    WeatherStation,
    EnvironmentSensor,
    GarageDoor,
    GetshopModule,
    GetshopLock,
    StaySerosModule,
    StayIdlock,
    TeLys,
    Unknown(u8),
}
impl Default for DevType {
    fn default() -> Self {
        DevType::Unknown(0)
    }
}
impl<'a> Serialize for DevType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        u8::from(*self).serialize(serializer)
    }
}
impl<'a> Deserialize<'a> for DevType {
    fn deserialize<D>(deserializer: D) -> Result<DevType, D::Error>
    where
        D: serde::de::Deserializer<'a>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(value.into())
    }
}

impl From<u8> for DevType {
    fn from(item: u8) -> Self {
        match item {
            0 => DevType::BorderRouter,
            1 => DevType::HortiLed,
            2 => DevType::HortiPlantSensor,
            3 => DevType::WeatherStation,
            4 => DevType::EnvironmentSensor,
            5 => DevType::GarageDoor,
            16 => DevType::GetshopModule,
            17 => DevType::GetshopLock,
            18 => DevType::StaySerosModule,
            19 => DevType::StayIdlock,
            20 => DevType::TeLys,
            n => DevType::Unknown(n),
        }
    }
}
impl From<DevType> for u8 {
    fn from(value: DevType) -> Self {
        match value {
            DevType::BorderRouter => 0,
            DevType::HortiLed => 1,
            DevType::HortiPlantSensor => 2,
            DevType::WeatherStation => 3,
            DevType::EnvironmentSensor => 4,
            DevType::GarageDoor => 5,
            DevType::GetshopModule => 16,
            DevType::GetshopLock => 17,
            DevType::StaySerosModule => 18,
            DevType::StayIdlock => 19,
            DevType::TeLys => 20,
            DevType::Unknown(n) => n,
        }
    }
}

impl Display for DevType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DevType::BorderRouter => write!(f, "BorderRouter"),
            DevType::HortiLed => write!(f, "HortiLed"),
            DevType::HortiPlantSensor => write!(f, "HortiPlantSensor"),
            DevType::WeatherStation => write!(f, "WeatherStation"),
            DevType::EnvironmentSensor => write!(f, "EnvironmentSensor"),
            DevType::GarageDoor => write!(f, "GarageDoor"),
            DevType::GetshopModule => write!(f, "GetshopModule"),
            DevType::GetshopLock => write!(f, "GetshopLock"),
            DevType::StaySerosModule => write!(f, "StaySerosModule"),
            DevType::StayIdlock => write!(f, "StayIdlock"),
            DevType::TeLys => write!(f, "TeLys"),
            DevType::Unknown(n) => write!(f, "Unknown {}", n),
        }
    }
}
impl Into<&'static str> for DevType {
    fn into(self) -> &'static str {
        match self {
            DevType::Unknown(_n) => "Unknown device",
            DevType::BorderRouter => "BorderRouter",
            DevType::HortiLed => "Horticulture: LED-panel",
            DevType::HortiPlantSensor => "Horticulture: Soil Sensor",
            DevType::WeatherStation => "Weather Station",
            DevType::EnvironmentSensor => "Environment sensor",
            DevType::GarageDoor => "Garage door control",
            DevType::GetshopModule => "GetShop Module 1.5",
            DevType::GetshopLock => "GetShop Module 1.9",
            DevType::StaySerosModule => "StaySeros Module",
            DevType::StayIdlock => "StayIdlock",
            DevType::TeLys => "TeLys",
        }
    }
}
impl From<&str> for DevType {
    fn from(item: &str) -> Self {
        match item {
            "BorderRouter" => DevType::BorderRouter,
            "HortiLed" => DevType::HortiLed,
            "HortiPlantSensor" => DevType::HortiPlantSensor,
            "WeatherStation" => DevType::WeatherStation,
            "EnvironmentSensor" => DevType::EnvironmentSensor,
            "GarageDoor" => DevType::GarageDoor,
            "GetshopModule" => DevType::GetshopModule,
            "GetshopLock" => DevType::GetshopLock,
            "StaySerosModule" => DevType::StaySerosModule,
            "StayIdlock" => DevType::StayIdlock,
            "TeLys" => DevType::TeLys,
            _ => DevType::Unknown(0),
        }
    }
}
impl From<DevStatus> for i32 {
    fn from(value: DevStatus) -> Self {
        match value {
            _ => 0,
        }
    }
}
// enum status_code // From zephyr code
#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum DevStatus {
    Unknown(u8),
    Error,
    RunningOk,
    Downloading,
    Flashing,
    Rebooting,
    Offline,
}
impl DevStatus {
    pub fn map_active(&self, last_active: DateTime<Utc>) -> Self {
        match self {
            _ if last_active < Utc::now() - chrono::Duration::hours(5) => DevStatus::Offline,
            status => *status,
        }
    }
}
impl Serialize for DevStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        u8::from(*self).serialize(serializer)
    }
}
impl<'a> Deserialize<'a> for DevStatus {
    fn deserialize<D>(deserializer: D) -> Result<DevStatus, D::Error>
    where
        D: serde::de::Deserializer<'a>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(value.into())
    }
}
impl From<DevStatus> for u8 {
    fn from(value: DevStatus) -> Self {
        match value {
            DevStatus::Unknown(n) => n,
            DevStatus::Error => 1,
            DevStatus::RunningOk => 2,
            DevStatus::Downloading => 3,
            DevStatus::Flashing => 4,
            DevStatus::Rebooting => 5,
            DevStatus::Offline => 6,
        }
    }
}

impl Default for DevStatus {
    fn default() -> Self {
        DevStatus::Unknown(0)
    }
}
impl From<u8> for DevStatus {
    fn from(item: u8) -> Self {
        match item {
            1 => DevStatus::Error,
            2 => DevStatus::RunningOk,
            3 => DevStatus::Downloading,
            4 => DevStatus::Flashing,
            5 => DevStatus::Rebooting,
            6 => DevStatus::Offline,
            n => DevStatus::Unknown(n),
        }
    }
}
impl From<i32> for DevStatus {
    fn from(item: i32) -> Self {
        match item {
            0..=255 => (item as u8).into(),
            _ => DevStatus::Unknown(0xff),
        }
    }
}
impl Display for DevStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DevStatus::Unknown(n) => write!(f, "Unknown: {n}"),
            DevStatus::Error => write!(f, "Error"),
            DevStatus::RunningOk => write!(f, "RunningOk"),
            DevStatus::Downloading => write!(f, "Downloading"),
            DevStatus::Flashing => write!(f, "Flashing"),
            DevStatus::Rebooting => write!(f, "Rebooting"),
            DevStatus::Offline => write!(f, "Offline"),
        }
    }
}
impl From<&str> for DevStatus {
    fn from(item: &str) -> Self {
        match item {
            "Unknown" => DevStatus::Unknown(0),
            "Error" => DevStatus::Error,
            "RunningOk" => DevStatus::RunningOk,
            "Downloading" => DevStatus::Downloading,
            "Flashing" => DevStatus::Flashing,
            "Rebooting" => DevStatus::Rebooting,
            "Offline" => DevStatus::Offline,
            _ => DevStatus::Unknown(0),
        }
    }
}

// Helper functions for serializing u64 as string
fn serialize_u64_as_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

fn deserialize_string_as_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<u64>().map_err(serde::de::Error::custom)
}
