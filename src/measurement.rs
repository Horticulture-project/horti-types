use chrono::{DateTime, Utc};
use log::*;
use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Debug, Clone)]
pub struct Measurement {
    pub channel: SensorChannel,
    #[serde(rename = "type")]
    pub measurement_type: MeasurementType,
    pub value1: i32,
    pub value2: i32,
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
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Debug, Clone)]
#[repr(u8)]
#[serde(into = "i32", from = "i32")]
pub enum MeasurementType {
    // Movement sensors
    AccelX = 0,
    AccelY = 1,
    AccelZ = 2,
    AccelXYZ = 3,
    GyroX = 4,
    GyroY = 5,
    GyroZ = 6,
    GyroXYZ = 7,
    MagnX = 8,
    MagnY = 9,
    MagnZ = 10,
    MagnXYZ = 11,
    
    // Temperature and environmental sensors
    DieTemp = 12,
    AmbientTemperature = 13,
    Pressure = 14,
    Proximity = 15,
    Humidity = 16,
    
    // Light sensors
    IlluminanceVisible = 17,
    IlluminanceInfraRed = 18,
    IlluminanceRed = 19,
    IlluminanceGreen = 20,
    IlluminanceBlue = 21,
    Altitude = 22,
    
    // Particle matter sensors
    PM1_0 = 23,
    PM2_5 = 24,
    PM10 = 25,
    Distance = 26,
    
    // Gas sensors
    Co2Level = 27,
    O2Level = 28,
    VocLevel = 29,
    GasSensorResistance = 30,
    
    // Electrical measurements
    Voltage = 31,
    ShuntVoltage = 32,
    Current = 33,
    Power = 34,
    Resistance = 35,
    
    // Motion and position
    Rotation = 36,
    PositionDeltaX = 37,
    PositionDeltaY = 38,
    PositionDeltaZ = 39,
    RPM = 40,
    
    // Battery gauge measurements
    GaugeVoltage = 41,
    GaugeAvgCurrent = 42,
    GaugeStandbyCurrent = 43,
    GaugeMaxLoadCurrent = 44,
    GaugeTemperature = 45,
    GaugeStateOfCharge = 46,
    GaugeFullChargeCapacity = 47,
    GaugeRemainingChargeCapacity = 48,
    GaugeNominalAvailableCapacity = 49,
    GaugeFullAvailableCapacity = 50,
    GaugeAvgPower = 51,
    GaugeStateOfHealth = 52,
    GaugeTimeToEmpty = 53,
    GaugeTimeToFull = 54,
    GaugeCycleCount = 55,
    GaugeDesignVoltage = 56,
    GaugeDesiredVoltage = 57,
    GaugeDesiredChargingCurrent = 58,
    
    // Custom sensor channels
    SensorChanF1_415 = 59,
    SensorChanF2_445 = 60,
    SensorChanF3_480 = 61,
    SensorChanF4_515 = 62,
    SensorChanF5_555 = 63,
    SensorChanF6_590 = 64,
    SensorChanF7_630 = 65,
    SensorChanF8_680 = 66,
    SensorChanNir = 67,
    PhSensor = 68,
    Tds = 69,
    
    // Special channels
    All = 70,
    
    // Legacy or other types
    DoorlockLogs = 100,
    UptimeCounter = 101,
    
    // Fallback for unknown types
    Other(u8),
}
impl From<u8> for MeasurementType {
    fn from(v: u8) -> Self {
        match v {
            0 => MeasurementType::AccelX,
            1 => MeasurementType::AccelY,
            2 => MeasurementType::AccelZ,
            3 => MeasurementType::AccelXYZ,
            4 => MeasurementType::GyroX,
            5 => MeasurementType::GyroY,
            6 => MeasurementType::GyroZ,
            7 => MeasurementType::GyroXYZ,
            8 => MeasurementType::MagnX,
            9 => MeasurementType::MagnY,
            10 => MeasurementType::MagnZ,
            11 => MeasurementType::MagnXYZ,
            12 => MeasurementType::DieTemp,
            13 => MeasurementType::AmbientTemperature,
            14 => MeasurementType::Pressure,
            15 => MeasurementType::Proximity,
            16 => MeasurementType::Humidity,
            17 => MeasurementType::IlluminanceVisible,
            18 => MeasurementType::IlluminanceInfraRed,
            19 => MeasurementType::IlluminanceRed,
            20 => MeasurementType::IlluminanceGreen,
            21 => MeasurementType::IlluminanceBlue,
            22 => MeasurementType::Altitude,
            23 => MeasurementType::PM1_0,
            24 => MeasurementType::PM2_5,
            25 => MeasurementType::PM10,
            26 => MeasurementType::Distance,
            27 => MeasurementType::Co2Level,
            28 => MeasurementType::O2Level,
            29 => MeasurementType::VocLevel,
            30 => MeasurementType::GasSensorResistance,
            31 => MeasurementType::Voltage,
            32 => MeasurementType::ShuntVoltage,
            33 => MeasurementType::Current,
            34 => MeasurementType::Power,
            35 => MeasurementType::Resistance,
            36 => MeasurementType::Rotation,
            37 => MeasurementType::PositionDeltaX,
            38 => MeasurementType::PositionDeltaY,
            39 => MeasurementType::PositionDeltaZ,
            40 => MeasurementType::RPM,
            41 => MeasurementType::GaugeVoltage,
            42 => MeasurementType::GaugeAvgCurrent,
            43 => MeasurementType::GaugeStandbyCurrent,
            44 => MeasurementType::GaugeMaxLoadCurrent,
            45 => MeasurementType::GaugeTemperature,
            46 => MeasurementType::GaugeStateOfCharge,
            47 => MeasurementType::GaugeFullChargeCapacity,
            48 => MeasurementType::GaugeRemainingChargeCapacity,
            49 => MeasurementType::GaugeNominalAvailableCapacity,
            50 => MeasurementType::GaugeFullAvailableCapacity,
            51 => MeasurementType::GaugeAvgPower,
            52 => MeasurementType::GaugeStateOfHealth,
            53 => MeasurementType::GaugeTimeToEmpty,
            54 => MeasurementType::GaugeTimeToFull,
            55 => MeasurementType::GaugeCycleCount,
            56 => MeasurementType::GaugeDesignVoltage,
            57 => MeasurementType::GaugeDesiredVoltage,
            58 => MeasurementType::GaugeDesiredChargingCurrent,
            59 => MeasurementType::All,
            60 => MeasurementType::SensorChanF1_415,
            61 => MeasurementType::SensorChanF2_445,
            62 => MeasurementType::SensorChanF3_480,
            63 => MeasurementType::SensorChanF4_515,
            64 => MeasurementType::SensorChanF5_555,
            65 => MeasurementType::SensorChanF6_590,
            66 => MeasurementType::SensorChanF7_630,
            67 => MeasurementType::SensorChanF8_680,
            68 => MeasurementType::SensorChanNir,
            69 => MeasurementType::PhSensor,
            70 => MeasurementType::Tds,
            71 => MeasurementType::All,
            100 => MeasurementType::DoorlockLogs,
            101 => MeasurementType::UptimeCounter,
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
            MeasurementType::AccelX => 0,
            MeasurementType::AccelY => 1,
            MeasurementType::AccelZ => 2,
            MeasurementType::AccelXYZ => 3,
            MeasurementType::GyroX => 4,
            MeasurementType::GyroY => 5,
            MeasurementType::GyroZ => 6,
            MeasurementType::GyroXYZ => 7,
            MeasurementType::MagnX => 8,
            MeasurementType::MagnY => 9,
            MeasurementType::MagnZ => 10,
            MeasurementType::MagnXYZ => 11,
            MeasurementType::DieTemp => 12,
            MeasurementType::AmbientTemperature => 13,
            MeasurementType::Pressure => 14,
            MeasurementType::Proximity => 15,
            MeasurementType::Humidity => 16,
            MeasurementType::IlluminanceVisible => 17,
            MeasurementType::IlluminanceInfraRed => 18,
            MeasurementType::IlluminanceRed => 19,
            MeasurementType::IlluminanceGreen => 20,
            MeasurementType::IlluminanceBlue => 21,
            MeasurementType::Altitude => 22,
            MeasurementType::PM1_0 => 23,
            MeasurementType::PM2_5 => 24,
            MeasurementType::PM10 => 25,
            MeasurementType::Distance => 26,
            MeasurementType::Co2Level => 27,
            MeasurementType::O2Level => 28,
            MeasurementType::VocLevel => 29,
            MeasurementType::GasSensorResistance => 30,
            MeasurementType::Voltage => 31,
            MeasurementType::ShuntVoltage => 32,
            MeasurementType::Current => 33,
            MeasurementType::Power => 34,
            MeasurementType::Resistance => 35,
            MeasurementType::Rotation => 36,
            MeasurementType::PositionDeltaX => 37,
            MeasurementType::PositionDeltaY => 38,
            MeasurementType::PositionDeltaZ => 39,
            MeasurementType::RPM => 40,
            MeasurementType::GaugeVoltage => 41,
            MeasurementType::GaugeAvgCurrent => 42,
            MeasurementType::GaugeStandbyCurrent => 43,
            MeasurementType::GaugeMaxLoadCurrent => 44,
            MeasurementType::GaugeTemperature => 45,
            MeasurementType::GaugeStateOfCharge => 46,
            MeasurementType::GaugeFullChargeCapacity => 47,
            MeasurementType::GaugeRemainingChargeCapacity => 48,
            MeasurementType::GaugeNominalAvailableCapacity => 49,
            MeasurementType::GaugeFullAvailableCapacity => 50,
            MeasurementType::GaugeAvgPower => 51,
            MeasurementType::GaugeStateOfHealth => 52,
            MeasurementType::GaugeTimeToEmpty => 53,
            MeasurementType::GaugeTimeToFull => 54,
            MeasurementType::GaugeCycleCount => 55,
            MeasurementType::GaugeDesignVoltage => 56,
            MeasurementType::GaugeDesiredVoltage => 57,
            MeasurementType::GaugeDesiredChargingCurrent => 58,
            MeasurementType::All => 59,
            MeasurementType::SensorChanF1_415 => 60,
            MeasurementType::SensorChanF2_445 => 61,
            MeasurementType::SensorChanF3_480 => 62,
            MeasurementType::SensorChanF4_515 => 63,
            MeasurementType::SensorChanF5_555 => 64,
            MeasurementType::SensorChanF6_590 => 65,
            MeasurementType::SensorChanF7_630 => 66,
            MeasurementType::SensorChanF8_680 => 67,
            MeasurementType::SensorChanNir => 68,
            MeasurementType::PhSensor => 69,
            MeasurementType::Tds => 70,
            MeasurementType::DoorlockLogs => 100,
            MeasurementType::UptimeCounter => 101,
            MeasurementType::Other(v) => v as i32,
        }
    }
}
impl fmt::Display for MeasurementType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MeasurementType::AccelX => write!(f, "AccelX"),
            MeasurementType::AccelY => write!(f, "AccelY"),
            MeasurementType::AccelZ => write!(f, "AccelZ"),
            MeasurementType::AccelXYZ => write!(f, "AccelXYZ"),
            MeasurementType::GyroX => write!(f, "GyroX"),
            MeasurementType::GyroY => write!(f, "GyroY"),
            MeasurementType::GyroZ => write!(f, "GyroZ"),
            MeasurementType::GyroXYZ => write!(f, "GyroXYZ"),
            MeasurementType::MagnX => write!(f, "MagnX"),
            MeasurementType::MagnY => write!(f, "MagnY"),
            MeasurementType::MagnZ => write!(f, "MagnZ"),
            MeasurementType::MagnXYZ => write!(f, "MagnXYZ"),
            MeasurementType::DieTemp => write!(f, "DieTemp"),
            MeasurementType::AmbientTemperature => write!(f, "AmbientTemperature"),
            MeasurementType::Pressure => write!(f, "Pressure"),
            MeasurementType::Proximity => write!(f, "Proximity"),
            MeasurementType::Humidity => write!(f, "Humidity"),
            MeasurementType::IlluminanceVisible => write!(f, "IlluminanceVisible"),
            MeasurementType::IlluminanceInfraRed => write!(f, "IlluminanceInfraRed"),
            MeasurementType::IlluminanceRed => write!(f, "IlluminanceRed"),
            MeasurementType::IlluminanceGreen => write!(f, "IlluminanceGreen"),
            MeasurementType::IlluminanceBlue => write!(f, "IlluminanceBlue"),
            MeasurementType::Altitude => write!(f, "Altitude"),
            MeasurementType::PM1_0 => write!(f, "PM1_0"),
            MeasurementType::PM2_5 => write!(f, "PM2_5"),
            MeasurementType::PM10 => write!(f, "PM10"),
            MeasurementType::Distance => write!(f, "Distance"),
            MeasurementType::Co2Level => write!(f, "Co2Level"),
            MeasurementType::O2Level => write!(f, "O2Level"),
            MeasurementType::VocLevel => write!(f, "VocLevel"),
            MeasurementType::GasSensorResistance => write!(f, "GasSensorResistance"),
            MeasurementType::Voltage => write!(f, "Voltage"),
            MeasurementType::ShuntVoltage => write!(f, "ShuntVoltage"),
            MeasurementType::Current => write!(f, "Current"),
            MeasurementType::Power => write!(f, "Power"),
            MeasurementType::Resistance => write!(f, "Resistance"),
            MeasurementType::Rotation => write!(f, "Rotation"),
            MeasurementType::PositionDeltaX => write!(f, "PositionDeltaX"),
            MeasurementType::PositionDeltaY => write!(f, "PositionDeltaY"),
            MeasurementType::PositionDeltaZ => write!(f, "PositionDeltaZ"),
            MeasurementType::RPM => write!(f, "RPM"),
            MeasurementType::GaugeVoltage => write!(f, "GaugeVoltage"),
            MeasurementType::GaugeAvgCurrent => write!(f, "GaugeAvgCurrent"),
            MeasurementType::GaugeStandbyCurrent => write!(f, "GaugeStandbyCurrent"),
            MeasurementType::GaugeMaxLoadCurrent => write!(f, "GaugeMaxLoadCurrent"),
            MeasurementType::GaugeTemperature => write!(f, "GaugeTemperature"),
            MeasurementType::GaugeStateOfCharge => write!(f, "GaugeStateOfCharge"),
            MeasurementType::GaugeFullChargeCapacity => write!(f, "GaugeFullChargeCapacity"),
            MeasurementType::GaugeRemainingChargeCapacity => write!(f, "GaugeRemainingChargeCapacity"),
            MeasurementType::GaugeNominalAvailableCapacity => write!(f, "GaugeNominalAvailableCapacity"),
            MeasurementType::GaugeFullAvailableCapacity => write!(f, "GaugeFullAvailableCapacity"),
            MeasurementType::GaugeAvgPower => write!(f, "GaugeAvgPower"),
            MeasurementType::GaugeStateOfHealth => write!(f, "GaugeStateOfHealth"),
            MeasurementType::GaugeTimeToEmpty => write!(f, "GaugeTimeToEmpty"),
            MeasurementType::GaugeTimeToFull => write!(f, "GaugeTimeToFull"),
            MeasurementType::GaugeCycleCount => write!(f, "GaugeCycleCount"),
            MeasurementType::GaugeDesignVoltage => write!(f, "GaugeDesignVoltage"),
            MeasurementType::GaugeDesiredVoltage => write!(f, "GaugeDesiredVoltage"),
            MeasurementType::GaugeDesiredChargingCurrent => write!(f, "GaugeDesiredChargingCurrent"),
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
            MeasurementType::All => write!(f, "All"),
            MeasurementType::DoorlockLogs => write!(f, "DoorlockLogs"),
            MeasurementType::UptimeCounter => write!(f, "UptimeCounter"),
            MeasurementType::Other(v) => write!(f, "Other({})", v),
        }
    }
}
impl From<&str> for MeasurementType {
    fn from(v: &str) -> Self {
        match v {
            "AccelX" => MeasurementType::AccelX,
            "AccelY" => MeasurementType::AccelY,
            "AccelZ" => MeasurementType::AccelZ,
            "AccelXYZ" => MeasurementType::AccelXYZ,
            "GyroX" => MeasurementType::GyroX,
            "GyroY" => MeasurementType::GyroY,
            "GyroZ" => MeasurementType::GyroZ,
            "GyroXYZ" => MeasurementType::GyroXYZ,
            "MagnX" => MeasurementType::MagnX,
            "MagnY" => MeasurementType::MagnY,
            "MagnZ" => MeasurementType::MagnZ,
            "MagnXYZ" => MeasurementType::MagnXYZ,
            "DieTemp" => MeasurementType::DieTemp,
            "AmbientTemperature" => MeasurementType::AmbientTemperature,
            "Pressure" => MeasurementType::Pressure,
            "Proximity" => MeasurementType::Proximity,
            "Humidity" => MeasurementType::Humidity,
            "IlluminanceVisible" => MeasurementType::IlluminanceVisible,
            "IlluminanceInfraRed" => MeasurementType::IlluminanceInfraRed,
            "IlluminanceRed" => MeasurementType::IlluminanceRed,
            "IlluminanceGreen" => MeasurementType::IlluminanceGreen,
            "IlluminanceBlue" => MeasurementType::IlluminanceBlue,
            "Altitude" => MeasurementType::Altitude,
            "PM1_0" => MeasurementType::PM1_0,
            "PM2_5" => MeasurementType::PM2_5,
            "PM10" => MeasurementType::PM10,
            "Distance" => MeasurementType::Distance,
            "Co2Level" => MeasurementType::Co2Level,
            "O2Level" => MeasurementType::O2Level,
            "VocLevel" => MeasurementType::VocLevel,
            "GasSensorResistance" => MeasurementType::GasSensorResistance,
            "Voltage" => MeasurementType::Voltage,
            "ShuntVoltage" => MeasurementType::ShuntVoltage,
            "Current" => MeasurementType::Current,
            "Power" => MeasurementType::Power,
            "Resistance" => MeasurementType::Resistance,
            "Rotation" => MeasurementType::Rotation,
            "PositionDeltaX" => MeasurementType::PositionDeltaX,
            "PositionDeltaY" => MeasurementType::PositionDeltaY,
            "PositionDeltaZ" => MeasurementType::PositionDeltaZ,
            "RPM" => MeasurementType::RPM,
            "GaugeVoltage" => MeasurementType::GaugeVoltage,
            "GaugeAvgCurrent" => MeasurementType::GaugeAvgCurrent,
            "GaugeStandbyCurrent" => MeasurementType::GaugeStandbyCurrent,
            "GaugeMaxLoadCurrent" => MeasurementType::GaugeMaxLoadCurrent,
            "GaugeTemperature" => MeasurementType::GaugeTemperature,
            "GaugeStateOfCharge" => MeasurementType::GaugeStateOfCharge,
            "GaugeFullChargeCapacity" => MeasurementType::GaugeFullChargeCapacity,
            "GaugeRemainingChargeCapacity" => MeasurementType::GaugeRemainingChargeCapacity,
            "GaugeNominalAvailableCapacity" => MeasurementType::GaugeNominalAvailableCapacity,
            "GaugeFullAvailableCapacity" => MeasurementType::GaugeFullAvailableCapacity,
            "GaugeAvgPower" => MeasurementType::GaugeAvgPower,
            "GaugeStateOfHealth" => MeasurementType::GaugeStateOfHealth,
            "GaugeTimeToEmpty" => MeasurementType::GaugeTimeToEmpty,
            "GaugeTimeToFull" => MeasurementType::GaugeTimeToFull,
            "GaugeCycleCount" => MeasurementType::GaugeCycleCount,
            "GaugeDesignVoltage" => MeasurementType::GaugeDesignVoltage,
            "GaugeDesiredVoltage" => MeasurementType::GaugeDesiredVoltage,
            "GaugeDesiredChargingCurrent" => MeasurementType::GaugeDesiredChargingCurrent,
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
            "All" => MeasurementType::All,
            "DoorlockLogs" => MeasurementType::DoorlockLogs,
            "UptimeCounter" => MeasurementType::UptimeCounter,
            _ => MeasurementType::Other(255),
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Debug, Clone, Copy)]
#[repr(i32)]
#[serde(into = "i32", from = "i32")]
pub enum SensorChannel {
    Other(u8),
}
impl From<u8> for SensorChannel {
    fn from(v: u8) -> Self {
        match v {
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
            SensorChannel::Other(v) => v as i32,
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "kind", rename = "Measurement")]
pub struct ApiMeasurements {
    id: String,
    current_item_count: usize,
    updated: DateTime<Utc>,
    items: Vec<Measurement>,
}
impl ApiMeasurements {
    pub fn new(id: u64) -> Self {
        Self {
            items: vec![],
            current_item_count: 0,
            id: id.to_string(),
            updated: Utc::now(),
        }
    }

    pub fn from_vec(id: u64, items: Vec<Measurement>) -> Self {
        let current_item_count = items.len();
        Self {
            items,
            current_item_count,
            id: id.to_string(),
            updated: Utc::now(),
        }
    }
    pub fn add_measurement(&mut self, measurement: Measurement) {
        self.items.push(measurement);
        self.current_item_count = self.items.len();
    }
    pub fn len(&self) -> usize {
        self.current_item_count
    }
    pub fn as_slice(&self) -> &[Measurement] {
        &self.items
    }
    pub fn into_vec(self) -> Vec<Measurement> {
        self.items
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn updated(&self) -> DateTime<Utc> {
        self.updated
    }
}
