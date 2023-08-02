//! Implements the `GetPlayerAchievements` endpoint

use serde::{Deserialize, Serialize};

use crate::{
    macros::{gen_args, do_http, optional_argument},
    errors::{ErrorHandle, SteamUserStatsError},
    Steam,
    steam_id::SteamId,
    BASE
};

use super::INTERFACE;

const ENDPOINT: &str = "GetPlayerAchievements";
const VERSION: &str = "1";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Wrapper {
    playerstats: PlayerStats
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
    pub achievements: Option<Vec<Achievement>>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Achievement {
    pub apiname: Option<String>,
    pub achieved: u8,
    // TODO: Reconsider this
    pub unlocktime: Option<u64>,
}

impl Steam {
    pub async fn get_player_achievements(
        &self,
        steamid: SteamId,
        appid: u32,
        language: Option<&str>
    ) -> Result<PlayerStats, SteamUserStatsError> {
        let key = &self.api_key.clone();
        let steamid = steamid.into_u64();
        let args = gen_args!(key, appid, steamid) + &optional_argument!(language, "l");
        let url = format!("{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}/?{args}");
        let wrapper = do_http!(url, Wrapper, ErrorHandle, SteamUserStatsError::GetPlayerAchievements);
        Ok(wrapper.playerstats)
    }
}

impl Achievement { pub fn achieved(&self) -> bool { self.achieved == 1 } }