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
            last_active: SystemTime::UNIX_EPOCH,
        }
    }
    pub fn last_active(&self) -> SystemTime {
        self.last_active
    }
}
impl super::Dev for Router {
    fn dev_id(&self) -> String {
        format!("{:#08x}", self.id)
    }
    fn display_name(&self) -> String {
        match self.name() {
            Some(name) => name.to_string(),
            None => self.dev_id(),
        }
    }
    fn last_active(&self) -> SystemTime {
        self.last_active
    }
    fn uptime(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.uptime? as u64))
    }
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
