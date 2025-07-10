use bincode::Encode;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, Derivative)]
#[derivative(PartialOrd, Ord, Eq)]
pub struct DevSetting {
    #[serde(rename = "updated")]
    pub updated_at: i32,
    #[serde(rename = "typeId")]
    pub settings_type: i64,
    pub channel: i32,
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    pub value: i32,
}

impl DevSetting {
    #[allow(dead_code)]
    pub fn to_zephyr(&self) -> DevSettingsZephyr {
        DevSettingsZephyr {
            settings_type: i16::try_from(self.settings_type).unwrap_or(0),
            channel: i16::try_from(self.channel).unwrap_or(0),
            value: self.value,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, Encode)]
pub struct DevSettingsZephyr {
    pub settings_type: i16,
    pub channel: i16,
    pub value: i32,
    pub updated_at: i32,
}

////////////////////////////////////////////////////////TEST///////////////////////////////////////////////////////

#[cfg(test)]
#[test]
fn order() {
    assert!(
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
