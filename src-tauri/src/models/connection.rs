use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ConnectionConfig {
    Website {
        url: String,
        #[serde(rename = "checkPath", skip_serializing_if = "Option::is_none")]
        check_path: Option<String>,
    },
    Ssh {
        host: String,
        port: u16,
        username: String,
        #[serde(rename = "wolEnabled", default)]
        wol_enabled: bool,
        #[serde(rename = "macAddress", skip_serializing_if = "Option::is_none")]
        mac_address: Option<String>,
        #[serde(rename = "broadcastAddr", skip_serializing_if = "Option::is_none")]
        broadcast_addr: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<String>,
    #[serde(default)]
    pub order: i32,
    pub config: ConnectionConfig,
    pub created_at: String,
    pub updated_at: String,
}

impl Connection {
    pub fn new(name: String, config: ConnectionConfig, order: i32) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4(),
            name,
            icon: None,
            icon_color: None,
            order,
            config,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
