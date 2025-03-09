//! Implements the `GetNumberOfCurrentPlayers` endpoint

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamUserStatsError},
    macros::{do_http, EndPoint},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetNumberOfCurrentPlayers";
const VERSION: &str = "1";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Wrapper {
    response: Response,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Response {
    player_count: Option<u64>,
    result: u8,
}

EndPoint!(
    get_number_of_current_players,
    NumberOfCurrentPlayersReq,
    format!("{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}/?"),
    u64,
    ( appid: u32 ),
    [ ],
    async fn internal(url: String) -> Result<u64, SteamUserStatsError> {
        let wrapper = do_http!(
            url,
            Wrapper,
            ErrorHandle,
            SteamUserStatsError::GetNumberOfCurrentPlayers
        );

        match wrapper.response.result {
            1 => Ok(wrapper.response.player_count.unwrap()),
            _ => Err(SteamUserStatsError::GetNumberOfCurrentPlayers(
                "App not found.".to_string(),
            )),
        }
        
    }
);