use std::sync::Mutex;

use chrono::{DateTime, Duration, Utc};

use crate::{
    client::Client,
    input_parser::{Config, SymlinkInfo},
};

#[derive(Debug)]
pub struct SharedData {
    pub config: Config,
    pub clients: Vec<Client>,
    pub times: Mutex<Option<(DateTime<Utc>, Duration)>>,
    pub symlink_info: SymlinkInfo,
    pub symlink_target: Mutex<Option<String>>,
}

impl SharedData {
    pub fn new(config: Config, clients: Vec<Client>, symlink_info: SymlinkInfo) -> Self {
        SharedData {
            config,
            clients,
            times: Mutex::new(None),
            symlink_info,
            symlink_target: Mutex::new(None),
        }
    }

    pub fn finish_time_as_unix(&self) -> Option<i64> {
        self.times
            .lock()
            .unwrap()
            .map(|(start_time, duration)| (start_time + duration).timestamp())
    }
}
