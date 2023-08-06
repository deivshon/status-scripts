use serde::Deserialize;

#[derive(Deserialize)]
pub struct Blacklist {
    pub name: String,
    pub link: String,
    pub blacklisted: bool,
}

#[derive(Deserialize)]
pub struct Blacklisted {
    pub blacklisted: bool,
    pub results: Vec<Blacklist>,
}

#[derive(Deserialize)]
pub struct MullvadData {
    pub ip: String,
    pub country: String,
    pub city: String,
    pub longitude: f32,
    pub latitude: f32,
    pub mullvad_exit_ip: bool,
    pub mullvad_exit_ip_hostname: Option<String>,
    pub mullvad_server_type: Option<String>,
    pub blacklisted: Blacklisted,
    pub organization: String,
}
