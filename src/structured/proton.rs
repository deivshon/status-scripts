use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConcreteServerData {
    pub entry_IP: String,
    pub exit_IP: String,
    pub domain: String,
    pub ID: String,
    pub label: String,
    pub x25519_public_key: Option<String>,
    pub generation: i32,
    pub status: i32,
    pub services_down: i32,
    pub services_down_reason: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationData {
    pub lat: f32,
    pub long: f32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LogicalServerData {
    pub name: String,
    pub entry_country: String,
    pub exit_country: String,
    pub domain: String,
    pub tier: i32,
    pub features: i32,
    pub region: Option<String>,
    pub city: Option<String>,
    pub score: f64,
    pub host_country: Option<String>,
    pub ID: String,
    pub location: LocationData,
    pub servers: Vec<ConcreteServerData>,
    pub load: i32,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProtonData {
    pub code: i32,
    pub logical_servers: Vec<LogicalServerData>,
}
