use base64::engine::general_purpose::STANDARD;
use base64_serde::base64_serde_type;
use chrono::{DateTime, Utc};

#[cfg(feature = "dbus")]
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

#[cfg(feature = "dbus")]
use dbus::{blocking::Connection, Error as WpanError};

#[cfg(feature = "dbus")]
use machine_uid;

#[cfg(not(feature = "dbus"))]
#[derive(Debug)]
pub struct WpanError;

#[cfg(not(feature = "dbus"))]
impl std::fmt::Display for WpanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dbus feature not enabled")
    }
}

#[cfg(not(feature = "dbus"))]
impl std::error::Error for WpanError {}

use serde::{Deserialize, Serialize};
base64_serde_type!(Base64Standard, STANDARD);

#[cfg(not(feature = "dbus"))]
use std::time::SystemTime;
#[cfg(feature = "dbus")]
use std::time::{Duration, SystemTime};

use crate::neighbors::Neighbor;
#[derive(PartialEq, Hash, Eq, Clone, Debug)]
#[allow(dead_code)]
enum WpanStatus {
    Disabled,
    Detached,
    Child,
    Router,
    Leader,
}
// wpanstatus returned from dbus as type s
#[cfg(feature = "dbus")]
impl<'a> dbus::arg::Get<'a> for WpanStatus {
    fn get(i: &mut dbus::arg::Iter<'a>) -> Option<Self> {
        i.get().and_then(|s: &str| match s {
            "disabled" => Some(WpanStatus::Disabled),
            "detached" => Some(WpanStatus::Detached),
            "child" => Some(WpanStatus::Child),
            "router" => Some(WpanStatus::Router),
            "leader" => Some(WpanStatus::Leader),
            _ => None,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OtNetwork {
    id: i32,
    kind: String,
    tlv: String,
    network_name: String,
    updated: Option<i64>,
}
#[derive(PartialEq, Hash, Eq, Clone, Debug)]
enum Tlvarray {
    D(Vec<u8>),
    None,
}

impl Serialize for Tlvarray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Tlvarray::D(tlv) => Base64Standard::serialize(&tlv, serializer),
            Tlvarray::None => None::<&str>.serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for Tlvarray {
    fn deserialize<D>(deserializer: D) -> Result<Tlvarray, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if let Ok(tlv) = Base64Standard::deserialize(deserializer) {
            Ok(Tlvarray::D(tlv))
        } else {
            Ok(Tlvarray::None)
        }
    }
}
#[derive(PartialEq, Hash, Eq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub struct OtNetConfig {
    #[serde(rename = "updated")]
    timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "networkName")]
    networkname: String, // The Thread network name.
    tlv: Tlvarray,
}

impl OtNetConfig {
    pub fn has_tlv(&self) -> bool {
        self.tlv != Tlvarray::None
    }
    pub fn has_netname(&self) -> bool {
        !self.networkname.is_empty()
    }
    pub fn new() -> OtNetConfig {
        OtNetConfig {
            timestamp: None,
            networkname: "".to_string(),
            tlv: Tlvarray::None,
        }
    }
    pub fn new_net(&self) -> (Vec<u8>, u16, String, u64, Vec<u8>, u32) {
        let netkey = Vec::new();
        let psk = Vec::new();
        let xpanid = u64::MAX;
        let panid = u16::MAX;
        let channel_mask = u32::MAX;
        (
            netkey,
            panid,
            self.networkname.clone(),
            xpanid,
            psk,
            channel_mask,
        )
    }
    pub fn set_netname(&mut self, netname: &str) {
        self.networkname = netname.to_string();
    }
    pub fn set_timestamp(mut self, ts: Option<SystemTime>) -> Self {
        if let Some(time) = ts {
            self.timestamp = Some(time.into());
        }
        self
    }
    pub fn set_tlv(&mut self, tlvs: &[u8]) {
        self.tlv = Tlvarray::D(tlvs.to_vec());
    }

    pub fn get_timestamp(&self) -> SystemTime {
        if let Some(ts) = self.timestamp {
            ts.into()
        } else {
            SystemTime::UNIX_EPOCH
        }
    }
    pub fn set_timestamp_now(&mut self) {
        self.timestamp = Some(Utc::now());
    }

    pub fn get_netname(&self) -> String {
        self.networkname.clone()
    }
    pub fn get_tlv(&self) -> Vec<u8> {
        match self.tlv {
            Tlvarray::D(ref tlv) => tlv.clone(),
            Tlvarray::None => Vec::new(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct WpanData {
    status: WpanStatus,
    rloc16: u16,
    _daemon_ver: u64,
    hwid: u64,
    neighbors: Vec<Neighbor>,
}
impl WpanData {
    pub fn new() -> WpanData {
        WpanData {
            status: WpanStatus::Disabled,
            rloc16: 0x000,
            _daemon_ver: 0,
            hwid: 0,
            neighbors: vec![],
        }
    }
    pub fn devid(&self) -> u64 {
        self.hwid
    }
    pub fn new_updated() -> Result<WpanData, WpanError> {
        let mut ret = WpanData::new();
        ret.update()?;
        Ok(ret)
    }
    pub fn update(&mut self) -> Result<(), WpanError> {
        #[cfg(feature = "dbus")]
        #[cfg(unix)]
        {
            let dbuspath = "/io/openthread/BorderRouter/wpan0";
            let conn = Connection::new_system()?;
            let proxy = conn.with_proxy(
                "io.openthread.BorderRouter.wpan0",
                dbuspath,
                Duration::from_millis(5000),
            );

            self.rloc16 = proxy.get("io.openthread.BorderRouter", "Rloc16")?;
            self.status = proxy.get("io.openthread.BorderRouter", "DeviceRole")?;
            self.neighbors = proxy.get("io.openthread.BorderRouter", "NeighborTable")?;
            self.hwid = u128::from_str_radix(&machine_uid::get().unwrap(), 16).unwrap() as u64;
            return Ok(());
        }
        #[cfg(not(feature = "dbus"))]
        {
            // When dbus is not available, return an error
            return Err(WpanError);
        }
        #[cfg(feature = "dbus")]
        #[cfg(not(unix))]
        {
            // When dbus is available but not on unix, return an error
            return Err(WpanError);
        }
    }
    pub fn is_connected(&self) -> bool {
        match self.status {
            WpanStatus::Disabled => false,
            WpanStatus::Detached => false,
            WpanStatus::Child => true,
            WpanStatus::Router => true,
            WpanStatus::Leader => true,
        }
    }
}
