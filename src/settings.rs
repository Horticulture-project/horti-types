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
    #[cfg(test)]
    pub fn to_zephyr(&self) -> DevSettingsZephyr {
        DevSettingsZephyr {
            settings_type: i16::try_from(self.settings_type).unwrap_or(0),
            channel: i16::try_from(self.channel).unwrap_or(0),
            value: self.value,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct DevSettingsZephyr {
    pub settings_type: i16,
    pub channel: i16,
    pub value: i32,
    pub updated_at: i32,
}

// horti_db=# select * from settings ;
//  device_id  | setting_type_id | i2c | channel | value | created_at | updated_at
// ------------+-----------------+-----+---------+-------+------------+------------
//  3203386110 |               1 |   0 |       0 |   255 |            |
//  3203386110 |               2 |   0 |       0 |   600 |            |
//  3203386110 |               3 |   0 |       0 |  6000 |            |
//  3203386110 |               7 |   0 |       0 |     1 |            |
// (4 rows)
// horti_db=# select * from setting_types ;
//  id |      description
// ----+-----------------------
//   0 | settins-rev
//   1 | PWM Value
//   2 | Time-ON
//   3 | Time-OFF
//   4 | Dim-Time/Log interval
//   5 | LED-State
//   6 | Follow-leader
//   7 | FW-Branch
// (7 rows)

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
