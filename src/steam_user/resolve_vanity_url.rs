use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};

use crate::{
    Steam,
    macros::{do_http, optional_argument},
    errors::{ErrorHandle, SteamUserError},
    BASE
};

use super::INTERFACE;

const ENDPOINT: &str = "ResolveVanityURL";
const VERSION: &str = "1";

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    response: Response
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    message: Option<String>, // The message associated with the request status. Currently only used on resolution failures.
    steamid: Option<String>, // The 64 bit Steam ID the vanity URL resolves to. Not returned on resolution failures.
    success: u8 // The status of the request. 1 if successful, 42 if there was no match.
}

impl Steam {
    pub async fn resolve_vanity_url(
        &self,
        vanity_url: &str, // The vanity URL to get a SteamID for
        url_type: Option<i32> // The type of vanity URL. 1 (default): Individual profile, 2: Group, 3: Official game group
    ) -> Result<Response, SteamUserError> {
        let query = vec![
            format!("?key={}", &self.api_key),
            format!("&vanityurl={}", vanity_url),
            optional_argument!(url_type)
        ];

        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query.concat());
        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetPlayerSummaries);
        let wrapper: Wrapper = ErrorHandle!(from_value(json.to_owned()), SteamUserError::GetPlayerSummaries);

        Ok(wrapper.response)
    }
}