use crate::models::HealthResult;
use crate::services::check_connection_health;
use crate::storage::ConfigStorage;
use uuid::Uuid;

#[tauri::command]
pub async fn check_health(id: String) -> Result<HealthResult, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let storage = ConfigStorage::new().map_err(|e| e.to_string())?;

    let connection = storage
        .get_connection(uuid)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Connection not found".to_string())?;

    Ok(check_connection_health(&connection).await)
}

#[tauri::command]
pub async fn check_all_health() -> Result<Vec<HealthResult>, String> {
    let storage = ConfigStorage::new().map_err(|e| e.to_string())?;
    let connections = storage.load_connections().map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for connection in connections {
        results.push(check_connection_health(&connection).await);
    }

    Ok(results)
}
