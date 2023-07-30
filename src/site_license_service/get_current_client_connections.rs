use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Steam,
    errors::{SiteLicenseServiceError, ErrorHandle},
    macros::do_http,
    BASE,
};

use super::{INTERFACE, GameName};

const ENDPOINT: &str = "GetCurrentClientConnections";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    id: u32,
    start_time: String,
    license_type: String // TODO: Define LicenseType enum as defined in https://partner.steamgames.com/doc/webapi/isitelicenseservice
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    ip_address: String,
    hostname: String,
    games: Vec<Game>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Site {
    siteid: u64,
    site_name: String,
    clients: Vec<Client>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentClientConnections {
    sites: Vec<Site>,
    game_names: Vec<GameName>
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: CurrentClientConnections
}

impl Steam {
    /// This method is unsupported as it requires a Steamworks Web API publisher authentication key and we cannot test it at the moment!
    /// 
    /// Bug reports are appreciated for unsupported methods, so please file one if you encounter errors.
    pub async fn get_current_client_connections(
        publisher_key: &str,
        siteid: u64
    ) -> Result<CurrentClientConnections, SiteLicenseServiceError> {
        let url = format!("{}/{}/{}/v{}/?key={}&siteid={}", BASE, INTERFACE, ENDPOINT, VERSION, publisher_key, siteid);
        let json = do_http!(url, Value, ErrorHandle, SiteLicenseServiceError::GetCurrentClientConnections);
        let wrapper: Wrapper = ErrorHandle!(serde_json::from_value(json.to_owned()), SiteLicenseServiceError::GetCurrentClientConnections);
        Ok(wrapper.response)
    }
}
