use serde::Deserialize;
use std::{
    fs::read_to_string,
    net::{Ipv4Addr, ToSocketAddrs},
};

use crate::client::Client;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub timer_port: u16,
    pub timer_duration_minutes: i64,
    pub timer_webpage_refresh_seconds: u32,
    pub timer_webpage_refresh_unstarted_seconds: u32,
    pub kanata_client_scan_interval_seconds: u64,
    pub kanata_tcp_timeout_ms: u64,
    pub kanata_port: u16,
    rooms: Vec<Room>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    name: String,
    domain: String,
    client_hostnames: Vec<String>,
}

pub fn get_ip_addresses_of_room(room_name: &str, config: &Config) -> Option<Vec<Client>> {
    let rooms_with_name = config
        .rooms
        .iter()
        .filter(|room| room.name == room_name)
        .collect::<Vec<_>>();
    if rooms_with_name.len() == 1 {
        let room = rooms_with_name[0];
        let hostnames = room.client_hostnames.clone();
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

pub fn parse_config(config_path: &str) -> Option<Config> {
    let filecontent = read_to_string(config_path).ok()?;
    serde_json::from_str(&filecontent).ok()
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
