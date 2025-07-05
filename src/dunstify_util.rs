use std::process::{Command, Stdio};
use std::path::PathBuf;

pub fn notify_with_action(path: &PathBuf) {
    let mut child = Command::new("dunstify")
        .arg("--action=default,Edit")
        .arg("Screenshot saved")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn dunstify");

    let output = child.wait_with_output().expect("failed to wait on dunstify");
    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.contains("2") || stdout.contains("Edit") || stdout.contains("default") {
        let _ = Command::new("swappy")
            .arg("-f")
            .arg(path)
            .spawn();
    }
}
