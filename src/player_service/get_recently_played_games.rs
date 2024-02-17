//! Implements the `GetRecentlyPlayedGames` endpoint.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, PlayerServiceError},
    macros::{do_http, optional_argument},
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetRecentlyPlayedGames";
const VERSION: &str = "0001";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    pub appid: u32,
    pub name: Option<String>,
    pub playtime_2weeks: Option<u32>,
    pub playtime_forever: u32,
    pub img_icon_url: Option<String>,
    pub img_logo_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecentlyPlayedGames {
    pub total_count: u32,
    pub games: Vec<Game>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    response: RecentlyPlayedGames,
}

impl Steam {
    /// Gets information about a player's recently played games
    ///
    /// # Arguments
    ///
    /// * `steam_id` - The SteamID of the player we're asking about.
    /// * `count` - The number of games to return.
    pub async fn get_recently_played_games(
        &self,
        steam_id: SteamId,
        count: Option<u64>,
    ) -> Result<RecentlyPlayedGames, PlayerServiceError> {
        let query = format!(
            "?key={}&steamid={}{}",
            &self.api_key,
            steam_id,
            optional_argument!(count)
        );
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let json = do_http!(url, Value, ErrorHandle, PlayerServiceError::GetOwnedGames);
        let recently_played_games: Response = ErrorHandle!(
            serde_json::from_value(json.to_owned()),
            PlayerServiceError::GetRecentlyPlayedGames
        );
        Ok(recently_played_games.response)
    }
}
