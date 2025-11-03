use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JoinerData {
    euid64: Option<u64>,
    pskd: Option<String>,
}
impl JoinerData {
    pub fn new(euid64: Option<u64>, pskd: Option<String>) -> Self {
        JoinerData { euid64, pskd }
    }
    pub fn euid64(&self) -> Option<u64> {
        self.euid64
    }
    pub fn pskd(&self) -> Option<&String> {
        self.pskd.as_ref()
    }
    pub fn json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}
impl From<&str> for JoinerData {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).unwrap_or_else(|_| JoinerData::new(None, None))
    }
}
