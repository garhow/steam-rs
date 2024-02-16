//! Implements the `GetNumberOfCurrentPlayers` endpoint

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamUserStatsError},
    macros::{do_http, gen_args},
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

impl Steam {
    /// Retrieves the number of current players for a game.
    ///
    /// # Arguments
    ///
    /// * `appid` - The ID of the application (game) for which to retrieve the number of current players.
    ///
    /// # Example
    ///
    /// ```
    ///     // Creates new `Steam` instance using the environment variable `STEAM_API_KEY`.
    ///     let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    ///
    ///     // Retrieves asset prices of app `440` (Team Fortress 2).
    ///     let number_of_players = steam.get_number_of_current_players(440).await.unwrap();
    ///
    ///     // Prints the vector of assets.
    ///     println!("{:?}", number_of_players);
    /// ```
    pub async fn get_number_of_current_players(
        &self,
        appid: u32,
    ) -> Result<u64, SteamUserStatsError> {
        let key = &self.api_key.clone();
        let args = gen_args!(key, appid);
        let url = format!("{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}/?{args}");
        
        let wrapper = do_http!(
            url,
            Wrapper,
            ErrorHandle,
            SteamUserStatsError::GetNumberOfCurrentPlayers
        );

        match wrapper.response.result {
            1 => Ok(wrapper.response.player_count.unwrap()),
            _ => Err(SteamUserStatsError::GetNumberOfCurrentPlayers("App not found.".to_string())),
        }
    }
}