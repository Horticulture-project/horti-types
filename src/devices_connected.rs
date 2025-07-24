use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind", rename = "devicesConnected")]
pub struct ApiDevicesConnected {
    current_item_count: usize,
    id: String,
    #[serde(rename = "items")]
    connected_devices: Vec<DevicesConnected>,
}
impl ApiDevicesConnected {
    pub fn from_bytes(bytes: &[u8], devid: u64) -> Self {
        let mut ret = vec![];
        let mut bytes = bytes;
        loop {
            if let [n1, n2, n3, n4, rest @ ..] = bytes {
                let current = u16::from_le_bytes([*n1, *n2]);
                let idx = u16::from_le_bytes([*n3, *n4]);
                ret.push(DevicesConnected {
                    device_id: DevicesConnectedTypes::from(current),
                    idx,
                });

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
        self.connected_devices.push(DevicesConnected::new(device));
        self.current_item_count = self.connected_devices.len();
    }
    pub fn add_device_idx(&mut self, device: DevicesConnectedTypes, idx: u16) {
        self.connected_devices
            .push(DevicesConnected::new_idx(device, idx));
        self.current_item_count = self.connected_devices.len();
    }
    pub fn len(&self) -> usize {
        self.current_item_count
    }
    pub fn as_slice(&self) -> &[DevicesConnected] {
        &self.connected_devices
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct DevicesConnected {
    device_id: DevicesConnectedTypes,
    idx: u16,
}
impl DevicesConnected {
    pub fn new(device_id: DevicesConnectedTypes) -> Self {
        Self { device_id, idx: 0 }
    }
    pub fn new_idx(device_id: DevicesConnectedTypes, idx: u16) -> Self {
        Self { device_id, idx }
    }
    pub fn device_id(&self) -> DevicesConnectedTypes {
        self.device_id
    }
    pub fn id(&self) -> u16 {
        self.device_id.into()
    }
    pub fn idx(&self) -> u16 {
        self.idx
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[repr(u16)]
#[serde(into = "u16", from = "u16")]
pub enum DevicesConnectedTypes {
    Default,
    HortiLed,
    Shmt3xSensor,
    WateringPump,
    FanController,
    FlickeringLed,
    StepperMotorDriver,
    DoorLock,
    DoorSensor,
    Other(u16),
}
impl From<u16> for DevicesConnectedTypes {
    fn from(value: u16) -> Self {
        match value {
            0 => DevicesConnectedTypes::Default,
            1 => DevicesConnectedTypes::HortiLed,
            2 => DevicesConnectedTypes::Shmt3xSensor,
            3 => DevicesConnectedTypes::WateringPump,
            4 => DevicesConnectedTypes::FanController,
            5 => DevicesConnectedTypes::FlickeringLed,
            6 => DevicesConnectedTypes::StepperMotorDriver,
            7 => DevicesConnectedTypes::DoorLock,
            8 => DevicesConnectedTypes::DoorSensor,
            _ => DevicesConnectedTypes::Other(value),
        }
    }
}

impl From<DevicesConnectedTypes> for u16 {
    fn from(device_type: DevicesConnectedTypes) -> Self {
        match device_type {
            DevicesConnectedTypes::Default => 0,
            DevicesConnectedTypes::HortiLed => 1,
            DevicesConnectedTypes::Shmt3xSensor => 2,
            DevicesConnectedTypes::WateringPump => 3,
            DevicesConnectedTypes::FanController => 4,
            DevicesConnectedTypes::FlickeringLed => 5,
            DevicesConnectedTypes::StepperMotorDriver => 6,
            DevicesConnectedTypes::DoorLock => 7,
            DevicesConnectedTypes::DoorSensor => 8,
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
        let result = ApiDevicesConnected::from_bytes(&bytes, 1337);
        println!("{:?}", result);

        // assert!(!connected_devices.connected_devices.is_empty());
    }
}
