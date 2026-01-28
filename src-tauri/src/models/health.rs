use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Online,
    Offline,
    Degraded,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResult {
    pub connection_id: Uuid,
    pub status: HealthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub checked_at: String,
}

impl HealthResult {
    pub fn online(connection_id: Uuid, latency_ms: u64) -> Self {
        Self {
            connection_id,
            status: HealthStatus::Online,
            latency_ms: Some(latency_ms),
            error: None,
            checked_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn offline(connection_id: Uuid, error: Option<String>) -> Self {
        Self {
            connection_id,
            status: HealthStatus::Offline,
            latency_ms: None,
            error,
            checked_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn unknown(connection_id: Uuid) -> Self {
        Self {
            connection_id,
            status: HealthStatus::Unknown,
            latency_ms: None,
            error: None,
            checked_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
