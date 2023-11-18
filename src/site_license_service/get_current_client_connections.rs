//! Implements the `GetCurrentClientConnections` endpoint

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, SiteLicenseServiceError},
    macros::do_http,
    Steam, BASE,
};

use super::{GameName, INTERFACE};

const ENDPOINT: &str = "GetCurrentClientConnections";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    /// ID of game in the `game_names` section of the response.
    pub id: u32,

    /// Time this game session started (RFC 3339 UTC format).
    pub start_time: String,

    /// Playtime is divided up for each game by the type of license used. See the GetCurrentClientConnections section above for the list.
    pub license_type: String, // TODO: Define LicenseType enum as defined in https://partner.steamgames.com/doc/webapi/isitelicenseservice
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    /// IP address of client computer.
    pub ip_address: String,

    /// Machine name of client computer.
    pub hostname: String,

    /// Vector of active game sessions on the client computer.
    pub games: Vec<Game>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Site {
    /// 64-bit SiteID of the site.
    #[serde(rename = "siteid")]
    pub id: u64,

    /// Site name assigned to the site.
    pub site_name: String,

    /// Vector of active clients at the site.
    pub clients: Vec<Client>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentClientConnections {
    /// A vector of site details.
    pub sites: Vec<Site>,

    /// Mapped vector of game names.
    pub game_names: Vec<GameName>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: CurrentClientConnections,
}

impl Steam {
    /// See current activity at one or more sites.
    ///
    /// # Warning
    ///
    /// This method is unsupported as it requires a Steamworks Web API publisher authentication key and we cannot test it at the moment!
    /// Bug reports are appreciated for unsupported methods, so please file one if you encounter errors.
    ///
    /// # Arguments
    ///
    /// * `publisher_key` - Steamworks Web API publisher authentication key
    /// * `site_id` - Site ID to see; zero for all sites
    ///
    /// # Example
    ///
    /// ```
    ///     // Creates new `Steam` instance using the environment variable `STEAM_API_KEY`.
    ///     let publisher_key = &std::env::var("STEAM_PUBLISHER_API_KEY").expect("Missing an API key");
    ///
    ///     // Retrive current client connections at all sites.
    ///     let current_connections = Steam::get_current_client_connections(publisher_key, 0).await.unwrap();
    ///
    ///     // Prints the client activity data.
    ///     println!("{:?}", current_connections);
    /// ```
    pub async fn get_current_client_connections(
        publisher_key: &str,
        site_id: u64,
    ) -> Result<CurrentClientConnections, SiteLicenseServiceError> {
        let url = format!(
            "{}/{}/{}/v{}/?key={}&siteid={}",
            BASE, INTERFACE, ENDPOINT, VERSION, publisher_key, site_id
        );
        let json = do_http!(
            url,
            Value,
            ErrorHandle,
            SiteLicenseServiceError::GetCurrentClientConnections
        );
        let wrapper: Wrapper = ErrorHandle!(
            serde_json::from_value(json.to_owned()),
            SiteLicenseServiceError::GetCurrentClientConnections
        );
        Ok(wrapper.response)
    }
}
