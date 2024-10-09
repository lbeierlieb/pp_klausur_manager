use serde::Deserialize;
use std::{
    fs::{self, read_to_string},
    io::Write,
    net::{Ipv4Addr, ToSocketAddrs},
    path::Path,
};

use crate::client::Client;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub timer_port: u16,
    pub timer_duration_minutes: i64,
    pub timer_webpage_refresh_seconds: u32,
    pub timer_webpage_refresh_unstarted_seconds: u32,
    pub timer_allow_nonclient_access: bool,
    pub kanata_client_scan_interval_seconds: u64,
    pub kanata_tcp_timeout_ms: u64,
    pub kanata_port: u16,
    pub tui_show_nonclient_timer_accesses: bool,
    rooms: Vec<Room>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    name: String,
    domain: String,
    symlink_info: SymlinkInfo,
    client_hostnames: Vec<String>,
    control_client: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SymlinkInfo {
    pub symlink_path: String,
    pub dummy_target: String,
    pub real_target: String,
}

pub fn get_rooms(config: &Config) -> Vec<&str> {
    config.rooms.iter().map(|room| room.name.as_str()).collect()
}

pub fn room_exists(room_name: &str, config: &Config) -> bool {
    config.rooms.iter().any(|room| room.name == room_name)
}

pub fn get_ip_addresses_of_room(room_name: &str, config: &Config) -> Option<Vec<Client>> {
    let rooms_with_name = config
        .rooms
        .iter()
        .filter(|room| room.name == room_name)
        .collect::<Vec<_>>();
    if rooms_with_name.len() == 1 {
        let room = rooms_with_name[0];
        let hostnames = room
            .client_hostnames
            .iter()
            .filter(|hostname| **hostname != room.control_client)
            .map(|hostname| hostname.to_owned())
            .collect::<Vec<_>>();
        let ips = hostnames
            .into_iter()
            .map(|hostname_short| {
                resolve_ipv4_addr(&format!("{}{}", hostname_short, &room.domain))
                    .map(|ip| (hostname_short, ip))
            })
            .collect::<Option<Vec<_>>>()?;
        let clients = ips
            .into_iter()
            .map(|(name, ip)| Client::new(name, ip))
            .collect();
        Some(clients)
    } else {
        None
    }
}

pub fn get_symlink_info_of_room(room_name: &str, config: &Config) -> Option<SymlinkInfo> {
    let rooms_with_name = config
        .rooms
        .iter()
        .filter(|room| room.name == room_name)
        .collect::<Vec<_>>();
    if rooms_with_name.len() == 1 {
        Some(rooms_with_name[0].symlink_info.clone())
    } else {
        None
    }
}

pub fn parse_config(config_path: &str) -> Option<Config> {
    let filecontent = read_to_string(config_path).ok()?;
    let config: Config = serde_json::from_str(&filecontent).ok()?;
    for room in &config.rooms {
        if !room.client_hostnames.contains(&room.control_client) {
            return None;
        }
    }
    Some(config)
}

fn resolve_ipv4_addr(hostname: &str) -> Option<Ipv4Addr> {
    let hostname = format!("{}:80", hostname);
    let addresses = hostname.to_socket_addrs().unwrap();

    for addr in addresses {
        if let std::net::SocketAddr::V4(socket_addr) = addr {
            return Some(socket_addr.ip().clone());
        }
    }
    None
}

pub fn create_default_config_if_necessary(path: &str, default_config_content: &str) -> Option<()> {
    if !Path::new(path).exists() {
        println!("No config file was found at path '{}'", path);
        let mut file = fs::File::create(path).ok()?;
        file.write_all(default_config_content.as_bytes()).ok()?;
        println!("Default config was created at path '{}'", path);
    }
    Some(())
}
