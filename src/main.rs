use std::sync::{Arc, Mutex};

use chrono::Utc;
use timing_webserver::start_webserver_thread;

mod timing_webserver;

fn main() {
    let now = Utc::now();
    let time = now.timestamp_millis() + 1000 * 60 * 90;
    let shared_time = Arc::new(Mutex::new(Some(time)));
    start_webserver_thread(shared_time);
}
