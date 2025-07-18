use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Router {
    id: u64,
    pub name: Option<String>,
    pub uptime: Option<u32>,
    pub last_active: DateTime<Utc>,
    pub fwver: Option<u32>,
    pub fwver_name: Option<String>,
    #[serde(default)]
    pub status: crate::devs::hb::DevStatus,
}
impl Router {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            name: None,
            uptime: None,
            last_active: DateTime::from_timestamp(0, 0).unwrap_or_else(|| Utc::now()),
            fwver: None,
            fwver_name: None,
            status: crate::devs::hb::DevStatus::Unknown(0),
        }
    }
    pub fn last_active(&self) -> DateTime<Utc> {
        self.last_active
    }
}
impl super::Dev for Router {
    fn dev_sn(&self) -> u64 {
        self.id
    }
    fn display_name(&self) -> String {
        match self.name() {
            Some(name) => name.to_string(),
            None => self.dev_id(),
        }
    }
    fn last_active(&self) -> DateTime<Utc> {
        self.last_active
    }
    fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    fn dev_type(&self) -> &'static str {
        "Router"
    }

    fn fwver(&self) -> Option<[u8; 4]> {
        self.fwver.map(|v| v.to_be_bytes())
    }
    fn fwver_name(&self) -> Option<String> {
        self.fwver_name.clone()
    }
    fn status(&self) -> crate::devs::hb::DevStatus {
        self.status
    }
}
