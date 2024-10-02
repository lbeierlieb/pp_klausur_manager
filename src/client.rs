use std::{net::Ipv4Addr, sync::Mutex};

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Client {
    pub name: String,
    pub ip_address: Ipv4Addr,
    pub current_layer: Mutex<Option<String>>,
    pub last_timer_access: Mutex<Option<DateTime<Utc>>>,
}

impl Client {
    pub fn new(name: String, ip: Ipv4Addr) -> Self {
        Client {
            name,
            ip_address: ip,
            current_layer: Mutex::new(None),
            last_timer_access: Mutex::new(None),
        }
    }
}
