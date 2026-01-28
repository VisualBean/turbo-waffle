use crate::models::{Connection, ConnectionConfig, HealthResult};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;

const TIMEOUT_SECS: u64 = 5;

pub async fn check_connection_health(connection: &Connection) -> HealthResult {
    match &connection.config {
        ConnectionConfig::Website { url, check_path } => {
            check_http_health(connection.id, url, check_path.as_deref()).await
        }
        ConnectionConfig::Ssh { host, port, .. } => {
            check_tcp_health(connection.id, host, *port).await
        }
    }
}

async fn check_http_health(
    connection_id: uuid::Uuid,
    base_url: &str,
    check_path: Option<&str>,
) -> HealthResult {
    let url = if let Some(path) = check_path {
        format!("{}{}", base_url.trim_end_matches('/'), path)
    } else {
        base_url.to_string()
    };

    let start = Instant::now();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .danger_accept_invalid_certs(true)
        .build();

    let client = match client {
        Ok(c) => c,
        Err(e) => return HealthResult::offline(connection_id, Some(e.to_string())),
    };

    match client.get(&url).send().await {
        Ok(response) => {
            let latency = start.elapsed().as_millis() as u64;
            if response.status().is_success() || response.status().is_redirection() {
                HealthResult::online(connection_id, latency)
            } else {
                HealthResult::offline(
                    connection_id,
                    Some(format!("HTTP {}", response.status().as_u16())),
                )
            }
        }
        Err(e) => HealthResult::offline(connection_id, Some(e.to_string())),
    }
}

async fn check_tcp_health(connection_id: uuid::Uuid, host: &str, port: u16) -> HealthResult {
    let addr_str = format!("{}:{}", host, port);

    let addr: SocketAddr = match addr_str.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr,
            None => {
                return HealthResult::offline(connection_id, Some("Could not resolve host".into()))
            }
        },
        Err(e) => return HealthResult::offline(connection_id, Some(e.to_string())),
    };

    let start = Instant::now();

    match timeout(Duration::from_secs(TIMEOUT_SECS), TcpStream::connect(addr)).await {
        Ok(Ok(_)) => {
            let latency = start.elapsed().as_millis() as u64;
            HealthResult::online(connection_id, latency)
        }
        Ok(Err(e)) => HealthResult::offline(connection_id, Some(e.to_string())),
        Err(_) => HealthResult::offline(connection_id, Some("Connection timed out".into())),
    }
}
