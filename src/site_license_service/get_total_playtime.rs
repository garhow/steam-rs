use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Steam,
    errors::{SiteLicenseServiceError, ErrorHandle},
    macros::do_http,
    BASE,
};

use super::{INTERFACE, GameName};

const ENDPOINT: &str = "GetTotalPlaytime";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    id: u32,
    license_type: String, // TODO: Define LicenseType enum as defined in https://partner.steamgames.com/doc/webapi/isitelicenseservice
    playtime_seconds: u32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Site {
    siteid: u64,
    site_name: String,
    games: Vec<Game>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TotalPlaytime {
    sites: Vec<Site>,
    game_names: Vec<GameName>
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: TotalPlaytime
}

impl Steam {
    /// This method is unsupported as it requires a Steamworks Web API publisher authentication key and we cannot test it at the moment!
    /// 
    /// Bug reports are appreciated for unsupported methods, so please file one if you encounter errors.
    pub async fn get_total_playtime(
        publisher_key: &str,
        start_time: &str,
        end_time: &str,
        siteid: u64
    ) -> Result<TotalPlaytime, SiteLicenseServiceError> {
        let url = format!("{}/{}/{}/v{}/?key={}&start_time={}&end_time={}&siteid={}", BASE, INTERFACE, ENDPOINT, VERSION, publisher_key, start_time, end_time, siteid);
        let json = do_http!(url, Value, ErrorHandle, SiteLicenseServiceError::GetCurrentClientConnections);
        let wrapper: Wrapper = ErrorHandle!(serde_json::from_value(json.to_owned()), SiteLicenseServiceError::GetCurrentClientConnections);
        Ok(wrapper.response)
    }
}
