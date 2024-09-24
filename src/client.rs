use std::{
    net::Ipv4Addr,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Client {
    pub ip_address: Ipv4Addr,
    pub current_layer: Arc<Mutex<Option<String>>>,
    pub last_timer_access: Mutex<Option<DateTime<Utc>>>,
}

impl Client {
    pub fn new(ip: Ipv4Addr) -> Self {
        Client {
            ip_address: ip,
            current_layer: Arc::new(Mutex::new(None)),
            last_timer_access: Mutex::new(None),
        }
    }
}
