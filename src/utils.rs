use std::{collections::HashMap, fs, path::Path};

pub const NET_DIR: &str = "/sys/class/net";

const SUFFIXES: &[&str; 7] = &["B", "K", "M", "G", "T", "P", "E"];

const ID: usize = 0;
const VALUE_DESCRIPTION: usize = 1;

pub const BATTERIES_PATH: &str = "/sys/class/power_supply";
pub const CAPACITY_FILE: &str = "capacity";
pub const STATUS_FILE: &str = "status";

pub fn failure(msg: &str) -> ! {
    eprintln!("Failure: {}", msg);
    std::process::exit(1);
}

pub fn format_bytes(bytes: u64) -> String {
    let mut converted_bytes = bytes as f64;
    let mut suffix_counter = 0;

    while converted_bytes >= 1024.0 {
        converted_bytes /= 1024.0;
        suffix_counter += 1;
    }

    return format!("{:.2}{}", converted_bytes, SUFFIXES[suffix_counter]);
}

pub fn first_matching_dir(
    root_dir: &str,
    prefixes: Option<Vec<&str>>,
    dir_check: Option<&dyn Fn(&str) -> bool>,
) -> Option<String> {
    let Ok(paths) = std::fs::read_dir(root_dir) else {
		return None
	};

    for p in paths {
        let Ok(dir) = p else {continue};

        let dir_path = dir.path();
        let Some(dir_str) = dir_path.as_os_str().to_str() else {
			continue
		};

        let dir_split = dir_str.split("/").collect::<Vec<&str>>();
        let dir_name = dir_split[dir_split.len() - 1];

        match prefixes {
            Some(ref p) => {
                if !p.iter().any(|p| dir_name.starts_with(p)) {
                    continue;
                }
            }
            None => (),
        }

        match dir_check {
            Some(check) => {
                if !check(dir_str) {
                    continue;
                }
            }
            None => (),
        }

        return Some(dir_name.to_string());
    }

    return None;
}

pub fn operstate_up(interface: &str) -> bool {
    let Ok(operstate) = std::fs::read_to_string(format!("{}/operstate", interface)) else {
		return false;
	};

    return operstate.trim() == "up";
}

pub fn format_output(data: HashMap<&str, &String>, format: String) -> String {
    let mut res = format.clone();

    for id in data.keys() {
        res = res.replace(id, &data[id]);
    }

    return res;
}

pub fn print_format_list(format_list: &[&[&str; 2]]) {
    println!("Supported format values");
    for arg in format_list {
        println!("\t{} -> {}", arg[ID], arg[VALUE_DESCRIPTION]);
    }
}

pub fn is_dir_battery(dir: &str) -> bool {
    let Ok(power_type) = fs::read_to_string(format!("{}/type", dir)) else {
		return false;
	};

    return power_type.trim() == "Battery"
        && Path::new(&format!("{}/{}", dir, CAPACITY_FILE)).exists()
        && Path::new(&format!("{}/{}", dir, STATUS_FILE)).exists();
}
