use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Router {
    id: u64,
    pub name: Option<String>,
    pub uptime: Option<u32>,
    pub last_active: SystemTime,
}
impl Router {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            name: None,
            uptime: None,
            last_active: SystemTime::now(),
        }
    }
    pub fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn dev_id(&self) -> String {
        format!("{:#08x}", self.id)
    }
}
