use std::process::Command;

pub struct Slurp;

impl Slurp {
    pub fn get_region() -> Option<String> {
        let output = Command::new("slurp")
            .output();

        let region = String::from_utf8(output.unwrap().stdout);
        Some(region.expect("region is undefined").trim().to_string())
    }
}
