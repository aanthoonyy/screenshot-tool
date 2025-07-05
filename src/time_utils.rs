use std::time::SystemTime;
use chrono::{DateTime, Local};
use std::path::PathBuf;
use crate::config::Config;

pub struct TimeUtil;

impl TimeUtil {
    fn get_sys_time() -> SystemTime {
        SystemTime::now()
    }

    fn get_datetime() -> DateTime<Local> {
        DateTime::<Local>::from(Self::get_sys_time())
    }

    fn get_formatted(config: &Config) -> String {
        let fmt = config.filename_format();
        let ext = config.extension();
        format!("{}.{ext}", Self::get_datetime().format(fmt))
    }

    pub fn get_directory(config: &Config) -> PathBuf {
        let mut path = config.save_dir().clone(); 
        path.push(Self::get_formatted(config));
        path
    }

    pub fn get_time(config: &Config) -> String {
        Self::get_formatted(config)
    }
}
