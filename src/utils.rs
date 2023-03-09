pub const NET_DIR: &str = "/sys/class/net";

const SUFFIXES: &[&str; 7] = &["B", "K", "M", "G", "T", "P", "E"];

pub fn format_bytes(bytes: u64) -> String {
	let mut converted_bytes = bytes as f64;
	let mut suffix_counter = 0;

	while converted_bytes >= 1024.0 {
		converted_bytes /= 1024.0;
		suffix_counter += 1;
	}

	return format!("{:.2}{}", converted_bytes, SUFFIXES[suffix_counter]);
}

pub fn first_matching_dir(root_dir: &str, prefixes: Vec<&str>, dir_check: Option<&dyn Fn(&str) -> bool>) -> Option<String> {
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
		if prefixes.iter().any(|p| dir_name.starts_with(p)) {
			match dir_check {
				Some(check) => {
					if !check(dir_str) {
						continue;
					}
				},
				None => ()
			}

			return Some(dir_name.to_string());
		}
	}

	return None
}

pub fn operstate_up(interface: &str) -> bool {
	let Ok(operstate) = std::fs::read_to_string(format!("{}/operstate", interface)) else {
		return false;
	};

	return operstate.trim() == "up";
}
