mod time_utils;

fn main() {
    println!("{}", time_utils::TimeUtil::get_time());

    let path = time_utils::TimeUtil::get_directory();
    println!("{}", path.display());
}
