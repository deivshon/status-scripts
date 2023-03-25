pub mod utils;

use std::collections::HashMap;

use argparse::{ArgumentParser, Store};
use serde::Deserialize;

const MULLVAD_CHECK: &str = "http://am.i.mullvad.net/json";

#[allow(dead_code)]
#[derive(Deserialize)]
struct Blacklist {
	name: String,
	link: String,
	blacklisted: bool
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Blacklisted {
	blacklisted: bool,
	results: Vec<Blacklist>
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct MullvadData {
    ip: String,
    country: String,
    city: String,
    longitude: f32,
    latitude: f32,
    mullvad_exit_ip: bool,
    mullvad_exit_ip_hostname: Option<String>,
    mullvad_server_type: Option<String>,
	blacklisted: Blacklisted,
	organization: String
}

fn main() {
	let mut format_up: String = String::from("%h");
	let mut format_down: String = String::from("N/C - %c");

	{
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format_up)
            .add_option(&["-u", "--format-up"], Store, "Format string when connected to Mullvad (%h -> server hostname | %c -> server country)");
		
		ap.refer(&mut format_down)
            .add_option(&["-d", "--format-down"], Store, "Format string when not connected to Mullvad (%c -> server country)");

        ap.parse_args_or_exit();
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
			None => panic!()
		}
		format_data.insert("%h", &mullvad_host);

		println!("{}", utils::format_output(format_data, format_up));
	}
	else {
		println!("{}", utils::format_output(format_data, format_down));
	}
}
