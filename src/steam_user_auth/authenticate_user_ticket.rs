//! # Implements the `AuthenticateUserTicket` endpoint

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamUserAuthError},
    macros::{do_http, gen_args},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "AuthenticateUserTicket";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
pub struct TicketAuthResponse {
    result: String,
    #[serde(rename = "steamid")]
    steam_id: String,
    #[serde(rename = "ownersteamid")]
    owner_steam_id: String,
    #[serde(rename = "vacbanned")]
    vac_banned: bool,
    #[serde(rename = "publisherbanned")]
    publisher_banned: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WrapperParams {
    params: TicketAuthResponse,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: WrapperParams,
}

impl Steam {
    pub async fn authenticate_user_ticket(
        &self,
        app_id: u32,
        ticket: &str,
    ) -> Result<TicketAuthResponse, SteamUserAuthError> {
        let key = &self.api_key.clone();
        let args = gen_args!(key, app_id, ticket);
        let url = format!("{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}/?{args}");
        let wrapper = do_http!(
            url,
            Wrapper,
            ErrorHandle,
            SteamUserAuthError::AuthenticateUserTicket
        );
        Ok(wrapper.response.params)
    }
}
