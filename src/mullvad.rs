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
	let Ok(res) = reqwest::blocking::get(MULLVAD_CHECK) else {
		eprintln!("Could not send request to {}", MULLVAD_CHECK);
		std::process::exit(1);
	};

	let data;
	match res.json::<MullvadData>() {
		Ok(d) => data = d,
		Err(e) => {
			eprintln!("{}", e);
			std::process::exit(1);
		}
	};

	if data.mullvad_exit_ip {
		match data.mullvad_exit_ip_hostname {
			Some(host) => println!("{}", host),
			None => panic!()
		}
	}
	else {
		println!("N/C - {}", data.country);
	}
}
