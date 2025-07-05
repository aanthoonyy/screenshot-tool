use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::slurp_util::Slurp;

pub struct Grim;

impl Grim {
    pub fn screenshot(path: &PathBuf) -> bool {
        let region = Slurp::get_region().unwrap();

        Command::new("grim")
            .arg("-g")
            .arg(region)
            .arg(path)
            .status()
            .unwrap()
            .success()
    }

    pub fn copy_to_clipboard(path: &PathBuf) {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("failed to open screenshot: {}", e);
                return;
            }
        };

        let mut buffer = Vec::new();
        if let Err(e) = file.read_to_end(&mut buffer) {
            eprintln!("failed to read screenshot: {}", e);
            return;
        }

        let mut child = match Command::new("wl-copy")
            .arg("--type")
            .arg("image/png")
            .stdin(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed to start wl-copy: {}", e);
                return;
            }
        };

        if let Err(e) = child.stdin.as_mut().unwrap().write_all(&buffer) {
            eprintln!("failed to write to clipboard: {}", e);
            return;
        }

        println!("screenshot copied to clipboard.");
    }
}
