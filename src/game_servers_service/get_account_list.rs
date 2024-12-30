//! Implements the 'GetAccountList' endpoint.
//! TODO: ALL THE DOCUMENTATION

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    errors::{ErrorHandle, GameServersServiceError},
    macros::do_http,
    steam_id::{de_steamid_from_str, SteamId},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetAccountList";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    #[serde(rename = "steamid")]
    #[serde(deserialize_with = "de_steamid_from_str")]
    pub steam_id: SteamId,
    pub appid: u32,
    pub login_token: String,
    pub memo: String,
    pub is_deleted: bool,
    pub is_expired: bool,
    pub rt_last_logon: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountsResponse {
    pub servers: Vec<Server>,
    pub is_banned: bool,
    pub expires: u32,
    #[serde(deserialize_with = "de_steamid_from_str")]
    pub actor: SteamId,
    last_action_time: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: AccountsResponse,
}

impl Steam {
    /// Get the List of server accounts and details on them
    pub async fn get_account_list(&self) -> Result<AccountsResponse, GameServersServiceError> {
        let query = format!("?key={}", &self.api_key);
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let json = do_http!(
            url,
            Value,
            ErrorHandle,
            GameServersServiceError::GetAccountList
        );
        let wrapper: Wrapper = ErrorHandle!(
            from_value(json.to_owned()),
            GameServersServiceError::GetAccountList
        );
        Ok(wrapper.response)
    }
}
