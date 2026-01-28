use crate::models::Connection;
use crate::storage::ConfigStorage;
use uuid::Uuid;

#[tauri::command]
pub fn get_connections() -> Result<Vec<Connection>, String> {
    let storage = ConfigStorage::new().map_err(|e| e.to_string())?;
    storage.load_connections().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_connection(connection: Connection) -> Result<(), String> {
    let storage = ConfigStorage::new().map_err(|e| e.to_string())?;
    storage.save_connection(connection).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_connection(id: String) -> Result<bool, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let storage = ConfigStorage::new().map_err(|e| e.to_string())?;
    storage.delete_connection(uuid).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reorder_connections(ids: Vec<String>) -> Result<(), String> {
    let storage = ConfigStorage::new().map_err(|e| e.to_string())?;
    let mut connections = storage.load_connections().map_err(|e| e.to_string())?;

    for (index, id) in ids.iter().enumerate() {
        let uuid = Uuid::parse_str(id).map_err(|e| e.to_string())?;
        if let Some(conn) = connections.iter_mut().find(|c| c.id == uuid) {
            conn.order = index as i32;
        }
    }

    storage.save_connections(&connections).map_err(|e| e.to_string())
}
