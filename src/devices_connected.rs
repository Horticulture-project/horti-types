use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub struct DevicesConnected {
    current_item_count: usize,
    id: String,
    #[serde(rename = "items")]
    connected_devices: Vec<DevicesConnectedTypes>,
}
impl DevicesConnected {
    pub fn from_bytes(bytes: &[u8], devid: u64) -> Self {
        let mut ret = vec![];
        let mut bytes = bytes;
        loop {
            if let Some((current, rest)) = bytes.split_at_checked(4) {
                if let Ok(arr) = current.try_into() {
                    let current = i32::from_le_bytes(arr);
                    ret.push(DevicesConnectedTypes::from(current));
                }
                bytes = rest;
            } else {
                break;
            }
        }
        Self {
            current_item_count: ret.len(),
            connected_devices: ret,

            id: devid.to_string(),
        }
    }
    pub fn new(devid: u64) -> Self {
        Self {
            connected_devices: vec![],
            current_item_count: 0,
            id: devid.to_string(),
        }
    }
    pub fn add_device(&mut self, device: DevicesConnectedTypes) {
        self.connected_devices.push(device);
        self.current_item_count = self.connected_devices.len();
    }
    pub fn len(&self) -> usize {
        self.current_item_count
    }
    pub fn as_slice(&self) -> &[DevicesConnectedTypes] {
        &self.connected_devices
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[repr(i32)]
#[serde(into = "i32", from = "i32")]
pub enum DevicesConnectedTypes {
    Default = 0,
    HortiLed1 = 1,
    HortiLed2 = 2,
    HortiLed3 = 3,
    HortiLed4 = 4,
    HortiLed5 = 5,
    Shmt3xSensor = 6,
    WateringPump = 7,
    FanController = 8,
    FlickeringLed = 9,
    StepperMotorDriver = 10,
    DoorLock = 11,
    DoorSensor = 12,
    Other(i32),
}
impl From<i32> for DevicesConnectedTypes {
    fn from(value: i32) -> Self {
        match value {
            0 => DevicesConnectedTypes::Default,
            1 => DevicesConnectedTypes::HortiLed1,
            2 => DevicesConnectedTypes::HortiLed2,
            3 => DevicesConnectedTypes::HortiLed3,
            4 => DevicesConnectedTypes::HortiLed4,
            5 => DevicesConnectedTypes::HortiLed5,
            6 => DevicesConnectedTypes::Shmt3xSensor,
            7 => DevicesConnectedTypes::WateringPump,
            8 => DevicesConnectedTypes::FanController,
            9 => DevicesConnectedTypes::FlickeringLed,
            10 => DevicesConnectedTypes::StepperMotorDriver,
            11 => DevicesConnectedTypes::DoorLock,
            12 => DevicesConnectedTypes::DoorSensor,
            _ => DevicesConnectedTypes::Other(value),
        }
    }
}

impl From<DevicesConnectedTypes> for i32 {
    fn from(device_type: DevicesConnectedTypes) -> Self {
        match device_type {
            DevicesConnectedTypes::Default => 0,
            DevicesConnectedTypes::HortiLed1 => 1,
            DevicesConnectedTypes::HortiLed2 => 2,
            DevicesConnectedTypes::HortiLed3 => 3,
            DevicesConnectedTypes::HortiLed4 => 4,
            DevicesConnectedTypes::HortiLed5 => 5,
            DevicesConnectedTypes::Shmt3xSensor => 6,
            DevicesConnectedTypes::WateringPump => 7,
            DevicesConnectedTypes::FanController => 8,
            DevicesConnectedTypes::FlickeringLed => 9,
            DevicesConnectedTypes::StepperMotorDriver => 10,
            DevicesConnectedTypes::DoorLock => 11,
            DevicesConnectedTypes::DoorSensor => 12,
            DevicesConnectedTypes::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_connected_devices() {
        let bytes: Vec<u8> = vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Example byte array
        let result = DevicesConnected::from_bytes(&bytes, 1337);
        println!("{:?}", result);

        // assert!(!connected_devices.connected_devices.is_empty());
    }
}
