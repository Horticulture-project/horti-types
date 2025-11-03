pub mod post;
use std::collections::HashMap;

use crate::devices_connected::ApiDevicesConnected;

use crate::measurement::ApiMeasurements;
use crate::neighbors::ApiNeighbors;

use crate::otnet::OtNetConfig;
use crate::otnet::OtNetwork;
use crate::settings::ApiDevSettings;
use crate::settings::ApiSettingTypes;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
impl JsonMessage {
    pub fn new(data: ItemTypes) -> Self {
        Self {
            api_version: "1.0".to_string(),
            data,
            warning: None,
            meta: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JsonMessage {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub data: ItemTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<HashMap<String, i32>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemTypes {
    Settings(ApiDevSettings),
    SettingTypes(ApiSettingTypes),
    ConnectedDevices(ApiDevicesConnected),
    OtNet(Vec<OtNetwork>),
    Measurement(ApiMeasurements),
    Neighbor(ApiNeighbors),
    OtNetConfig(Vec<OtNetConfig>),
    DeviceInfo(crate::devs::DevInfo),
    HeartBeat(crate::devs::hb::HeartBeat),
    NameChange(crate::devs::NameChange),
    DescriptionChange(crate::devs::DescriptionChange), // remove
}
impl ItemTypes {
    pub fn len(&self) -> usize {
        match self {
            ItemTypes::Settings(settings) => settings.len(),
            ItemTypes::HeartBeat(_) => 1,
            ItemTypes::ConnectedDevices(connected_devices) => connected_devices.len(),
            ItemTypes::OtNet(otnet) => otnet.len(),
            ItemTypes::Measurement(measurements) => measurements.len(),
            ItemTypes::Neighbor(neighbors) => neighbors.len(),
            ItemTypes::OtNetConfig(otconfig) => otconfig.len(),
            ItemTypes::SettingTypes(setting_types) => setting_types.len(),
            ItemTypes::NameChange(_) => 1,
            ItemTypes::DescriptionChange(_) => 1,
            ItemTypes::DeviceInfo(_) => 1,
        }
    }
    pub fn kind(&self) -> &str {
        match self {
            ItemTypes::Settings(_) => "Setting",
            ItemTypes::ConnectedDevices(_) => "ConnectedDeviceType",
            ItemTypes::OtNet(_) => "OtNet",
            ItemTypes::Measurement(_) => "Measurement",
            ItemTypes::Neighbor(_) => "Neighbor",
            ItemTypes::OtNetConfig(_) => "OtNetConfig",
            ItemTypes::HeartBeat(_) => "HeartBeat",
            ItemTypes::SettingTypes(_) => "SettingType",
            ItemTypes::NameChange(_) => "NameChange",
            ItemTypes::DescriptionChange(_) => "DescriptionChange",
            ItemTypes::DeviceInfo(_) => "DeviceInfo",
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiDataList {
    id: String,
    kind: String,
    // updated: String,
    #[serde(rename = "currentItemCount")]
    current_item_count: usize,
    items: Option<ItemTypes>,
}
impl ApiDataList {
    pub fn new(id: String, items: ItemTypes) -> Self {
        Self {
            id,
            kind: items.kind().to_string(),
            current_item_count: items.len(),
            items: Some(items),
        }
    }
}
