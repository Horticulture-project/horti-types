use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LedPanel {
    id: u64,
    pub name: Option<String>,
    pub uptime: Option<u32>,
    pub last_active: SystemTime,
}
impl LedPanel {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            name: None,
            uptime: None,
            last_active: SystemTime::now(),
        }
    }
}
impl super::Dev for LedPanel {
    fn dev_id(&self) -> String {
        format!("{:#08x}", self.id)
    }
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }

    fn last_active(&self) -> SystemTime {
        self.last_active
    }
}
