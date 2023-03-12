pub mod utils;
use std::fs;

const MEMINFO_PATH: &str = "/proc/meminfo";

fn main() {
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

	println!("RAM {}/{} ({:.2}%)",
		utils::format_bytes(mem_used),
		utils::format_bytes(mem_total),
		(mem_used as f32 / mem_total as f32) * 100.0
	);
}
