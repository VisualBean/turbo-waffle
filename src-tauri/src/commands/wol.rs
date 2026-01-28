use crate::models::ConnectionConfig;
use crate::services::{lookup_mac_address, send_wol_packet};
use crate::storage::ConfigStorage;
use uuid::Uuid;

#[tauri::command]
pub fn send_wol(id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let storage = ConfigStorage::new().map_err(|e| e.to_string())?;

    let connection = storage
        .get_connection(uuid)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Connection not found".to_string())?;

    match &connection.config {
        ConnectionConfig::Ssh {
            wol_enabled,
            mac_address,
            broadcast_addr,
            ..
        } => {
            if !wol_enabled {
                return Err("Wake-on-LAN is not enabled for this connection".to_string());
            }

            let mac = mac_address
                .as_ref()
                .ok_or_else(|| "No MAC address configured".to_string())?;

            send_wol_packet(mac, broadcast_addr.as_deref()).map_err(|e| e.to_string())
        }
        _ => Err("Wake-on-LAN is only supported for SSH connections".to_string()),
    }
}

#[tauri::command]
pub fn lookup_mac(host: String) -> Result<String, String> {
    lookup_mac_address(&host).map_err(|e| e.to_string())
}
