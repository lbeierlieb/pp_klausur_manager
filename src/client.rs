use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Client {
    pub ip_address: Ipv4Addr,
    pub is_online: bool,
    pub current_layer: Option<String>,
}

impl Client {
    pub fn new(ip: Ipv4Addr) -> Self {
        Client {
            ip_address: ip,
            is_online: false,
            current_layer: None,
        }
    }
}
