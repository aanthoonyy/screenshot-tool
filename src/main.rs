mod time_utils;
mod slurp_util;

fn main() {
    // println!("{}", time_utils::TimeUtil::get_time());

    // let path = time_utils::TimeUtil::get_directory();
    // println!("{}", path.display());

    let region = slurp_util::Slurp::get_region()
        .expect("error");

    println!("{}", region);
}
