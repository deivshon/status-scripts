pub mod utils;

use std::{fs, collections::HashMap};
use argparse::{ArgumentParser, Store};

const MEMINFO_PATH: &str = "/proc/meminfo";

fn main() {
	let mut format: String = String::from("RAM %u/%t (%p%)");
	{
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format)
            .add_option(&["-f", "--format"], Store, "Format string (%p -> RAM use percentage | %a available RAM | %u used RAM | %t total RAM)");

        ap.parse_args_or_exit();
    }

	let Ok(meminfo) = fs::read_to_string(MEMINFO_PATH) else {
		eprintln!("Could not read {}", MEMINFO_PATH);
		std::process::exit(1);
	};

	let mut mem_total: Option<u64> = None;
	let mut mem_avail: Option<u64> = None;
	for line in meminfo.lines() {
		let split_line = line.split_whitespace().collect::<Vec<&str>>();
		if split_line.len() < 2 {continue}

		if split_line[0] == "MemTotal:" {
			match split_line[1].parse::<u64>() {
				Ok(mem) => mem_total = Some(mem * 1024),
				Err(_) => continue
			}
		}
		else if split_line[0] == "MemAvailable:" {
			match split_line[1].parse::<u64>() {
				Ok(mem) => mem_avail = Some(mem * 1024),
				Err(_) => continue
			}
		}
	}

	let (Some(mem_total), Some(mem_avail)) = (mem_total, mem_avail) else {
		eprintln!("Could not parse {}", MEMINFO_PATH);
		std::process::exit(1);
	};
	let mem_used = mem_total - mem_avail;

	let mut data: HashMap<&str, &String> = HashMap::new();
	let mem_percentage = format!("{:.2}", (mem_used as f32 / mem_total as f32) * 100.0);
	let mem_avail = format!("{}", utils::format_bytes(mem_avail));
	let mem_used = format!("{}", utils::format_bytes(mem_used));
	let mem_total = format!("{}", utils::format_bytes(mem_total));

	data.insert("%p", &mem_percentage);
	data.insert("%a", &mem_avail);
	data.insert("%u", &mem_used);
	data.insert("%t", &mem_total);

	println!("{}", utils::format_output(data, format));
}
