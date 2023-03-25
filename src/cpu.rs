pub mod utils;

use std::{fmt, collections::HashMap};
use argparse::{ArgumentParser, Store};

const PROC_STAT: &str = "/proc/stat";
const STORAGE_FILE: &str = "/tmp/cpu-status-data";

const USER_IDX: usize = 0;
const NICE_IDX: usize = 1;
const SYSTEM_IDX: usize = 2;
const IDLE_IDX: usize = 3;
const IOWAIT_IDX: usize = 4;
const IRQ_IDX: usize = 5;
const SOFTIRQ_IDX: usize = 6;
const STEAL_IDX: usize = 7;
const GUEST_IDX: usize = 8;
const GUEST_NICE_IDX: usize = 9;

#[derive(Debug)]
struct CpuUsage {
	user: u64,
	nice: u64,
	system: u64,
	idle: u64,
	iowait: u64,
	irq: u64,
	softirq: u64,
	steal: u64,
	guest: u64,
	guest_nice: u64,
	total: u64
}

enum Error {
	ParseErr,
	IOErr(std::io::Error)
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::ParseErr => write!(f, "Could not parse data"),
			Error::IOErr(e) => write!(f, "An IO error occurred: {}", e)
		}
	}
}

impl std::ops::Sub<CpuUsage> for CpuUsage {
	type Output = CpuUsage;

	fn sub(self, rhs: CpuUsage) -> Self::Output {
		CpuUsage {
			user: self.user - rhs.user,
			nice: self.nice - rhs.nice,
			system: self.system - rhs.system,
			idle: self.idle - rhs.idle,
			iowait: self.iowait - rhs.iowait,
			irq: self.irq - rhs.irq,
			softirq: self.softirq - rhs.softirq,
			steal: self.steal - rhs.steal,
			guest: self.guest - rhs.guest,
			guest_nice: self.guest_nice - rhs.guest_nice,
			total: self.total - rhs.total
		}
	}
}

impl CpuUsage {
	fn from_file(filepath: &str, store_result: bool) -> Result<Self, Error> {
		let file_content;
		match std::fs::read_to_string(filepath) {
			Ok(d) => file_content = d,
			Err(e) => return Err(Error::IOErr(e))
		}

		let file_lines = file_content.split("\n").collect::<Vec<&str>>();
		if file_lines.is_empty() {
			return Err(Error::ParseErr);
		}

		let data = file_lines[0].split_whitespace().collect::<Vec<&str>>();
		if data.len() != 11 {
			return Err(Error::ParseErr);
		}

		let cpu_times: Vec<u64> = data[1..]
			.iter()
			.map(|v| v.parse::<u64>())
			.filter_map(|r| r.ok())
			.collect();
		
		if cpu_times.len() != 10 {
			return Err(Error::ParseErr);
		}
		
		if store_result {
			match std::fs::write(STORAGE_FILE, file_lines[0]) {
				Ok(_) => (),
				Err(e) => return Err(Error::IOErr(e))
			}
		}

		return Ok(CpuUsage {
			user: cpu_times[USER_IDX],
			nice: cpu_times[NICE_IDX],
			system: cpu_times[SYSTEM_IDX],
			idle: cpu_times[IDLE_IDX],
			iowait: cpu_times[IOWAIT_IDX],
			irq: cpu_times[IRQ_IDX],
			softirq: cpu_times[SOFTIRQ_IDX],
			steal: cpu_times[STEAL_IDX],
			guest: cpu_times[GUEST_IDX],
			guest_nice: cpu_times[GUEST_NICE_IDX],
			total: cpu_times.iter().sum()
		})
	}

	pub fn get() -> Result<Self, Error> {
		if std::path::Path::new(STORAGE_FILE).exists() {
			match (CpuUsage::from_file(STORAGE_FILE, false),
				   CpuUsage::from_file(PROC_STAT, true)) {
				(Ok(old), Ok(new)) => return Ok(new - old),
				(Err(e), _) => return Err(e),
				(_, Err(e)) => return Err(e)
			}
		}
		else {
			match CpuUsage::from_file(PROC_STAT, true) {
				Ok(res) => return Ok(res),
				Err(e) => return Err(e)
			}
		}
	}

	pub fn percentage(&self) -> f32 {
		return 100.0 - ((self.idle + self.iowait) as f32 / self.total as f32) * 100.0;
	}
}
fn main() {
	let cpu_usage;
	let mut format: String = String::from("CPU %p%");

	{
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format)
            .add_option(&["-f", "--format"], Store, "Format string (%p -> CPU use percentage)");

        ap.parse_args_or_exit();
    }

	match CpuUsage::get() {
		Ok(c) => cpu_usage = c,
		Err(e) => {
			eprintln!("Could not acquire CPU usage: {}", e);
			std::process::exit(1);
		}
	};

	let mut data: HashMap<&str, &String> = HashMap::new();
	let cpu_percentage = format!("{:.2}", cpu_usage.percentage());
	data.insert("%p", &cpu_percentage);

	println!("{}", utils::format_output(data, format));
}
