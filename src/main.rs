mod config;
mod time_utils;
mod slurp_util;
mod grim_util;

fn main() {
    let config = config::Config::load();
    let path = time_utils::TimeUtil::get_directory(&config);

    if grim_util::Grim::screenshot(&path) {
        println!("Screenshot saved to: {}", path.display());

        if config.copy_to_clipboard() {
            grim_util::Grim::copy_to_clipboard(&path);
        }
    } else {
        println!("Screenshot failed");
    }
}
