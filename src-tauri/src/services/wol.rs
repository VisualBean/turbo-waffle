use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use thiserror::Error;
use wake_on_lan::MagicPacket;

#[derive(Error, Debug)]
pub enum WolError {
    #[error("Invalid MAC address format")]
    InvalidMacAddress,
    #[error("Could not resolve MAC address for host")]
    MacNotFound,
    #[error("Failed to create magic packet: {0}")]
    PacketCreation(String),
    #[error("Failed to send packet: {0}")]
    SendFailed(#[from] std::io::Error),
}

pub fn send_wol_packet(mac_address: &str, broadcast_addr: Option<&str>) -> Result<(), WolError> {
    let mac_bytes = parse_mac_address(mac_address)?;
    send_wol_packet_bytes(&mac_bytes, broadcast_addr)
}

pub fn send_wol_for_host(host: &str, broadcast_addr: Option<&str>) -> Result<(), WolError> {
    let mac = lookup_mac_address(host)?;
    let mac_bytes = parse_mac_address(&mac)?;
    send_wol_packet_bytes(&mac_bytes, broadcast_addr)
}

fn send_wol_packet_bytes(mac_bytes: &[u8; 6], broadcast_addr: Option<&str>) -> Result<(), WolError> {
    let magic_packet = MagicPacket::new(mac_bytes);

    let broadcast = broadcast_addr.unwrap_or("255.255.255.255");
    let target: SocketAddr = format!("{}:9", broadcast)
        .parse()
        .map_err(|e| WolError::PacketCreation(format!("Invalid broadcast address: {}", e)))?;

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.send_to(magic_packet.magic_bytes(), target)?;

    Ok(())
}

pub fn lookup_mac_address(host: &str) -> Result<String, WolError> {
    let ip = resolve_to_ip(host)?;

    #[cfg(target_os = "linux")]
    {
        lookup_mac_linux(&ip)
    }

    #[cfg(target_os = "macos")]
    {
        lookup_mac_macos(&ip)
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        Err(WolError::MacNotFound)
    }
}

fn resolve_to_ip(host: &str) -> Result<String, WolError> {
    let addr = format!("{}:0", host);
    addr.to_socket_addrs()
        .map_err(|_| WolError::MacNotFound)?
        .next()
        .map(|a| a.ip().to_string())
        .ok_or(WolError::MacNotFound)
}

#[cfg(target_os = "linux")]
fn lookup_mac_linux(ip: &str) -> Result<String, WolError> {
    let contents = std::fs::read_to_string("/proc/net/arp").map_err(|_| WolError::MacNotFound)?;

    for line in contents.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 && parts[0] == ip {
            let mac = parts[3];
            if mac != "00:00:00:00:00:00" {
                return Ok(mac.to_uppercase());
            }
        }
    }

    Err(WolError::MacNotFound)
}

#[cfg(target_os = "macos")]
fn lookup_mac_macos(ip: &str) -> Result<String, WolError> {
    use std::process::Command;
    let output = Command::new("arp")
        .args(["-n", ip])
        .output()
        .map_err(|_| WolError::MacNotFound)?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains(ip) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for part in parts {
                if part.contains(':') && part.len() >= 11 {
                    return Ok(part.to_uppercase());
                }
            }
        }
    }

    Err(WolError::MacNotFound)
}

fn parse_mac_address(mac: &str) -> Result<[u8; 6], WolError> {
    let mac_clean = mac.replace([':', '-', '.'], "");

    if mac_clean.len() != 12 {
        return Err(WolError::InvalidMacAddress);
    }

    let mut bytes = [0u8; 6];
    for (i, byte) in bytes.iter_mut().enumerate() {
        let start = i * 2;
        *byte = u8::from_str_radix(&mac_clean[start..start + 2], 16)
            .map_err(|_| WolError::InvalidMacAddress)?;
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mac_address() {
        // arrange
        let mac = "AA:BB:CC:DD:EE:FF";

        // act
        let result = parse_mac_address(mac);

        // assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    }

    #[test]
    fn test_parse_mac_address_dashes() {
        // arrange
        let mac = "AA-BB-CC-DD-EE-FF";

        // act
        let result = parse_mac_address(mac);

        // assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_mac_address_invalid() {
        // arrange
        let mac = "invalid";

        // act
        let result = parse_mac_address(mac);

        // assert
        assert!(result.is_err());
    }
}
