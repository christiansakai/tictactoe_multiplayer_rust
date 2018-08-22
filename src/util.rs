use std::process::Command;

pub fn clear_terminal() {
    Command::new("clear").status()
        .expect("Failed to clear console");
}
