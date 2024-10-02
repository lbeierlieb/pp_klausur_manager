use std::sync::Arc;

use input_parser::{get_ip_addresses_of_room, parse_config};
use kanata_tcp::start_client_update_thread;
use shared_data::SharedData;
use timing_webserver::start_webserver_thread;

mod client;
mod input_parser;
mod kanata_tcp;
mod shared_data;
mod timing_webserver;
mod tui;
mod tui_basic;

fn main() {
    let config = parse_config("ppmngr_cfg.json").expect("failed to parse config");
    let room = "dummy";
    let clients =
        get_ip_addresses_of_room(room, &config).expect(&format!("Room '{}' does not exist", room));
    let shared_data = Arc::new(SharedData::new(config, clients));
    start_webserver_thread(shared_data.clone());
    start_client_update_thread(shared_data.clone());
    tui::tui_main(shared_data).unwrap();
}
