//! Implements the `QueryLoginToken` endpoint

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    errors::{ErrorHandle, GameServersServiceError},
    macros::do_http,
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "QueryLoginToken";
const VERSION: &str = "1";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Wrapper {
    response: LoginTokenResponse,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginTokenResponse {
    is_banned: bool,
    expires: u32,
    #[serde(rename = "steamid")]
    server_steam_id: SteamId,
}

impl Steam {
    // Get the information on a login token for a server
    //
    // # Arguments
    // * login_token : Token for a server, can be sourced from 'get_account_list'
    pub async fn query_login_token(
        &self,
        login_token: &str,
    ) -> Result<LoginTokenResponse, GameServersServiceError> {
        let query = format!("?key={}&login_token={}", &self.api_key, login_token);
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let json = do_http!(
            url,
            Value,
            ErrorHandle,
            GameServersServiceError::QueryLoginToken
        );
        let wrapper: Wrapper = ErrorHandle!(
            from_value(json.to_owned()),
            GameServersServiceError::QueryLoginToken
        );
        Ok(wrapper.response)
    }
}
