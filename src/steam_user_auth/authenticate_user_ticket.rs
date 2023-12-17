//! # Implements the `AuthenticateUserTicket` endpoint

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamUserAuthError},
    macros::do_http,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "AuthenticateUserTicket";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TicketAuthResponse {
    pub result: String,
    #[serde(rename = "steamid")]
    pub steam_id: String,
    #[serde(rename = "ownersteamid")]
    pub owner_steam_id: String,
    #[serde(rename = "vacbanned")]
    pub vac_banned: bool,
    #[serde(rename = "publisherbanned")]
    pub publisher_banned: bool,
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
        let url = format!(
            "{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}/?key={key}&appid={app_id}&ticket={ticket}"
        );
        let wrapper = do_http!(
            url,
            Wrapper,
            ErrorHandle,
            SteamUserAuthError::AuthenticateUserTicket
        );
        Ok(wrapper.response.params)
    }
}
