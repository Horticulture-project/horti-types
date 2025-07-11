use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LedPanel {
    id: u64,
    pub name: Option<String>,
    pub uptime: Option<u32>,
    pub last_active: DateTime<Utc>,
    pub fwver: Option<u32>,
}
impl LedPanel {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            name: None,
            uptime: None,
            last_active: Utc::now(),
            fwver: None,
        }
    }
}
impl super::Dev for LedPanel {
    fn dev_sn(&self) -> u64 {
        self.id
    }
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }

    fn last_active(&self) -> DateTime<Utc> {
        self.last_active
    }
    fn dev_type(&self) -> &'static str {
        "LED Panel"
    }

    fn fwver(&self) -> Option<[u8; 4]> {
        self.fwver.map(|v| v.to_be_bytes())
    }
}
