pub mod structured;
pub mod utils;

use argparse::{ArgumentParser, Store, StoreTrue};
use std::collections::HashMap;
use structured::mullvad::MullvadData;

const MULLVAD_CHECK: &str = "http://am.i.mullvad.net/json";

const FORMAT_LIST: &[&[&str; 2]] = &[
    &["%h", "server hostname"],
    &["%c", "country based on IP geolocation"],
];

fn main() {
    let mut show_format_list = false;
    let mut format_up: String = String::from("%h");
    let mut format_down: String = String::from("N/C - %c");
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format_up).add_option(
            &["-u", "--format-up"],
            Store,
            "Format string when connected to Mullvad",
        );

        ap.refer(&mut format_down).add_option(
            &["-d", "--format-down"],
            Store,
            "Format string when not connected to Mullvad",
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

    let Ok(res) = reqwest::blocking::get(MULLVAD_CHECK) else {
        eprintln!("Could not send request to {}", MULLVAD_CHECK);
        std::process::exit(1);
    };

    let mullvad_data;
    match res.json::<MullvadData>() {
        Ok(d) => mullvad_data = d,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut format_data: HashMap<&str, &String> = HashMap::new();
    format_data.insert("%c", &mullvad_data.country);

    if mullvad_data.mullvad_exit_ip {
        let mullvad_host;
        match mullvad_data.mullvad_exit_ip_hostname {
            Some(host) => mullvad_host = host,
            None => panic!(),
        }
        format_data.insert("%h", &mullvad_host);

        println!("{}", utils::format_output(format_data, format_up));
    } else {
        println!("{}", utils::format_output(format_data, format_down));
    }
}
