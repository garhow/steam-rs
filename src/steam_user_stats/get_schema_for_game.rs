//! Implements the `GetSchemaForGame` endpoint

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamUserStatsError}, macros::{do_http, gen_args, optional_argument}, Steam, BASE
};

use super::INTERFACE;

const ENDPOINT: &str = "GetSchemaForGame";
const VERSION: &str = "2";

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    game: Game,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "availableGameStats")]
    pub available_game_stats: AvailableGameStats
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AvailableGameStats {
    pub stats: Vec<Stat>,
    pub achievements: Vec<Achievement>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Stat {
    pub name: String,
    #[serde(rename = "defaultvalue")]
    pub default_value: i32,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Achievement {
    pub name: String,
    #[serde(rename = "defaultvalue")]
    pub default_value: i32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub hidden: i8,
    pub description: String,
    pub icon: String,
    #[serde(rename = "icongray")]
    pub icon_gray: String,
}

impl Steam {
    /// Gets the complete list of stats and achievements for the specified game.
    ///
    /// # Arguments
    ///
    /// * `appid` - The ID of the application (game) for which to retrieve the number of current players.
    /// * `language` - Localized language to return (english, french, etc.).
    pub async fn get_schema_for_game(
        &self,
        appid: u32,
        language: Option<&str>
    ) -> Result<Game, SteamUserStatsError> {
        let key = &self.api_key;
        let query = format!("?key={}{}{}", key, gen_args!(appid), optional_argument!(language, "l"));
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let response = do_http!(
            url,
            Response,
            ErrorHandle,
            SteamUserStatsError::GetSchemaForGame
        );
        Ok(response.game)
    }
}
