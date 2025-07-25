use bincode::Encode;
use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
// enum settings_list {
//     DEV_TYPE = 0,
//     FW_BRANCH = 1,
//     NETWORK_ID = 9,
//     DIM_TIME = 10,
//     TIME_ON = 11,
//     TIME_OFF = 12,
//     PWM_VAL = 13,
//     LED_MODE = 14,
//     LOG_INTERVAL = 20,
//     DEFAULT_POS = 30,
//     DEFAULT_SPEED = 40,

//     DOORLOCK_MODE = 50,
//     DOORLOCK_OPEN_TIME = 52,
//     DOORLOCK_CODE = 53,
//     DOORLOCK_CODE_VALID = 54,
// };
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, Hash, Eq)]
#[repr(i32)]
pub enum SettingsType {
    DevType = 0,
    FwBranch = 1,
    NetworkId = 9,
    DimTime = 10,
    TimeOn = 11,
    TimeOff = 12,
    PwmVal = 13,
    LedMode = 14,
    LogInterval = 20,
    DefaultPos = 30,
    DefaultSpeed = 40,
    DoorlockMode = 50,
    DoorlockOpenTime = 52,
    DoorlockCode = 53,
    DoorlockCodeValid = 54,
    Unknown(i32),
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, Derivative)]
#[derivative(PartialOrd, Ord, Eq)]
pub struct DevSetting {
    #[serde(rename = "updated")]
    pub updated_at: i32,
    #[serde(rename = "typeId")]
    pub settings_type: i32,
    pub channel: i32,
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    pub value: i32,
}

impl DevSetting {
    pub fn to_zephyr(&self) -> DevSettingsZephyr {
        DevSettingsZephyr {
            settings_type: i16::try_from(self.settings_type).unwrap_or(0),
            channel: i16::try_from(self.channel).unwrap_or(0),
            value: self.value,
            updated_at: self.updated_at,
        }
    }
    pub fn id(&self) -> i32 {
        self.settings_type
    }

    pub fn channel(&self) -> i32 {
        self.channel
    }
    pub fn value(&self) -> i32 {
        self.value
    }
    pub fn settings_type(&self) -> SettingsType {
        match self.settings_type {
            0 => SettingsType::DevType,
            1 => SettingsType::FwBranch,
            9 => SettingsType::NetworkId,
            10 => SettingsType::DimTime,
            11 => SettingsType::TimeOn,
            12 => SettingsType::TimeOff,
            13 => SettingsType::PwmVal,
            14 => SettingsType::LedMode,
            20 => SettingsType::LogInterval,
            30 => SettingsType::DefaultPos,
            40 => SettingsType::DefaultSpeed,
            50 => SettingsType::DoorlockMode,
            52 => SettingsType::DoorlockOpenTime,
            53 => SettingsType::DoorlockCode,
            54 => SettingsType::DoorlockCodeValid,
            n => SettingsType::Unknown(n),
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "kind", rename = "Settings")]
pub struct ApiDevSettings {
    id: String,
    updated: DateTime<Utc>,
    current_item_count: usize,
    items: Vec<DevSetting>,
}
impl ApiDevSettings {
    pub fn new(id: u64) -> Self {
        Self {
            items: vec![],
            current_item_count: 0,
            id: id.to_string(),
            updated: Utc::now(),
        }
    }
    pub fn from_vec(id: u64, items: Vec<DevSetting>) -> Self {
        let current_item_count = items.len();
        Self {
            items,
            current_item_count,
            id: id.to_string(),
            updated: Utc::now(),
        }
    }
    pub fn add_setting(&mut self, setting: DevSetting) {
        self.items.push(setting);
        self.current_item_count = self.items.len();
    }
    pub fn len(&self) -> usize {
        self.current_item_count
    }
    pub fn as_slice(&self) -> &[DevSetting] {
        &self.items
    }
    pub fn into_vec(self) -> Vec<DevSetting> {
        self.items
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "kind", rename = "SettingTypes")]
pub struct ApiSettingTypes {
    id: String,
    current_item_count: usize,
    updated: DateTime<Utc>,
    items: Vec<SettingTypes>,
}
impl ApiSettingTypes {
    pub fn new(id: u64) -> Self {
        Self {
            items: vec![],
            current_item_count: 0,
            id: id.to_string(),
            updated: Utc::now(),
        }
    }
    pub fn from_vec(id: u64, items: Vec<SettingTypes>) -> Self {
        let current_item_count = items.len();
        Self {
            items,
            current_item_count,
            id: id.to_string(),
            updated: Utc::now(),
        }
    }
    pub fn add_setting_type(&mut self, setting_type: SettingTypes) {
        self.items.push(setting_type);
        self.current_item_count = self.items.len();
    }
    pub fn len(&self) -> usize {
        self.current_item_count
    }
    pub fn as_slice(&self) -> &[SettingTypes] {
        &self.items
    }
    pub fn into_vec(self) -> Vec<SettingTypes> {
        self.items
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub struct SettingTypes {
    setting_type_id: i32,
    setting_type_name: String,
    setting_type_description: String,
    setting_type_unit: String,
    setting_type_icon: String,
    channel: i32,
    default_value: i32,
    min_value: i32,
    max_value: i32,
}
impl SettingTypes {
    pub fn new(
        setting_type_id: i32,
        channel: i32,
        default_value: i32,
        min_value: i32,
        max_value: i32,
    ) -> Self {
        Self {
            setting_type_id,
            setting_type_name: String::new(),
            setting_type_description: String::new(),
            setting_type_unit: String::new(),
            setting_type_icon: String::new(),
            channel,
            default_value,
            min_value,
            max_value,
        }
    }
    pub fn id(&self) -> i32 {
        self.setting_type_id
    }
    pub fn channel(&self) -> i32 {
        self.channel
    }
    pub fn default_value(&self) -> i32 {
        self.default_value
    }
    pub fn min_value(&self) -> i32 {
        self.min_value
    }
    pub fn max_value(&self) -> i32 {
        self.max_value
    }
    pub fn name(&self) -> &str {
        &self.setting_type_name
    }
    pub fn description(&self) -> &str {
        &self.setting_type_description
    }
    pub fn unit(&self) -> &str {
        &self.setting_type_unit
    }
    pub fn icon(&self) -> &str {
        &self.setting_type_icon
    }
    pub fn set_name(&mut self, name: String) {
        self.setting_type_name = name;
    }
    pub fn set_description(&mut self, description: String) {
        self.setting_type_description = description;
    }
    pub fn set_unit(&mut self, unit: String) {
        self.setting_type_unit = unit;
    }
    pub fn set_icon(&mut self, icon: String) {
        self.setting_type_icon = icon;
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, Encode)]
#[repr(C)]
pub struct DevSettingsZephyr {
    pub settings_type: i16,
    pub channel: i16,
    pub value: i32,
    pub updated_at: i32,
}
#[cfg(test)]
#[test]
fn order() {
    assert!(
        // Ensure that DevSetting can be ordered correctly for CoapServer settings Cache
        !(DevSetting {
            updated_at: 1691096258,
            settings_type: 14,
            channel: 3,
            value: 0
        } > DevSetting {
            updated_at: 1691096258,
            settings_type: 14,
            channel: 3,
            value: 3
        })
    );
    assert!(
        !(DevSetting {
            updated_at: 1691096258,
            settings_type: 14,
            channel: 3,
            value: 0
        } < DevSetting {
            updated_at: 1691096258,
            settings_type: 14,
            channel: 3,
            value: 3
        })
    );
}
