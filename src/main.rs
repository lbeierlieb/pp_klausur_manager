use std::{
    io::stdin,
    sync::{Arc, Mutex},
};

use chrono::Utc;
use timing_webserver::start_webserver_thread;

mod timing_webserver;

fn main() {
    let shared_time = Arc::new(Mutex::new(None));
    start_webserver_thread(shared_time.clone());

    let mut input_buffer = String::new();
    loop {
        stdin().read_line(&mut input_buffer).unwrap();
        println!("resetting timer");
        let time = Utc::now().timestamp_millis() + 1000 * 60 * 90;
        *shared_time.lock().unwrap() = Some(time);
    }
}
