//! Implements the `GetSchemaForGame` endpoint

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamUserStatsError},
    macros::{do_http, optional_argument, EndPoint},
    Steam, BASE,
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
    pub available_game_stats: AvailableGameStats,
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

EndPoint!(
    get_schema_for_game,
    SchemaForGameReq,
    format!("{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}/?"),
    Game,
    ( appid: u32 ),
    [language: Option<String>],
    async fn internal(url: String) -> Result<Game, SteamUserStatsError> {
        let response = do_http!(
            url,
            Response,
            ErrorHandle,
            SteamUserStatsError::GetSchemaForGame
        );
        Ok(response.game)
    }
);