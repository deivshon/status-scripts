pub mod structured;
pub mod utils;

use structured::mullvad::MullvadData;
use structured::proton::ProtonData;

use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::{collections::HashMap, fs::File};

use argparse::{ArgumentParser, Store, StoreTrue};
use serde::{Deserialize, Serialize};
use utils::failure;
use whoami;

const MULLVAD_CHECK: &str = "http://am.i.mullvad.net/json";
const SERVER_LIST_API: &str = "https://api.protonmail.ch/vpn/logicals";
const SERVER_LIST_CACHE_NAME: &str = "server-list-cache.json";
const FORMAT_LIST: &[&[&str; 2]] = &[
    &["%h", "server hostname"],
    &["%c", "country based on IP geolocation"],
];
const DEFAULT_CACHE_EXPIRY: u64 = 3600 * 12;

#[derive(Deserialize, Serialize)]
struct CachedProtonData {
    timestamp: u64,
    data: ProtonData,
}

impl ProtonData {
    fn download_server_list() -> ProtonData {
        let Ok(res) = reqwest::blocking::get(SERVER_LIST_API) else {
            eprintln!("Could not send request to {}", SERVER_LIST_API);
            std::process::exit(1);
        };

        match res.json::<ProtonData>() {
            Ok(d) => return d,
            Err(e) => failure(&format!("Could not deserialize Proton data: {}", e)),
        }
    }

    fn update_cache(cache_file: &Path, timestamp: u64) -> CachedProtonData {
        let server_list = Self::download_server_list();
        let data = CachedProtonData {
            timestamp: timestamp,
            data: server_list,
        };

        let file = match File::create(cache_file) {
            Ok(file) => file,
            Err(err) => failure(&format!("Error creating file: {}", err)),
        };

        let writer = BufWriter::new(file);
        match serde_json::to_writer(writer, &data) {
            Ok(_) => return data,
            Err(e) => failure(&format!("Could not cache server list to file: {}", e)),
        }
    }

    pub fn get(cache_file: &Path, cache_expiry: u64) -> ProtonData {
        let timestamp = std::time::SystemTime::elapsed(&UNIX_EPOCH)
            .expect("Could not get timestamp")
            .as_secs();

        if Path::exists(cache_file) {
            let file = match File::open(cache_file) {
                Ok(file) => file,
                Err(err) => failure(&format!("Error creating file: {}", err)),
            };

            let reader = BufReader::new(file);
            let cached: CachedProtonData = match serde_json::from_reader(reader) {
                Ok(d) => d,
                Err(e) => failure(&format!("Could not read cached server list file: {}", e)),
            };

            if timestamp - cached.timestamp > cache_expiry {
                let new_cached = Self::update_cache(cache_file, timestamp);
                return new_cached.data;
            } else {
                return cached.data;
            }
        } else {
            let new_cached = Self::update_cache(cache_file, timestamp);
            return new_cached.data;
        }
    }
}

struct IPData {
    ip: String,
    country: String,
}

fn get_ip() -> IPData {
    let Ok(res) = reqwest::blocking::get(MULLVAD_CHECK) else {
        eprintln!("Could not send request to {}", MULLVAD_CHECK);
        std::process::exit(1);
    };

    let mullvad_data;
    match res.json::<MullvadData>() {
        Ok(d) => mullvad_data = d,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    return IPData {
        ip: mullvad_data.ip,
        country: mullvad_data.country,
    };
}

fn main() {
    let mut show_format_list = false;
    let mut format_up: String = String::from("%h");
    let mut format_down: String = String::from("N/C - %c");
    let mut cache_expiry: u64 = DEFAULT_CACHE_EXPIRY;
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut format_up).add_option(
            &["-u", "--format-up"],
            Store,
            "Format string when connected to Proton VPN",
        );

        ap.refer(&mut format_down).add_option(
            &["-d", "--format-down"],
            Store,
            "Format string when not connected to Proton VPN",
        );

        ap.refer(&mut show_format_list).add_option(
            &["-v", "--format-values"],
            StoreTrue,
            "Show possible format values",
        );

        ap.refer(&mut cache_expiry).add_option(
            &["-c", "--cache-expiry"],
            Store,
            "Cache expiry in seconds (defaults to 12 hours)",
        );

        ap.parse_args_or_exit();
    }

    if show_format_list {
        utils::print_format_list(FORMAT_LIST);
        std::process::exit(0);
    }

    let server_list_cache = format!(
        "/home/{}/.cache/proton-status/{}",
        whoami::username(),
        SERVER_LIST_CACHE_NAME
    );

    let server_list_cache_path = Path::new(&server_list_cache);
    let Some(server_list_cache_dir) = server_list_cache_path.parent() else {
        failure("Could not get parent path of server list cache file")
    };

    match std::fs::create_dir_all(server_list_cache_dir) {
        Ok(_) => (),
        Err(e) => failure(&format!(
            "Could not create directories for server list cache file: {}",
            e
        )),
    }

    let data = ProtonData::get(server_list_cache_path, cache_expiry);
    let ip_data = get_ip();

    let mut format_data: HashMap<&str, &String> = HashMap::new();
    format_data.insert("%c", &ip_data.country);

    let mut hostname: Option<String> = None;
    for server in data.logical_servers {
        for concrete_server in server.servers {
            if ip_data.ip.eq(&concrete_server.exit_IP) {
                hostname = Some(server.name);
                break;
            }
        }

        match &hostname {
            Some(_) => break,
            None => (),
        }
    }

    match &hostname {
        Some(h) => {
            format_data.insert("%h", h);
            println!("{}", utils::format_output(format_data, format_up));
        }
        None => {
            println!("{}", utils::format_output(format_data, format_down));
        }
    }
}
