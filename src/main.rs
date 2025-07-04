mod time_utils;
mod slurp_util;
mod grim_util;

fn main() {
    let path = time_utils::TimeUtil::get_directory();

    if grim_util::Grim::screenshot(&path) {
        println!("✅ Screenshot saved to: {}", path.display());
    } else {
        println!("❌ Screenshot failed");
    }
}
