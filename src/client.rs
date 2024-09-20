use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Client {
    pub ip_address: Ipv4Addr,
    pub is_online: bool,
}

impl Client {
    pub fn new(ip: Ipv4Addr) -> Self {
        Client {
            ip_address: ip,
            is_online: false,
        }
    }
}
