use std::process::Command;
use std::path::PathBuf;
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
}
