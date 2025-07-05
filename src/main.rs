mod config;
mod time_utils;
mod slurp_util;
mod grim_util;

fn main() {
    let config = config::Config::load();

    let path = time_utils::TimeUtil::get_directory(&config);

    if grim_util::Grim::screenshot(&path) {
        println!("✅ Screenshot saved to: {}", path.display());
    } else {
        println!("❌ Screenshot failed");
    }
}
