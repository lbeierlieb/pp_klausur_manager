use std::{env, process::exit, sync::Arc};

use input_parser::{
    create_default_config_if_necessary, get_ip_addresses_of_room, get_rooms,
    get_symlink_info_of_room, parse_config, room_exists, Config,
};
use kanata_tcp::start_client_update_thread;
use shared_data::SharedData;
use symlinks::update_symlink_status;
use timing_webserver::start_webserver_thread;

mod client;
mod input_parser;
mod kanata_tcp;
mod shared_data;
mod symlinks;
mod timing_webserver;
mod tui;
mod tui_basic;

const DEFAULT_CONFIG_CONTENT: &str = include_str!("../res/ppmngr_cfg_default.json");
const CONFIG_RUNTIME_PATH: &str = "ppmngr_cfg.json";

fn main() {
    create_default_config_if_necessary(CONFIG_RUNTIME_PATH, DEFAULT_CONFIG_CONTENT);
    let config = parse_config(CONFIG_RUNTIME_PATH).expect("failed to parse config");
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 || args[1] == "-h" || args[1] == "--help" {
        print_usage(&config);
        exit(0);
    }
    let room = &args[1];
    if !room_exists(room, &config) {
        print_available_rooms(&config);
        exit(1);
    }
    let clients =
        get_ip_addresses_of_room(&room, &config).expect(&format!("Room '{}' does not exist", room));
    let symlink_info = get_symlink_info_of_room(&room, &config)
        .expect("this should be safe at this point, can only fail if room would not exist");
    let shared_data = Arc::new(SharedData::new(config, clients, symlink_info));
    update_symlink_status(shared_data.clone());
    start_webserver_thread(shared_data.clone());
    start_client_update_thread(shared_data.clone());
    tui::tui_main(shared_data).unwrap();
}

fn print_usage(config: &Config) {
    println!("Usage: pp_klausur_manager <room>");
    print_available_rooms(config);
}

fn print_available_rooms(config: &Config) {
    let rooms = get_rooms(&config);
    println!(
        "Available rooms are: [{}]",
        rooms
            .into_iter()
            .filter(|room| room != &"dummy")
            .collect::<Vec<_>>()
            .join(", ")
    );
}
