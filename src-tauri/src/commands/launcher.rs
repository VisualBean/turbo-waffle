use crate::models::ConnectionConfig;
use crate::services::open_ssh_in_terminal;
use crate::storage::ConfigStorage;
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;

#[tauri::command]
pub async fn open_connection(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let storage = ConfigStorage::new().map_err(|e| e.to_string())?;

    let connection = storage
        .get_connection(uuid)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Connection not found".to_string())?;

    match &connection.config {
        ConnectionConfig::Website { url, .. } => {
            app.opener()
                .open_url(url, None::<&str>)
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        ConnectionConfig::Ssh {
            host,
            port,
            username,
            ..
        } => open_ssh_in_terminal(username, host, *port).map_err(|e| e.to_string()),
    }
}
