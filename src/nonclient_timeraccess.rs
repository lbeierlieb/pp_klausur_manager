use std::net::Ipv4Addr;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct NonclientTimerAccess {
    pub ip_address: Ipv4Addr,
    pub last_timer_access: DateTime<Utc>,
}

impl NonclientTimerAccess {
    pub fn new(ip_address: Ipv4Addr, last_timer_access: DateTime<Utc>) -> Self {
        NonclientTimerAccess {
            ip_address,
            last_timer_access,
        }
    }
}
