use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyGroup {
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String,
    pub now: Option<String>,
    pub all: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub history: Option<Vec<DelayHistory>>,
    pub alive: Option<bool>,
    pub now: Option<String>,
    pub all: Option<Vec<String>>,
    pub udp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelayHistory {
    pub time: String,
    pub delay: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxiesResponse {
    pub proxies: HashMap<String, Proxy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchRequest {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum TrayEvent {
    SwitchNode { group: String, node: String },
    Refresh,
    Quit,
}
