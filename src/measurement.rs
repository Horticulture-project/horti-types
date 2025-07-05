use log::*;
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Debug, Clone)]
pub struct Measurement {
    pub channel: SensorChannel,
    #[serde(rename = "type")]
    pub measurement_type: MeasurementType,
    pub value1: i32,
    pub value2: i32,
    pub timestamp: Option<SystemTime>,
}

impl Measurement {
    pub fn from_payload(payload: Vec<u8>) -> Option<Measurement> {
        trace!("sensor payload len{}", payload.len());
        if payload.len() == 12 {
            Some(Measurement {
                channel: payload[0].into(),
                measurement_type: payload[1].into(),
                value1: i32::from_le_bytes([payload[4], payload[5], payload[6], payload[7]]),
                value2: i32::from_le_bytes([payload[8], payload[9], payload[10], payload[11]]),
                timestamp: Some(SystemTime::now()),
            })
        } else {
            None
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Debug, Clone)]
pub struct SensorDataZephyr {
    pub channel: u8,
    pub measurement_type: u8,
    pub dts_id: u8,
    pub pad1: u8,
    pub value1: i32,
    pub value2: i32,
}

impl SensorDataZephyr {
    #[allow(dead_code)]
    pub fn to_plain(&self) -> Measurement {
        Measurement {
            channel: self.channel.into(),
            measurement_type: self.measurement_type.into(),
            value1: self.value1,
            value2: self.value2,
            timestamp: Some(SystemTime::now()),
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Debug, Clone)]
#[repr(u8)]
pub enum MeasurementType {
    DoorlockLogs = 10,
    AmbientTemperature = 13,
    Pressure = 14,
    Humidity = 16,
    IlluminanceVisible = 17,
    IlluminanceInfraRed = 18,
    IlluminanceRed = 19,
    IlluminanceGreen = 20,
    IlluminanceBlue = 21,
    Altitude = 22,
    Co2Level = 27,
    VocLevel = 28,
    GasSensorResistance = 29,
    Voltage = 30,
    Current = 31,
    Power = 32,
    UptimeCounter = 57,
    SensorChanF1_415 = 58,
    SensorChanF2_445 = 59,
    SensorChanF3_480 = 60,
    SensorChanF4_515 = 61,
    SensorChanF5_555 = 62,
    SensorChanF6_590 = 63,
    SensorChanF7_630 = 64,
    SensorChanF8_680 = 65,
    SensorChanNir = 66,
    PhSensor = 67,
    Tds = 68,
    Other(u8),
}
impl From<u8> for MeasurementType {
    fn from(v: u8) -> Self {
        match v {
            10 => MeasurementType::DoorlockLogs,
            13 => MeasurementType::AmbientTemperature,
            14 => MeasurementType::Pressure,
            16 => MeasurementType::Humidity,
            17 => MeasurementType::IlluminanceVisible,
            18 => MeasurementType::IlluminanceInfraRed,
            19 => MeasurementType::IlluminanceRed,
            20 => MeasurementType::IlluminanceGreen,
            21 => MeasurementType::IlluminanceBlue,
            22 => MeasurementType::Altitude,
            27 => MeasurementType::Co2Level,
            28 => MeasurementType::VocLevel,
            29 => MeasurementType::GasSensorResistance,
            30 => MeasurementType::Voltage,
            31 => MeasurementType::Current,
            32 => MeasurementType::Power,
            57 => MeasurementType::UptimeCounter,
            58 => MeasurementType::UptimeCounter,
            68 => MeasurementType::PhSensor,
            69 => MeasurementType::Tds,
            _ => MeasurementType::Other(v),
        }
    }
}
impl From<i32> for MeasurementType {
    fn from(v: i32) -> Self {
        match v {
            0..=255 => (v as u8).into(),
            _ => MeasurementType::Other(255),
        }
    }
}
impl From<i64> for MeasurementType {
    fn from(v: i64) -> Self {
        match v {
            0..=255 => (v as u8).into(),
            _ => MeasurementType::Other(255),
        }
    }
}

impl Into<i32> for MeasurementType {
    fn into(self) -> i32 {
        match self {
            MeasurementType::DoorlockLogs => 10,
            MeasurementType::AmbientTemperature => 13,
            MeasurementType::Pressure => 14,
            MeasurementType::Humidity => 16,
            MeasurementType::IlluminanceVisible => 17,
            MeasurementType::IlluminanceInfraRed => 18,
            MeasurementType::IlluminanceRed => 19,
            MeasurementType::IlluminanceGreen => 20,
            MeasurementType::IlluminanceBlue => 21,
            MeasurementType::Altitude => 22,
            MeasurementType::Co2Level => 27,
            MeasurementType::VocLevel => 28,
            MeasurementType::GasSensorResistance => 29,
            MeasurementType::Voltage => 30,
            MeasurementType::Current => 31,
            MeasurementType::Power => 32,
            MeasurementType::UptimeCounter => 57,
            MeasurementType::SensorChanF1_415 => 58,
            MeasurementType::SensorChanF2_445 => 59,
            MeasurementType::SensorChanF3_480 => 60,
            MeasurementType::SensorChanF4_515 => 61,
            MeasurementType::SensorChanF5_555 => 62,
            MeasurementType::SensorChanF6_590 => 63,
            MeasurementType::SensorChanF7_630 => 64,
            MeasurementType::SensorChanF8_680 => 65,
            MeasurementType::SensorChanNir => 66,
            MeasurementType::PhSensor => 67,
            MeasurementType::Tds => 68,
            MeasurementType::Other(v) => v as i32,
        }
    }
}
impl fmt::Display for MeasurementType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MeasurementType::DoorlockLogs => write!(f, "DoorlockLogs"),
            MeasurementType::AmbientTemperature => write!(f, "AmbientTemperature"),
            MeasurementType::Pressure => write!(f, "Pressure"),
            MeasurementType::Humidity => write!(f, "Humidity"),
            MeasurementType::IlluminanceVisible => write!(f, "IlluminanceVisible"),
            MeasurementType::IlluminanceInfraRed => write!(f, "IlluminanceInfraRed"),
            MeasurementType::IlluminanceRed => write!(f, "IlluminanceRed"),
            MeasurementType::IlluminanceGreen => write!(f, "IlluminanceGreen"),
            MeasurementType::IlluminanceBlue => write!(f, "IlluminanceBlue"),
            MeasurementType::Altitude => write!(f, "Altitude"),
            MeasurementType::Co2Level => write!(f, "Co2Level"),
            MeasurementType::VocLevel => write!(f, "VocLevel"),
            MeasurementType::GasSensorResistance => write!(f, "GasSensorResistance"),
            MeasurementType::Voltage => write!(f, "Voltage"),
            MeasurementType::Current => write!(f, "Current"),
            MeasurementType::Power => write!(f, "Power"),
            MeasurementType::UptimeCounter => write!(f, "UptimeCounter"),
            MeasurementType::SensorChanF1_415 => write!(f, "SensorChanF1_415"),
            MeasurementType::SensorChanF2_445 => write!(f, "SensorChanF2_445"),
            MeasurementType::SensorChanF3_480 => write!(f, "SensorChanF3_480"),
            MeasurementType::SensorChanF4_515 => write!(f, "SensorChanF4_515"),
            MeasurementType::SensorChanF5_555 => write!(f, "SensorChanF5_555"),
            MeasurementType::SensorChanF6_590 => write!(f, "SensorChanF6_590"),
            MeasurementType::SensorChanF7_630 => write!(f, "SensorChanF7_630"),
            MeasurementType::SensorChanF8_680 => write!(f, "SensorChanF8_680"),
            MeasurementType::SensorChanNir => write!(f, "SensorChanNir"),
            MeasurementType::PhSensor => write!(f, "PhSensor"),
            MeasurementType::Tds => write!(f, "Tds"),
            MeasurementType::Other(v) => write!(f, "Other({})", v),
        }
    }
}
impl From<&str> for MeasurementType {
    fn from(v: &str) -> Self {
        match v {
            "DoorlockLogs" => MeasurementType::DoorlockLogs,
            "AmbientTemperature" => MeasurementType::AmbientTemperature,
            "Pressure" => MeasurementType::Pressure,
            "Humidity" => MeasurementType::Humidity,
            "IlluminanceVisible" => MeasurementType::IlluminanceVisible,
            "IlluminanceInfraRed" => MeasurementType::IlluminanceInfraRed,
            "IlluminanceRed" => MeasurementType::IlluminanceRed,
            "IlluminanceGreen" => MeasurementType::IlluminanceGreen,
            "IlluminanceBlue" => MeasurementType::IlluminanceBlue,
            "Altitude" => MeasurementType::Altitude,
            "Co2Level" => MeasurementType::Co2Level,
            "VocLevel" => MeasurementType::VocLevel,
            "GasSensorResistance" => MeasurementType::GasSensorResistance,
            "Voltage" => MeasurementType::Voltage,
            "Current" => MeasurementType::Current,
            "Power" => MeasurementType::Power,
            "UptimeCounter" => MeasurementType::UptimeCounter,
            "SensorChanF1_415" => MeasurementType::SensorChanF1_415,
            "SensorChanF2_445" => MeasurementType::SensorChanF2_445,
            "SensorChanF3_480" => MeasurementType::SensorChanF3_480,
            "SensorChanF4_515" => MeasurementType::SensorChanF4_515,
            "SensorChanF5_555" => MeasurementType::SensorChanF5_555,
            "SensorChanF6_590" => MeasurementType::SensorChanF6_590,
            "SensorChanF7_630" => MeasurementType::SensorChanF7_630,
            "SensorChanF8_680" => MeasurementType::SensorChanF8_680,
            "SensorChanNir" => MeasurementType::SensorChanNir,
            "PhSensor" => MeasurementType::PhSensor,
            "Tds" => MeasurementType::Tds,
            _ => MeasurementType::Other(255),
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Debug, Clone, Copy)]
#[repr(i32)]
pub enum SensorChannel {
    Shtc3,
    CapSense,
    Other(u8),
}
impl From<u8> for SensorChannel {
    fn from(v: u8) -> Self {
        match v {
            0 => SensorChannel::Shtc3,
            1 => SensorChannel::CapSense,
            n => SensorChannel::Other(n),
        }
    }
}
impl From<i32> for SensorChannel {
    fn from(v: i32) -> Self {
        match v {
            0..=255 => SensorChannel::from(v as u8),
            _ => SensorChannel::Other(255),
        }
    }
}
impl Into<i32> for SensorChannel {
    fn into(self) -> i32 {
        match self {
            SensorChannel::Shtc3 => 0,
            SensorChannel::CapSense => 1,
            SensorChannel::Other(v) => v as i32,
        }
    }
}
