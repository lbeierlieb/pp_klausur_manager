use std::os::unix::fs as unixfs;
use std::sync::Arc;
use std::{fs, io, path::Path};

use crate::shared_data::SharedData;

pub fn unlock_taskdescription(shared_data: Arc<SharedData>) {
    let real_target = shared_data.symlink_info.real_target.clone();
    try_set_symlink_target_and_update(shared_data, &real_target);
}

pub fn lock_taskdescription(shared_data: Arc<SharedData>) {
    let dummy_target = shared_data.symlink_info.dummy_target.clone();
    try_set_symlink_target_and_update(shared_data, &dummy_target);
}

fn try_set_symlink_target_and_update(shared_data: Arc<SharedData>, target: &str) {
    let symlink = shared_data.symlink_info.symlink_path.as_str();
    let _ = set_symlink_target(target, symlink);
    update_symlink_status(shared_data);
}

fn set_symlink_target(target: &str, symlink: &str) -> io::Result<()> {
    let target = Path::new(target);
    let symlink = Path::new(symlink);
    // remove if symlink exists
    if symlink.symlink_metadata().is_ok() {
        fs::remove_file(symlink)?;
    }
    unixfs::symlink(target, symlink)
}

fn get_symlink_target(symlink_path: &str) -> Option<String> {
    let symlink_path = Path::new(symlink_path);
    fs::read_link(symlink_path)
        .ok()
        .and_then(|target| target.to_str().map(|str| str.to_string()))
}

pub fn update_symlink_status(shared_data: Arc<SharedData>) {
    let target = get_symlink_target(&shared_data.symlink_info.symlink_path);
    *shared_data.symlink_target.lock().unwrap() = target;
}
