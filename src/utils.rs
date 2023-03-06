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
