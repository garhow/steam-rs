//! Implements the `GetPlayerAchievements` endpoint

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamUserStatsError},
    macros::{do_http, optional_argument, EndPoint},
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
    format!("{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}/?"),
    PlayerStats,
    ( steamid: SteamId,
        appid: u32
    ),
    [language: Option<String>],

    async fn internal(url: String) -> Result<PlayerStats, SteamUserStatsError> {
        let wrapper = do_http!(
            url,
            Wrapper,
            ErrorHandle,
            SteamUserStatsError::GetPlayerAchievements
        );
        Ok(wrapper.playerstats)
    }
);