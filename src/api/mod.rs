pub mod post;
use crate::devs::hb;
use crate::measurement::Measurement;
use crate::neighbors::Neighbor;
use crate::otnet::OtNetConfig;
use crate::otnet::OtNetwork;
use crate::settings::DevSetting;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
enum ItemTypes {
    Setting(DevSetting),
    OtNet(OtNetwork),
    HeartBeat(hb::HeartBeat),
    Measurement(Measurement),
    Neighbor(Vec<Neighbor>),
    OtNetConfig(OtNetConfig),
}
