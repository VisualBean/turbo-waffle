use crate::models::Connection;
use chrono::Utc;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to get config directory")]
    NoConfigDir,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct ConfigStorage {
    config_path: PathBuf,
}

impl ConfigStorage {
    pub fn new() -> Result<Self, StorageError> {
        let project_dirs =
            ProjectDirs::from("com", "turbowaffle", "turbo-waffle").ok_or(StorageError::NoConfigDir)?;
        let config_dir = project_dirs.config_dir();

        fs::create_dir_all(config_dir)?;

        let config_path = config_dir.join("connections.json");

        Ok(Self { config_path })
    }

    pub fn load_connections(&self) -> Result<Vec<Connection>, StorageError> {
        if !self.config_path.exists() {
            return Ok(Vec::new());
        }

        let contents = fs::read_to_string(&self.config_path)?;

        match serde_json::from_str::<Vec<Connection>>(&contents) {
            Ok(connections) => Ok(connections),
            Err(_) => {
                let values: Vec<serde_json::Value> = serde_json::from_str(&contents)?;
                let mut connections = Vec::new();
                let mut invalid = Vec::new();

                for value in values {
                    match serde_json::from_value::<Connection>(value.clone()) {
                        Ok(connection) => connections.push(connection),
                        Err(_) => invalid.push(value),
                    }
                }

                if !invalid.is_empty() {
                    let backup_path = self.config_path.with_file_name(format!(
                        "connections.invalid-{}.json",
                        Utc::now().format("%Y%m%d%H%M%S")
                    ));
                    if let Ok(contents) = serde_json::to_string_pretty(&invalid) {
                        let _ = fs::write(backup_path, contents);
                    }
                }

                Ok(connections)
            }
        }
    }

    pub fn save_connections(&self, connections: &[Connection]) -> Result<(), StorageError> {
        let contents = serde_json::to_string_pretty(connections)?;
        fs::write(&self.config_path, contents)?;
        Ok(())
    }

    pub fn get_connection(&self, id: Uuid) -> Result<Option<Connection>, StorageError> {
        let connections = self.load_connections()?;
        Ok(connections.into_iter().find(|c| c.id == id))
    }

    pub fn save_connection(&self, connection: Connection) -> Result<(), StorageError> {
        let mut connections = self.load_connections()?;

        if let Some(idx) = connections.iter().position(|c| c.id == connection.id) {
            connections[idx] = connection;
        } else {
            connections.push(connection);
        }

        self.save_connections(&connections)
    }

    pub fn delete_connection(&self, id: Uuid) -> Result<bool, StorageError> {
        let mut connections = self.load_connections()?;
        let original_len = connections.len();
        connections.retain(|c| c.id != id);

        if connections.len() != original_len {
            self.save_connections(&connections)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
