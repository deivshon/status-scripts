pub mod utils;

use utils::{BATTERIES_PATH, CAPACITY_FILE, STATUS_FILE};
use utils::is_dir_battery;

fn main() {
    let Some(battery_path) = utils::first_matching_dir(
		BATTERIES_PATH,
		None,
		Some(&is_dir_battery)
	) else {
		eprintln!("No battery found");
		std::process::exit(0);
	};


    let capacity_path = format!("{}/{}/{}", BATTERIES_PATH, battery_path, CAPACITY_FILE);
    let status_path = format!("{}/{}/{}", BATTERIES_PATH, battery_path, STATUS_FILE);

    println!("{}, {}", capacity_path, status_path)
}
