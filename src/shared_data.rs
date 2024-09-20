use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};

use crate::client::Client;

#[derive(Debug, Clone)]
pub struct SharedData {
    pub clients: Arc<Mutex<Vec<Client>>>,
    pub times: Arc<Mutex<Option<(DateTime<Utc>, Duration)>>>, // (start_time, finish_time)
}

impl SharedData {
    pub fn new(clients: Vec<Client>) -> Self {
        SharedData {
            clients: Arc::new(Mutex::new(clients)),
            times: Arc::new(Mutex::new(None)),
        }
    }

    pub fn finish_time_as_unix(&self) -> Option<i64> {
        self.times
            .lock()
            .unwrap()
            .map(|(start_time, duration)| (start_time + duration).timestamp())
    }
}
