pub mod utils;

use argparse::{ArgumentParser, Store, StoreTrue};
use std::{collections::HashMap, fs};

use utils::is_dir_battery;
use utils::{BATTERIES_PATH, CAPACITY_FILE, STATUS_FILE};

const FORMAT_LIST: &[&[&str; 2]] = &[&["%p", "remaining battery capacity (percentage)"]];

fn main() {
    let mut show_format_list = false;
    let mut format: String = String::from("BAT %p%");
    let mut format_charging: String = String::from("BAT CHR %p%");
    let mut format_none: String = String::from("NO BAT");
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format).add_option(
            &["-f", "--format"],
            Store,
            "Format string when not charging",
        );

        ap.refer(&mut format_charging).add_option(
            &["-c", "--format-charging"],
            Store,
            "Format string when charging",
        );

        ap.refer(&mut format_none).add_option(
            &["-n", "--format-none"],
            Store,
            "Format string when no battery is detected",
        );

        ap.refer(&mut show_format_list).add_option(
            &["-v", "--format-values"],
            StoreTrue,
            "Show possible format values",
        );

        ap.parse_args_or_exit();
    }

    if show_format_list {
        utils::print_format_list(FORMAT_LIST);
        std::process::exit(0);
    }

    let Some(battery_path) = utils::first_matching_dir(BATTERIES_PATH, None, Some(&is_dir_battery))
    else {
        println!("{}", format_none);
        std::process::exit(0);
    };

    let capacity_path = format!("{}/{}/{}", BATTERIES_PATH, battery_path, CAPACITY_FILE);
    let status_path = format!("{}/{}/{}", BATTERIES_PATH, battery_path, STATUS_FILE);

    let Ok(mut capacity) = fs::read_to_string(&capacity_path) else {
        eprintln!("Could not open {}", &capacity_path);
        std::process::exit(1);
    };

    let Ok(charging_state) = fs::read_to_string(&status_path) else {
        eprintln!("Could not open {}", &status_path);
        std::process::exit(1);
    };
    let is_charging = charging_state.trim() == "Charging";

    let mut data: HashMap<&str, &String> = HashMap::new();

    capacity = String::from(capacity.trim());

    data.insert("%p", &capacity);

    if is_charging {
        println!("{}", utils::format_output(data, format_charging))
    } else {
        println!("{}", utils::format_output(data, format))
    }
}
