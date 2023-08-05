//! # Implements the `GetServersAtAddress` endpoint

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamAppsError},
    macros::do_http,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetServersAtAddress";
const VERSION: &str = "1";

#[derive(Deserialize, Serialize, Debug)]
pub struct Server {
    /// Gives the ip address, and the port number. Ex: "64.94.100.204:27015"
    #[serde(rename = "addr")]
    pub address: String,

    /// Gives the GMS index. Ex: 65534
    #[serde(rename = "gmsindex")]
    pub gms_index: i32,

    // TODO: provide documentation for this field
    pub steamid: String,

    // TODO: provide documentation for this field
    pub reject: Option<String>,

    /// Gives the steam game appid. Ex: 730
    pub appid: u32,

    /// Tells which directory the game is from. Ex: "csgo"
    pub gamedir: String,

    /// Gives the region of the server. Ex: 1
    pub region: i32,

    /// If server is secure or not.
    pub secure: bool,
    
    /// If server is a lan game.
    pub lan: bool,

    /// Gives the port number for the server. Ex: 27015
    pub gameport: u32,

    /// Gives the specport. Ex: 0
    pub specport: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServersResponse {
    /// Returns true if IP address is valid, does not mean server is functioning properly.
    pub success: bool,

    /// A list of every server from this IP address. 
    pub servers: Option<Vec<Server>>,

    /// Message given if success is false.
    pub message: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Wrapper {
    response: ServersResponse,
}

impl Steam {
    /// Gets a list of all Steam-compatible game servers running at the specified IP address.
    /// 
    /// # Arguments
    ///
    /// * `address` - IP or IP:queryport to list.
    ///
    /// # Example
    /// 
    /// ```
    ///     // Find servers at the specified address.
    ///     let servers = Steam::get_servers_at_address("203.0.113.0").await.unwrap();
    /// 
    ///     // Prints the response.
    ///     println!("{:?}", servers);
    /// ```
    pub async fn get_servers_at_address(address: &str) -> Result<ServersResponse, SteamAppsError> {
        let url = format!(
            "{}/{}/{}/v{}/?addr={}",
            BASE, INTERFACE, ENDPOINT, VERSION, address
        );
        let wrapper = do_http!(
            url,
            Wrapper,
            ErrorHandle,
            SteamAppsError::GetServersAtAddress
        );
        Ok(wrapper.response)
    }
}
