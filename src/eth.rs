pub mod utils;

fn main() {
	let Some(_) = utils::first_matching_dir(utils::NET_DIR, vec!["eth", "enp"], Some(&utils::operstate_up)) else {
		std::process::exit(0);
	};

	println!("ETH UP");
}
