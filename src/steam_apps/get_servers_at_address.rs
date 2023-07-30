use serde::{Deserialize, Serialize};

use crate::{
    Steam,
    BASE,
    macros::do_http,
    errors::{ErrorHandle, SteamAppsError}
};

use super::INTERFACE;

const ENDPOINT: &str = "GetServersAtAddress";
const VERSION: &str = "1";

#[derive(Deserialize, Serialize, Debug)]
pub struct Server {
    addr: String,
    gmsindex: i32,
    steamid: String,
    reject: Option<String>,
    appid: u32,
    gamedir: String,
    region: i32,
    secure: bool,
    lan: bool,
    gameport: u32,
    specport: u32
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServersResponse {
    success: bool,
    servers: Option<Vec<Server>>,
    message: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Wrapper {
    response: ServersResponse
}

impl Steam {
    /// Gets a list of game servers running at the specified IP address.
    pub async fn get_servers_at_address(address: &str) -> Result<ServersResponse, SteamAppsError> {
        let url = format!("{}/{}/{}/v{}/?addr={}", BASE, INTERFACE, ENDPOINT, VERSION, address);
        let wrapper = do_http!(url, Wrapper, ErrorHandle, SteamAppsError::GetServersAtAddress);
        Ok(wrapper.response)
    }
}