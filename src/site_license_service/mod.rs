use serde::{Deserialize, Serialize};

pub mod get_current_client_connections;
pub mod get_total_playtime;

const INTERFACE: &str = "ISiteLicenseService";

#[derive(Debug, Deserialize, Serialize)]
pub struct GameName {
    id: u32,
    name: String
}