use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream},
    sync::Arc,
    thread,
    time::Duration,
};

use serde_json::Value;

use crate::{client::Client, shared_data::SharedData};

fn try_layer_change(ip: Ipv4Addr, port: u16, layer: &str, timeout: Duration) {
    let msg = format!("{{\"ChangeLayer\":{{\"new\":\"{}\"}}}}\n", layer);

    match TcpStream::connect_timeout(&SocketAddr::V4(SocketAddrV4::new(ip, port)), timeout) {
        Ok(mut stream) => {
            let mut buf = [0; 1024];
            // read message from kanata first, otherwise it won't accept the command
            let _ = stream.read(&mut buf);
            let _ = stream.write(msg.as_bytes());
        }
        _ => {}
    }
}

fn try_layer_change_all(shared_data: Arc<SharedData>, layer: &str) {
    let port = shared_data.config.kanata_port;
    for client in &shared_data.clients {
        let ip = client.ip_address;
        let layer = layer.to_string();
        let timeout = Duration::from_millis(shared_data.config.kanata_tcp_timeout_ms);
        thread::spawn(move || try_layer_change(ip, port, &layer, timeout));
    }
}

pub fn enable_keyboards(shared_data: Arc<SharedData>) {
    try_layer_change_all(shared_data, "enabled");
}

pub fn disable_keyboards(shared_data: Arc<SharedData>) {
    try_layer_change_all(shared_data, "disabled");
}

fn get_current_layer(client: &Client, port: u16, timeout: Duration) -> Option<String> {
    let mut stream = TcpStream::connect_timeout(
        &SocketAddr::V4(SocketAddrV4::new(client.ip_address, port)),
        timeout,
    )
    .ok()?;

    let mut buf = [0; 1024];
    stream.read(&mut buf).ok()?;
    // write invalid message to disconnect from kanata - otherwise it accumulates connections
    let _ = stream.write("{}".as_bytes());
    let msg_str = String::from_utf8_lossy(&buf);
    let msg_trim = msg_str.lines().next()?;
    let msg_json: Value = serde_json::from_str(&msg_trim).unwrap();
    let new_layer = msg_json["LayerChange"]["new"].as_str()?;

    Some(new_layer.to_string())
}

fn update_client(shared_data: Arc<SharedData>, client_index: usize) {
    let client = shared_data
        .clients
        .get(client_index)
        .expect("trusting update_clients on bounds check");
    let current_layer = get_current_layer(
        client,
        shared_data.config.kanata_port,
        Duration::from_millis(shared_data.config.kanata_tcp_timeout_ms),
    );
    *client.current_layer.lock().unwrap() = current_layer;
}

fn update_clients(shared_data: Arc<SharedData>) {
    for i in 0..shared_data.clients.len() {
        let shared_data_clone = shared_data.clone();
        thread::spawn(move || update_client(shared_data_clone, i));
    }
}

pub fn start_client_update_thread(shared_data: Arc<SharedData>) {
    thread::spawn(move || loop {
        update_clients(shared_data.clone());
        thread::sleep(Duration::from_secs(
            shared_data.config.kanata_client_scan_interval_seconds,
        ));
    });
}
