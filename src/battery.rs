pub mod utils;

use std::fs;
use std::path::Path;

const BATTERIES_PATH: &str = "/sys/class/power_supply";
const CAPACITY_FILE: &str = "capacity";
const STATUS_FILE: &str = "status";

fn is_battery(dir: &str) -> bool {
	let Ok(power_type) = fs::read_to_string(format!("{}/type", dir)) else {
		return false;
	};

	return power_type.trim() == "Battery" &&
		   Path::new(&format!("{}/{}", dir, CAPACITY_FILE)).exists() &&
		   Path::new(&format!("{}/{}", dir, STATUS_FILE)).exists()
}

fn main() {
	let Some(battery_path) = utils::first_matching_dir(
		BATTERIES_PATH,
		None,
		Some(&is_battery)
	) else {
		std::process::exit(0);
	};

	let capacity_path = format!("{}/{}/{}", BATTERIES_PATH, battery_path, CAPACITY_FILE);
	let status_path = format!("{}/{}/{}", BATTERIES_PATH, battery_path, STATUS_FILE);

	let Ok(capacity) = fs::read_to_string(&capacity_path) else {
		eprintln!("Could not open {}", &capacity_path);
		std::process::exit(1);
	};

	let Ok(charging_state) = fs::read_to_string(&status_path) else {
		eprintln!("Could not open {}", &status_path);
		std::process::exit(1);
	};
	let is_charging = charging_state.trim() == "Charging";

	println!("BAT{} {}%", if is_charging{" CHR"} else {""}, capacity.trim());
}
