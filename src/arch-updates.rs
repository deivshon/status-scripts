pub mod utils;

use std::{collections::HashMap, process::Command};

use argparse::{ArgumentParser, Store, StoreTrue};
use utils::failure;

const FORMAT_LIST: &[&[&str; 2]] = &[
    &["%p", "pacman updates number"],
    &["%y", "yay updates number"],
    &["%t", "total updates number"],
];

const CHECKUPDATES_COMMAND: &str = "checkupdates";
const YAY_COMMAND: &str = "yay";
const YAY_ARG: &str = "-Qua";

fn main() {
    let mut show_format_list = false;
    let mut format: String = String::from("%p/%y");
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format)
            .add_option(&["-f", "--format"], Store, "Format string");

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

    let pacman_updates = match Command::new(CHECKUPDATES_COMMAND).output() {
        Ok(o) => o.stdout,
        Err(e) => {
            failure(format!("Could not obtain `{}` output: {}", CHECKUPDATES_COMMAND, e).as_str())
        }
    };
    let pacman_updates = String::from_utf8_lossy(&pacman_updates).lines().count();

    let yay_updates = match Command::new(YAY_COMMAND).arg(YAY_ARG).output() {
        Ok(o) => o.stdout,
        Err(e) => failure(
            format!(
                "Could not obtain `{} {}` output: {}",
                YAY_COMMAND, YAY_ARG, e
            )
            .as_str(),
        ),
    };
    let yay_updates = String::from_utf8_lossy(&yay_updates).lines().count();
    let total_updates = pacman_updates + yay_updates;

    let pacman_updates = format!("{}", pacman_updates);
    let yay_updates = format!("{}", yay_updates);
    let total_updates = format!("{}", total_updates);

    let mut data: HashMap<&str, &String> = HashMap::new();

    data.insert("%p", &pacman_updates);
    data.insert("%y", &yay_updates);
    data.insert("%t", &total_updates);

    println!("{}", utils::format_output(data, format))
}
