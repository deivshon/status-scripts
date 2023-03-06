mod utils;
use nix::sys::statvfs::statvfs;

fn main() {
	let Ok(disk) = statvfs("/") else {
		eprintln!("Could not retrieve disk data");
		std::process::exit(1);
	};
	
	println!("{}", utils::format_bytes(disk.blocks_available() * disk.block_size()))
}
