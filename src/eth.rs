pub mod utils;

use argparse::{ArgumentParser, Store};

fn main() {
	let mut format_up: String = String::from("ETH UP");
	let mut format_down: String = String::from("ETH DOWN");
	{
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format_up)
            .add_option(&["-u", "--format-up"], Store, "Format string when connected");

        ap.refer(&mut format_down)
            .add_option(&["-d", "--format-down"], Store, "Format string when not connected");

        ap.parse_args_or_exit();
    }

	let Some(_) = utils::first_matching_dir(utils::NET_DIR,
		Some(vec!["eth", "enp"]),
		Some(&utils::operstate_up)
	) else {
		println!("{}", format_down);
		std::process::exit(0);
	};

	println!("{}", format_up);
}
