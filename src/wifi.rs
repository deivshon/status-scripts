use std::fs;

const NET_DIR: &str = "/sys/class/net";

fn terminate_early(interface: Option<String>) -> ! {
	match interface {
		Some(_) =>  println!("WIFI UP"),
		None => ()
	}

	std::process::exit(0);
}

fn operstate_up(interface: &str) -> bool {
	let Ok(operstate) = fs::read_to_string(format!("{}/operstate", interface)) else {
		return false;
	};

	return operstate.trim() == "up";
}

fn get_interface() -> Option<String> {
	let Ok(paths) = fs::read_dir(NET_DIR) else {
		return None
	};

	for p in paths {
		let Ok(ifa) = p else {continue};

		let ifa_path = ifa.path();
		let Some(ifa_str) = ifa_path.as_os_str().to_str() else {
			continue
		};

		let ifa_split = ifa_str.split("/").collect::<Vec<&str>>();
		let ifa_name = ifa_split[ifa_split.len() - 1];
		if (ifa_name.starts_with("wlan") || ifa_name.starts_with("wlp")) &&
		   operstate_up(ifa_str) {
			return Some(ifa_name.to_string());
		}
	}

	return None
}

fn strength_percentage(dbm: i32) -> i32 {
	if dbm > -50 {
		return 100;
	}
	if dbm < -100 {
		return 0;
	}

	return (dbm + 100) * 2;
}

fn main() {
	let Some(interface) = get_interface() else {
		terminate_early(None);
	};

	let mut iw_command = std::process::Command::new("iw");
	iw_command.arg("dev");
	iw_command.arg(&interface);
	iw_command.arg("link");

	let Ok(iw_result) = iw_command.output() else {
		terminate_early(Some(interface));
	};

	let Ok(iw_output) = String::from_utf8(iw_result.stdout) else {
		terminate_early(Some(interface));
	};

	let mut ssid: Option<&str> = None;
	let mut dbm: Option<i32> = None;
	for line in iw_output.lines() {
		let split_line = line.split_whitespace().collect::<Vec<&str>>();

		if split_line[0] == "SSID:" && split_line.len() > 1 {
			ssid = Some(split_line[1]);
			continue;
		}
		if split_line[0] == "signal:" && split_line.len() > 1 {
			match split_line[1].parse::<i32>() {
				Ok(signal) => dbm = Some(signal),
				Err(_) => terminate_early(Some(interface))
			}

			break;
		}
	}

	let (Some(dbm), Some(ssid)) = (dbm, ssid) else {
		terminate_early(Some(interface));
	};

	println!("WIFI {}% {}", strength_percentage(dbm), ssid);
}
