pub mod utils;
use nix::sys::statvfs::statvfs;

fn main() {
	let mounted_on;
	match std::env::args().nth(1) {
		Some(arg) => mounted_on = arg,
		None => mounted_on = String::from("/")
	}

	let Ok(disk) = statvfs(mounted_on.as_str()) else {
		eprintln!("Could not retrieve disk data");
		std::process::exit(1);
	};
	
	println!("DISK {}", utils::format_bytes(disk.blocks_available() * disk.block_size()))
}
