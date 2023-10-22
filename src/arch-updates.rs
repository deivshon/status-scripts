pub mod utils;

use std::{collections::HashMap, process::Command};

use argparse::{ArgumentParser, Store, StoreTrue};
use utils::failure;

const FORMAT_LIST: &[&[&str; 2]] = &[
    &["%p", "pacman updates number"],
    &["%y", "aur updates number"],
    &["%t", "total updates number"],
];

const CHECKUPDATES_COMMAND: &str = "checkupdates";
const YAY_COMMAND: &str = "yay";
const PARU_COMMAND: &str = "paru";
const AUR_HELPER_ARG: &str = "-Qua";

fn main() {
    let aur_helper_command: &str;

    let mut show_format_list = false;
    let mut format: String = String::from("%p/%y");
    let mut use_paru: bool = false;
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format)
            .add_option(&["-f", "--format"], Store, "Format string");

        ap.refer(&mut show_format_list).add_option(
            &["-v", "--format-values"],
            StoreTrue,
            "Show possible format values",
        );

        ap.refer(&mut use_paru)
            .add_option(&["-p", "--paru"], StoreTrue, "Use paru instead of yay");

        ap.parse_args_or_exit();
    }

    if show_format_list {
        utils::print_format_list(FORMAT_LIST);
        std::process::exit(0);
    }

    aur_helper_command = match use_paru {
        false => YAY_COMMAND,
        true => PARU_COMMAND,
    };

    let pacman_updates = match Command::new(CHECKUPDATES_COMMAND).output() {
        Ok(o) => o.stdout,
        Err(e) => {
            failure(format!("Could not obtain `{}` output: {}", CHECKUPDATES_COMMAND, e).as_str())
        }
    };
    let pacman_updates = String::from_utf8_lossy(&pacman_updates).lines().count();

    let aur_updates = match Command::new(aur_helper_command)
        .arg(AUR_HELPER_ARG)
        .output()
    {
        Ok(o) => o.stdout,
        Err(e) => failure(
            format!(
                "Could not obtain `{} {}` output: {}",
                aur_helper_command, AUR_HELPER_ARG, e
            )
            .as_str(),
        ),
    };
    let aur_updates = String::from_utf8_lossy(&aur_updates).lines().count();
    let total_updates = pacman_updates + aur_updates;

    let pacman_updates = format!("{}", pacman_updates);
    let aur_updates = format!("{}", aur_updates);
    let total_updates = format!("{}", total_updates);

    let mut data: HashMap<&str, &String> = HashMap::new();

    data.insert("%p", &pacman_updates);
    data.insert("%y", &aur_updates);
    data.insert("%t", &total_updates);

    println!("{}", utils::format_output(data, format))
}
