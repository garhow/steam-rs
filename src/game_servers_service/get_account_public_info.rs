//! Implements the 'GetAccountPublicInfo' endpoint.

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    errors::{ErrorHandle, GameServersServiceError},
    macros::do_http,
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetAccountPublicInfo";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
pub struct Wrapper {
    response: PublicInfoResponse,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PublicInfoResponse {
    #[serde(rename = "steamid")]
    server_steam_id: Option<SteamId>,
    appid: Option<u32>,
}

impl Steam {
    // Get the public accessable info about a steam server from it's steam id.
    //
    // # Arguments
    // * `server_steam_id` - The server's steam id, this can be got from such functions as 'get_account_list` user accounts wont return anything
    pub async fn get_account_public_info(
        &self,
        server_steam_id: SteamId,
    ) -> Result<PublicInfoResponse, GameServersServiceError> {
        let query = format!("?key={}&steamid={}", &self.api_key, server_steam_id.0);
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let json = do_http!(
            url,
            Value,
            ErrorHandle,
            GameServersServiceError::GetAccountPublicInfo
        );
        let wrapper: Wrapper = ErrorHandle!(
            from_value(json.to_owned()),
            GameServersServiceError::GetAccountPublicInfo
        );
        Ok(wrapper.response)
    }
}
