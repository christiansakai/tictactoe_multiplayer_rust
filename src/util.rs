use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn clear_terminal() {
    Command::new("clear").status()
        .expect("Failed to clear console");
}

pub fn sleep(ms: usize) {
    let ms = ms as u64;
    thread::sleep(Duration::from_millis(ms));
}
