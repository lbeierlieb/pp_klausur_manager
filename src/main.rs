use shared_data::SharedData;
use timing_webserver::start_webserver_thread;

mod client;
mod shared_data;
mod timing_webserver;
mod tui;
mod tui_basic;

fn main() {
    let shared_data = SharedData::new();
    start_webserver_thread(shared_data.clone());
    tui::tui_main(shared_data).unwrap();
}
