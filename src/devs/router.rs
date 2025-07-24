use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Router {
    id: u64,
    name: Option<String>,
    uptime: Option<u32>,
    last_active: DateTime<Utc>,
    fwver: Option<u32>,
    fwver_name: Option<String>,
    status: crate::devs::hb::DevStatus,
}
impl Router {
    pub fn new(
        id: u64,
        name: Option<String>,
        uptime: Option<u32>,
        last_active: DateTime<Utc>,
        fwver: Option<u32>,
        fwver_name: Option<String>,
        status: crate::devs::hb::DevStatus,
    ) -> Self {
        Self {
            id,
            name,
            uptime,
            last_active,
            fwver,
            fwver_name,
            status,
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
        self.status.map_active(self.last_active)
    }
}
