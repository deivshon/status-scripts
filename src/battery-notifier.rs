pub mod utils;

use notify_rust::{Notification, Urgency};
use std::fs;
use std::thread::sleep;

use argparse::{ArgumentParser, Store};
use utils::is_dir_battery;
use utils::{failure, BATTERIES_PATH, CAPACITY_FILE, STATUS_FILE};

fn main() {
    let Some(battery_path) = utils::first_matching_dir(BATTERIES_PATH, None, Some(&is_dir_battery))
    else {
        eprintln!("No battery found");
        std::process::exit(0);
    };

    let mut low_threshold: u8 = 20;
    let mut high_threshold: u8 = 80;
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut low_threshold).add_option(
            &["-l", "--lower-threshold"],
            Store,
            "Threshold which triggers low battery charge notification",
        );
        ap.refer(&mut high_threshold).add_option(
            &["-u", "--upper-threshold"],
            Store,
            "Threshold which triggers high battery charge notification if the battery is still charging",
        );

        ap.parse_args_or_exit();
    }

    let capacity_path = format!("{}/{}/{}", BATTERIES_PATH, battery_path, CAPACITY_FILE);
    let status_path = format!("{}/{}/{}", BATTERIES_PATH, battery_path, STATUS_FILE);

    let mut been_notified_low = false;
    let mut been_notified_high = false;
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

        if been_notified_low && is_charging {
            been_notified_low = false;
        }
        if been_notified_high && !is_charging {
            been_notified_high = false
        }

        if current_capacity < low_threshold && !is_charging && !been_notified_low {
            let notification = Notification::new()
                .summary("Low battery charge")
                .body(format!("Battery lower than set threshold of {}%", &low_threshold).as_str())
                .urgency(Urgency::Critical)
                .show();

            match notification {
                Ok(_) => been_notified_low = true,
                Err(_) => eprintln!("Could not show low charge notification"),
            }
        }
        if current_capacity > high_threshold && is_charging && !been_notified_high {
            let notification = Notification::new()
                .summary("High battery charge")
                .body(
                    format!(
                        "You can remove the charging device in order to preserve battery health"
                    )
                    .as_str(),
                )
                .urgency(Urgency::Critical)
                .show();

            match notification {
                Ok(_) => been_notified_high = true,
                Err(_) => eprintln!("Could not show high charge notification"),
            }
        }

        sleep(std::time::Duration::from_millis(1000));
    }
}
