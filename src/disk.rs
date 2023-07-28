pub mod utils;
use std::collections::HashMap;

use argparse::{ArgumentParser, Store, StoreTrue};
use nix::sys::statvfs::statvfs;

const FORMAT_LIST: &[&[&str; 2]] = &[&["%u", "used disk space"]];

fn main() {
    let mut show_format_list = false;
    let mut format: String = String::from("DISK %u");
    let mut mounted_on: String = String::from("/");
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut mounted_on).add_option(
            &["-m", "--mounted-on"],
            Store,
            "Mount point of disk of interest",
        );
        ap.refer(&mut format)
            .add_option(&["-f", "--format"], Store, "Format string");

        ap.refer(&mut show_format_list).add_option(
            &["-v", "--format-values"],
            StoreTrue,
            "Show possible format values",
        );

        ap.parse_args_or_exit();
    }

    if show_format_list {
        utils::print_format_list(FORMAT_LIST);
        std::process::exit(0);
    }

    let Ok(disk) = statvfs(mounted_on.as_str()) else {
		eprintln!("Could not retrieve disk data");
		std::process::exit(1);
	};

    let used_bytes = utils::format_bytes(disk.blocks_available() * disk.block_size());

    let mut data: HashMap<&str, &String> = HashMap::new();
    data.insert("%u", &used_bytes);

    println!("{}", utils::format_output(data, format))
}
