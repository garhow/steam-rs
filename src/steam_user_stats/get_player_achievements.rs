//! Implements the `GetPlayerAchievements` endpoint


use crate::macros::SteamAPI;
use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamUserStatsError},
    macros::{do_http, gen_args, optional_argument, EndPoint},
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetPlayerAchievements";
const VERSION: &str = "1";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Wrapper {
    playerstats: PlayerStats,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerStats {
    pub error: Option<String>,
    pub success: bool,
    #[serde(rename = "steamID")]
    // TODO: Make this SteamId
    pub steam_id: Option<String>,
    #[serde(rename = "gameName")]
    pub game_name: Option<String>,
    pub achievements: Option<Vec<Achievement>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Achievement {
    pub apiname: Option<String>,
    pub achieved: u8,
    // TODO: Reconsider this
    pub unlocktime: Option<u64>,
}

impl Achievement {
    pub fn achieved(&self) -> bool {
        self.achieved == 1
    }
}

EndPoint!(
    get_player_achievements,
    PlayerAchievementsReq,
    PlayerStats,
    ( steamid: SteamId,
        appid: u32
    ),
    [language: Option<String>]
);
