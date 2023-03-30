use std::collections::HashMap;

use argparse::{ArgumentParser, Store};
pub mod utils;

fn terminate_early(interface: Option<String>, format_up: &String, format_down: &String) -> ! {
	match interface {
		Some(_) =>  println!("{}", format_up),
		None => println!("{}", format_down)
	}

	std::process::exit(0);
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
	let mut format_up: String = String::from("WIFI UP");
	let mut format_down: String = String::from("WIFI DOWN");
	{
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format_up)
            .add_option(&["-u", "--format-up"], Store, "Format string when connected, values require iw (%p -> signal strength percentage | %s ssid");

		ap.refer(&mut format_down)
			.add_option(&["-d", "--format-down"], Store, "Format string when disconnected");

        ap.parse_args_or_exit();
    }

	let Some(interface) = utils::first_matching_dir(
		utils::NET_DIR,
		Some(vec!["wlan", "wlp"]),
		Some(&utils::operstate_up)
	) else {
		terminate_early(None, &format_up, &format_down);
	};

	let mut iw_command = std::process::Command::new("iw");
	iw_command.arg("dev");
	iw_command.arg(&interface);
	iw_command.arg("link");

	let Ok(iw_result) = iw_command.output() else {
		terminate_early(Some(interface), &format_up, &format_down);
	};

	let Ok(iw_output) = String::from_utf8(iw_result.stdout) else {
		terminate_early(Some(interface), &format_up, &format_down);
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
				Err(_) => terminate_early(Some(interface), &format_up, &format_down)
			}

			break;
		}
	}

	let (Some(dbm), Some(ssid)) = (dbm, ssid) else {
		terminate_early(Some(interface), &format_up, &format_down);
	};

	let mut data: HashMap<&str, &String> = HashMap::new();
	
	let strength_percentage = format!("{}", strength_percentage(dbm));
	let ssid = format!("{}", ssid);

	data.insert("%i", &interface);
	data.insert("%p", &strength_percentage);
	data.insert("%s", &ssid);

	println!("{}", utils::format_output(data, format_up));
}
