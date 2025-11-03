#[cfg(feature = "dbus")]
use dbus::{arg::ArgType, Signature};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::devs::{self};

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Debug, Clone)]
pub struct Neighbor {
    pub rloc16: u16,
    #[serde(rename = "rssi")]
    pub m_last_rssi: i8,
    #[serde(rename = "mLinkQuality")]
    pub m_link_quality: u8,
    #[serde(rename = "mAverageRssi")]
    pub m_average_rssi: i8,
    #[serde(rename = "rxOnIdle")]
    pub rx_on_idle: bool,
    pub child: bool,
    pub ftd: bool,
    pub fnd: bool,
}
impl Display for Neighbor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Neighbor Rloc: {:#06x}, Rssi: {}, LinkQuality: {}, Child: {}, SED: {}, FTD: {}, FND: {} ",
            self.rloc16, self.m_average_rssi, self.m_link_quality, self.child, !self.rx_on_idle, self.ftd, self.fnd
        )
    }
}
#[allow(dead_code)]
impl Neighbor {
    pub fn new() -> Neighbor {
        Neighbor {
            rloc16: 0,
            m_link_quality: 0,
            m_last_rssi: 0,
            m_average_rssi: 0,
            rx_on_idle: true,
            child: false,
            ftd: true,
            fnd: false,
        }
    }
    pub fn rloc16(&mut self, rloc16: u16) {
        self.rloc16 = rloc16;
    }
    fn age(&mut self, _age: u32) {
        // self.m_age = age;
    }
    fn ext_address(&mut self, _ext: u64) {
        // self.m_ext_address = ext;
    }
    fn link_frame_counter(&mut self, _lfc: u32) {
        // self.m_link_frame_counter = lfc;
    }
    fn mle_frame_counter(&mut self, _mfc: u32) {
        // self.m_mle_frame_counter = mfc;
    }
    fn link_quality(&mut self, lq: u8) {
        self.m_link_quality = lq;
    }
    fn avg_rssi(&mut self, rssi: i8) {
        self.m_average_rssi = rssi;
    }
    fn last_rssi(&mut self, rssi: i8) {
        self.m_last_rssi = rssi;
    }
    fn frame_error_rate(&mut self, _fer: u16) {
        // self.m_frame_error_rate = fer;
    }
    fn message_error_rate(&mut self, _mer: u16) {
        // self.m_message_error_rate = mer;
    }
    fn version(&mut self, _ver: u16) {
        // self.m_version = ver;
    }
    fn rx_on_when_idle(&mut self, rx_on: bool) {
        self.rx_on_idle = rx_on;
    }
    fn full_thread_device(&mut self, ftd: bool) {
        self.ftd = ftd;
    }
    fn full_network_data(&mut self, fnd: bool) {
        self.fnd = fnd;
    }
    fn is_child(&mut self, child: bool) {
        self.child = child;
    }
    pub fn from_payload(payload: &[u8]) -> Option<Neighbor> {
        if payload.len() == size_of::<NeighborDataZephyr>() {
            Some(Neighbor {
                rloc16: u16::from_le_bytes(payload[0..2].try_into().ok()?),
                m_link_quality: u8::from_le_bytes([payload[2]]),
                m_last_rssi: i8::from_le_bytes([payload[3]]),
                m_average_rssi: i8::from_le_bytes([payload[4]]),
                rx_on_idle: (payload[5] & 0x1) != 0, // Bool
                child: (payload[5] & 0x2) != 0,      // Bool
                ftd: (payload[5] & 0x4) != 0,        // Bool
                fnd: (payload[5] & 0x8) != 0,        // Bool
            })
        } else {
            None
        }
    }
}
pub fn neighbors_from_payload(payload: &[u8]) -> Result<Vec<Neighbor>, &str> {
    if payload.len() % size_of::<NeighborDataZephyr>() != 0 {
        return Err("Invalid package length for Neighbordata");
    }
    let ret = payload
        .chunks(size_of::<NeighborDataZephyr>())
        .map(|n| Neighbor::from_payload(n).ok_or("Unable to parse Neighbor from package"))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ret)
}

#[cfg(feature = "dbus")]
impl<'a> dbus::arg::Get<'a> for Neighbor {
    fn get(i: &mut dbus::arg::Iter<'a>) -> Option<Self> {
        if let Some(i) = i.recurse(ArgType::Struct) {
            let mut ret = Neighbor::new();
            for (n, itm) in i.enumerate() {
                match (n, itm) {
                    (0, m_ext_address) => ret.ext_address(m_ext_address.as_u64()?),
                    (1, m_age) => ret.age(m_age.as_u64()? as u32),
                    (2, m_rloc16) => ret.rloc16(m_rloc16.as_u64()? as u16),
                    (3, m_link_frame_counter) => {
                        ret.link_frame_counter(m_link_frame_counter.as_u64()? as u32)
                    }
                    (4, m_mle_frame_counter) => {
                        ret.mle_frame_counter(m_mle_frame_counter.as_u64()? as u32)
                    }
                    (5, m_link_quality_in) => ret.link_quality(m_link_quality_in.as_u64()? as u8),
                    (6, m_average_rssi) => ret.avg_rssi(m_average_rssi.as_i64()? as i8),
                    (7, m_last_rssi) => ret.last_rssi(m_last_rssi.as_i64()? as i8),
                    (8, m_frame_error_rate) => {
                        ret.frame_error_rate(m_frame_error_rate.as_u64()? as u16)
                    }
                    (9, m_message_error_rate) => {
                        ret.message_error_rate(m_message_error_rate.as_u64()? as u16)
                    }
                    (10, m_version) => ret.version(m_version.as_u64()? as u16),
                    (11, m_rx_on_when_idle) => {
                        ret.rx_on_when_idle(m_rx_on_when_idle.as_u64()? != 0)
                    }
                    (12, m_full_thread_device) => {
                        ret.full_thread_device(m_full_thread_device.as_u64()? != 0)
                    }
                    (13, m_full_network_data) => {
                        ret.full_network_data(m_full_network_data.as_u64()? != 0)
                    }
                    (14, m_is_child) => ret.is_child(m_is_child.as_u64()? != 0),
                    _ => return None,
                };
            }
            Some(ret)
        } else {
            None
        }
    }
}

#[cfg(feature = "dbus")]
impl dbus::arg::Arg for Neighbor {
    const ARG_TYPE: ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        Signature::from("(tuquuyyyqqqbbbb)")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Debug, Clone)]
#[repr(C)]
pub struct NeighborDataZephyr {
    pub mrloc16: u16,
    pub m_link_quality: u8,
    pub m_last_rssi: i8,
    pub m_average_rssi: i8,
    pub bools: u8,
    pub pad1: u16,
}
#[cfg(test)]
#[allow(clippy::too_many_arguments)]
impl NeighborDataZephyr {
    pub fn new_from(
        rloc16: u16,
        m_last_rssi: i8,
        m_link_quality: u8,
        m_average_rssi: i8,
        rx_on_idle: bool,
        child: bool,
        ftd: bool,
        fnd: bool,
    ) -> NeighborDataZephyr {
        NeighborDataZephyr {
            mrloc16: u16::try_from(rloc16).unwrap_or(0),
            m_link_quality: u8::try_from(m_link_quality).unwrap_or(0),
            m_last_rssi: i8::try_from(m_last_rssi).unwrap_or(0),
            m_average_rssi: i8::try_from(m_average_rssi).unwrap_or(0),
            bools: (u8::from(fnd) << 3
                | u8::from(ftd) << 2
                | u8::from(child) << 1
                | u8::from(rx_on_idle)),
            pad1: 0,
        }
    }
    unsafe fn as_u8_slice(&self) -> &[u8] {
        ::core::slice::from_raw_parts(
            (self as *const Self) as *const u8,
            ::core::mem::size_of::<Self>(),
        )
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct NetGraphEdge {
    pub source: usize,
    pub dest: usize,
    pub rssi: i32,
    pub link_quality: i32,
    pub rx_on_idle: bool,
    pub child: bool,
    pub ftd: bool,
    pub fnd: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct NetGraph {
    pub edges: Vec<NetGraphEdge>,
    pub nodes: Vec<devs::DevInfo>,
}
impl Into<(Vec<NetGraphEdge>, Vec<devs::DevInfo>)> for NetGraph {
    fn into(self) -> (Vec<NetGraphEdge>, Vec<devs::DevInfo>) {
        (self.edges, self.nodes)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Neighbor", rename_all = "camelCase", tag = "kind")]
pub struct ApiNeighbors {
    id: String,
    current_item_count: usize,
    items: Vec<Neighbor>,
}
impl ApiNeighbors {
    pub fn new(nd: &[Neighbor], devid: u64) -> Self {
        Self {
            id: devid.to_string(),
            current_item_count: nd.len(),
            items: nd.to_vec(),
        }
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn as_slice(&self) -> &[Neighbor] {
        &self.items
    }
    pub fn into_vec(self) -> Vec<Neighbor> {
        self.items
    }
}

#[cfg(test)]
mod test {
    use super::{Neighbor, NeighborDataZephyr};

    #[test]
    fn from_zephyr_payload() {
        let pkg = NeighborDataZephyr::new_from(1, 3, 5, 3, true, false, false, false);
        let payload = unsafe {
            // Booooo Boooo!
            pkg.as_u8_slice()
        };
        let neigh = Neighbor::from_payload(payload).unwrap();
        println!("neigh: {:?}", neigh);
        assert_eq!(neigh, neigh);
    }
}
