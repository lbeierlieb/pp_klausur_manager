use std::{
    fs::{self, read_to_string},
    path::Path,
};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

const PERSISTANCE_PATH: &str = "pp.save";

#[derive(Debug, Deserialize, Serialize)]
struct SaveState {
    start_time: DateTime<Utc>,
    duration_min: u32,
}

pub fn get_persisted_time() -> Option<(DateTime<Utc>, Duration)> {
    let filecontent = read_to_string(PERSISTANCE_PATH).ok()?;
    let save_state = serde_json::from_str::<SaveState>(&filecontent).ok()?;
    Some((
        save_state.start_time,
        Duration::new(60 * save_state.duration_min as i64, 0)
            .expect("invalid time in persistance file"),
    ))
}

pub fn persist_time(start_time: DateTime<Utc>, duration: Duration) {
    let save_state = SaveState {
        start_time,
        duration_min: duration.num_minutes() as u32,
    };
    if let Ok(json) = serde_json::to_string(&save_state) {
        let _ = fs::write(PERSISTANCE_PATH, json);
    }
}

pub fn delete_persisted_time() {
    let path = Path::new(PERSISTANCE_PATH);
    if path.exists() {
        let _ = fs::remove_file(path);
    }
}
