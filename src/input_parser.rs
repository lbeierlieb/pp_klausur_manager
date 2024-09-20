use std::fs::read_to_string;

use crate::client::Client;

pub fn parse_ip_address_list(filename: &str) -> Option<Vec<Client>> {
    let filecontent = read_to_string(filename).ok()?;
    let mut addresses = vec![];
    for line in filecontent.lines() {
        let address = line.parse().ok()?;
        let client = Client::new(address);
        addresses.push(client);
    }
    Some(addresses)
}
