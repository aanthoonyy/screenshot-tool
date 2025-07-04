use std::time::SystemTime;
use chrono::{DateTime, Local};
use std::path::PathBuf;
use dirs; 

pub struct TimeUtil;

impl TimeUtil {
    fn get_sys_time() -> SystemTime {
        SystemTime::now()
    }

    fn get_datetime() -> DateTime<Local> {
        DateTime::<Local>::from(Self::get_sys_time())
    }

    fn get_formatted() -> String {
        Self::get_datetime().format("screenshot-%Y-%m-%d-%H-%M-%S.png").to_string()
    }

    pub fn get_directory() -> PathBuf {
        let mut path = dirs::picture_dir().expect("error, could not find the pictures dir");
        path.push(Self::get_formatted());
        return path
    }

    pub fn get_time() -> String {
        Self::get_formatted()
    }
}
