use input_parser::parse_ip_address_list;
use shared_data::SharedData;
use timing_webserver::start_webserver_thread;

mod client;
mod input_parser;
mod shared_data;
mod timing_webserver;
mod tui;
mod tui_basic;

fn main() {
    let clients =
        parse_ip_address_list("ip_addresses.txt").expect("Failed to parsed input IP addresses");
    let shared_data = SharedData::new(clients);
    start_webserver_thread(shared_data.clone());
    tui::tui_main(shared_data).unwrap();
}
