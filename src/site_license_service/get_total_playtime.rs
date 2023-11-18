//! Implements the `GetTotalPlaytime` endpoint

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, SiteLicenseServiceError},
    macros::do_http,
    Steam, BASE,
};

use super::{GameName, INTERFACE};

const ENDPOINT: &str = "GetTotalPlaytime";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    /// ID of game in the `game_names` section of the response.
    pub id: u32,

    /// Playtime is divided up for each game by the type of license used. See the `GetCurrentClientConnections` endpoint for the list.
    pub license_type: String, // TODO: Define LicenseType enum as defined in https://partner.steamgames.com/doc/webapi/isitelicenseservice

    /// Total playtime for this game and license type, in seconds, for the requested period.
    pub playtime_seconds: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Site {
    /// 64-bit SiteID of the site.
    #[serde(rename = "siteid")]
    pub id: u64,

    /// Site name assigned to the site.
    pub site_name: String,

    /// List of all game titles played.
    pub games: Vec<Game>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TotalPlaytime {
    /// A vector of site details.
    pub sites: Vec<Site>,

    /// Mapped vector of game names.
    pub game_names: Vec<GameName>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: TotalPlaytime,
}

impl Steam {
    /// Get total playtime amounts for all games over a period of time; for one or all sites.
    ///
    /// # Warning
    ///
    /// This method is unsupported as it requires a Steamworks Web API publisher authentication key and we cannot test it at the moment!
    /// Bug reports are appreciated for unsupported methods, so please file one if you encounter errors.
    ///
    /// # Arguments
    ///
    /// * `publisher_key` - Steamworks Web API publisher authentication key
    /// * `start_time` - Report activity starting on or after this time. RFC 3339 UTC format.
    /// * `end_time` - Report activity starting before this time. RFC 3339 UTC format.
    /// * `site_id` - Site ID to see; zero for all sites
    ///
    /// # Example
    ///
    /// ```
    ///     // Creates new `Steam` instance using the environment variable `STEAM_API_KEY`.
    ///     let publisher_key = &std::env::var("STEAM_PUBLISHER_API_KEY").expect("Missing an API key");
    ///
    ///     // Retrive total playtime from 01 January 2020 to 14 March 2022.
    ///     let total_playtime = Steam::get_total_playtime(
    ///         publisher_key,
    ///         "2020-01-01T04:30:10Z",
    ///         "2022-03-14T02:29:44Z",
    ///         0,
    ///     ).await.unwrap();
    ///
    ///     // Prints the total playtime data.
    ///     println!("{:?}", total_playtime);
    /// ```
    pub async fn get_total_playtime(
        publisher_key: &str,
        start_time: &str,
        end_time: &str,
        site_id: u64,
    ) -> Result<TotalPlaytime, SiteLicenseServiceError> {
        let url = format!(
            "{}/{}/{}/v{}/?key={}&start_time={}&end_time={}&siteid={}",
            BASE, INTERFACE, ENDPOINT, VERSION, publisher_key, start_time, end_time, site_id
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
