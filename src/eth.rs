pub mod utils;

use argparse::{ArgumentParser, Store, StoreTrue};
use std::collections::HashMap;

const FORMAT_LIST: &[&[&str; 2]] = &[&["%i", "interface name"]];

fn main() {
    let mut show_format_list = false;
    let mut format_up: String = String::from("ETH UP");
    let mut format_down: String = String::from("ETH DOWN");
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format_up).add_option(
            &["-u", "--format-up"],
            Store,
            "Format string when connected",
        );

        ap.refer(&mut format_down).add_option(
            &["-d", "--format-down"],
            Store,
            "Format string when not connected",
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

    let Some(interface) = utils::first_matching_dir(
        utils::NET_DIR,
        Some(vec!["eth", "enp"]),
        Some(&utils::operstate_up),
    ) else {
        println!("{}", format_down);
        std::process::exit(0);
    };

    let mut data: HashMap<&str, &String> = HashMap::new();
    data.insert("%i", &interface);

    println!("{}", utils::format_output(data, format_up));
}
