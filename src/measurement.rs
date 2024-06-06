use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Debug, Clone)]
pub struct Measurement {
    pub channel: i32,
    #[serde(rename = "type")]
    pub measurement_type: i64,
    pub value1: i32,
    pub value2: i32,
}

impl Measurement {
    pub fn from_payload(payload: Vec<u8>) -> Option<Measurement> {
        trace!("sensor payload len{}", payload.len());
        if payload.len() == 12 {
            Some(Measurement {
                channel: i32::from_le_bytes([payload[0], 0, 0, 0]),
                measurement_type: i64::from_le_bytes([payload[1], 0, 0, 0, 0, 0, 0, 0]),
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
            channel: i32::from(self.channel),
            measurement_type: i64::from(self.measurement_type),
            value1: self.value1,
            value2: self.value2,
        }
    }
}
