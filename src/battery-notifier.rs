pub mod utils;

use notify_rust::{Notification, Urgency};
use std::fs;
use std::thread::sleep;

use argparse::{ArgumentParser, Store};
use utils::is_dir_battery;
use utils::{BATTERIES_PATH, CAPACITY_FILE, STATUS_FILE};

fn failure(msg: &str) -> ! {
    eprintln!("Failure: {}", msg);
    std::process::exit(1);
}

fn main() {
    let Some(battery_path) = utils::first_matching_dir(
		BATTERIES_PATH,
		None,
		Some(&is_dir_battery)
	) else {
		eprintln!("No battery found");
		std::process::exit(0);
	};

    let mut threshold: u8 = 15;
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut threshold).add_option(
            &["-t", "--threshold"],
            Store,
            "Threshold which triggers low battery charge notification",
        );

        ap.parse_args_or_exit();
    }

    let capacity_path = format!("{}/{}/{}", BATTERIES_PATH, battery_path, CAPACITY_FILE);
    let status_path = format!("{}/{}/{}", BATTERIES_PATH, battery_path, STATUS_FILE);

    let mut been_notified = false;
    loop {
        let current_capacity = match fs::read_to_string(&capacity_path) {
            Ok(c) => c,
            Err(_) => failure("Could not get battery capacity"),
        };
        let current_capacity = match current_capacity.trim().parse::<u8>() {
            Ok(c) => c,
            Err(_) => failure("Could not parse battery capacity"),
        };

        let current_status = match fs::read_to_string(&status_path) {
            Ok(s) => s,
            Err(_) => failure("Could not get battery status"),
        };
        let current_status = current_status.trim();
        let is_charging = current_status == "Charging";

        if been_notified && is_charging {
            been_notified = false;
        }

        if current_capacity < threshold && !is_charging && !been_notified {
            let notification = Notification::new()
                .summary("Battery low")
                .body(format!("Battery lower than threshold ({}%)", &threshold).as_str())
                .urgency(Urgency::Critical)
                .show();

            match notification {
                Ok(_) => been_notified = true,
                Err(_) => failure("Could not show notification"),
            }
        }

        sleep(std::time::Duration::from_millis(1000));
    }
}
