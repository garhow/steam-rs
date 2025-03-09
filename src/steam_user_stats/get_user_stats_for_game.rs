use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamUserStatsError},
    macros::{do_http, EndPoint},
    steam_id::SteamId,
    Steam,
};

const END_POINT: &str = "https://api.steampowered.com/ISteamUserStats/GetUserStatsForGame/v1";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UserGameStats {
    #[serde(rename = "playerstats")]
    PlayerStats {
        #[serde(rename = "steamID")]
        /// TODO: Make this SteamId
        steam_id: String,
        #[serde(rename = "gameName")]
        game_name: String,
        achievements: HashMap<String, Achievement>,
        stats: HashMap<String, Stat>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Achievement {
    pub apiname: Option<String>,
    pub achieved: u8,
    /// TODO: Reconsider this
    pub unlocktime: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stat {
    pub value: u64,
}

EndPoint!(
    get_user_stats_for_game,
    UserGameStatsReq,
    format!("{END_POINT}?"),
    UserGameStats,
    ( steamid: SteamId, appid: u32 ),
    [ ], // TODO: Remove the need to do this
    async fn internal(url: String) -> Result<UserGameStats, SteamUserStatsError> {
        Ok(do_http!(url, UserGameStats, ErrorHandle, SteamUserStatsError::GetUserStatsForGame))
    }
);