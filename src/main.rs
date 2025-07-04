#![allow(unused)]
use std::time::SystemTime;
use chrono::{DateTime, Utc};

fn main() {
    
    let sys_time = SystemTime::now();
    println!("{:?}", sys_time);

    let datetime: DateTime<Utc> = DateTime::<Utc>::from(sys_time);
    let formatted = datetime.format("%Y-%m-%d-%H:%M:%S").to_string();
    println!("screenshot-{}", formatted);
}