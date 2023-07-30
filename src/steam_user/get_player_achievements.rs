use serde::{Deserialize, Serialize};

use crate::{macros::{gen_args, do_http, optional_argument}, errors::{SteamUserError, ErrorHandle}, Steam, steam_id::SteamId};

const END_POINT: &str = "https://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v1";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PlayerAchievements {
    #[serde(rename = "playerstats")]
    PlayerStats {
        success: bool,
        #[serde(rename = "steamID")]
        /// TODO: Make this SteamId
        steam_id: String,
        #[serde(rename = "gameName")]
        game_name: String,
        
        achievements: Vec<Achievement>
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Achievement {
    pub apiname: Option<String>,
    pub achieved: u8,
    /// TODO: Reconsider this
    pub unlocktime: Option<u64>,
}

impl Steam {
    /// This end point gives a 1:1 output of the api, if you were to put the endpoint into a browser, expect the same layout
    /// ( carbon-copy )
    pub async fn get_player_achievements(&self, steamid: SteamId, appid: u32, language: Option<&str>) -> Result<PlayerAchievements, SteamUserError> {
        let key = &self.api_key.clone();
        let steamid = steamid.into_u64();
        let args = gen_args!(key, appid, steamid) + &optional_argument!(language, "l");
        let url = format!("{END_POINT}?{args}");
        Ok(do_http!(url, PlayerAchievements, ErrorHandle, SteamUserError::GetPlayerAchievements))
    }
}

impl Achievement { pub fn achieved(&self) -> bool { self.achieved == 1 } }