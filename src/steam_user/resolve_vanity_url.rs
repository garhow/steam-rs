//! Implements the `ResolveVanityURL` endpoint

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    errors::{ErrorHandle, SteamUserError},
    macros::{do_http, optional_argument},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "ResolveVanityURL";
const VERSION: &str = "1";

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    response: Response,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    /// The message associated with the request status. Currently only used on resolution failures.
    pub message: Option<String>,

    /// The 64 bit Steam ID the vanity URL resolves to. Not returned on resolution failures.
    pub steamid: Option<String>,

    /// The status of the request. 1 if successful, 42 if there was no match.
    pub success: u8,
}

impl Steam {
    /// Resolve vanity URL parts to a 64-bit ID.
    ///
    /// # Arguments
    ///
    /// * `vanity_url` - The user's vanity URL that you would like to retrieve a steam ID for, e.g. <http://steamcommunity.com/id/gabelogannewell> would use "gabelogannewell"
    /// * `url_type` - The type of vanity URL.
    ///     * 1 (default): Individual profile
    ///     * 2: Group,
    ///     * 3: Official game group
    ///
    /// # Example
    ///
    /// ```
    ///     // Creates new `Steam` instance using the environment variable `STEAM_API_KEY`.
    ///     let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    ///
    ///     // Resolves the vanity URL `gabelogannewell`.
    ///     let resolved_vanity_url = steam.resolve_vanity_url("gabelogannewell", None).await.unwrap();
    ///
    ///     // Prints the 64-bit ID of the user of the vanity URL
    ///     println!("{:?}", resolved_vanity_url.steamid);
    /// ```
    pub async fn resolve_vanity_url(
        &self,
        vanity_url: &str,
        url_type: Option<i32>,
    ) -> Result<Response, SteamUserError> {
        let query = vec![
            format!("?key={}", &self.api_key),
            format!("&vanityurl={}", vanity_url),
            optional_argument!(url_type),
        ];

        let url = format!(
            "{}/{}/{}/v{}/{}",
            BASE,
            INTERFACE,
            ENDPOINT,
            VERSION,
            query.concat()
        );
        let json = do_http!(url, Value, ErrorHandle, SteamUserError::ResolveVanityURL);
        let wrapper: Wrapper = ErrorHandle!(
            from_value(json.to_owned()),
            SteamUserError::ResolveVanityURL
        );

        Ok(wrapper.response)
    }
}
