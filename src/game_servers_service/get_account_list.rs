//! Implements the 'GetAccountList' endpoint.

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    errors::{ErrorHandle, GameServersServiceError},
    macros::do_http,
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetAccountList";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    #[serde(rename = "steamid")]
    pub server_steam_id: SteamId,
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
    pub actor: SteamId,
    pub last_action_time: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: AccountsResponse,
}

impl Steam {
    /// Get the List of server accounts linked to the account the steam key is connected to
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
