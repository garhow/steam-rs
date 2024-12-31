//! Implements the 'GetAccountPublicInfo' endpoint.
//! TODO: ALL THE DOCUMENTATION

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    errors::{ErrorHandle, GameServersServiceError},
    macros::do_http,
    steam_id::{de_steamid_from_str_opt, SteamId},
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
    #[serde(deserialize_with = "de_steamid_from_str_opt")]
    steam_id: Option<SteamId>,
    appid: Option<u32>,
}

impl Steam {
    pub async fn get_account_public_info(
        &self,
        steam_id: SteamId,
    ) -> Result<PublicInfoResponse, GameServersServiceError> {
        let query = format!("?key={}&steamid={}", &self.api_key, steam_id.0);
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
