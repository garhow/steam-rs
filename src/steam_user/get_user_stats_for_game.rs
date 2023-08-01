use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{macros::{gen_args, do_http}, errors::{SteamUserError, ErrorHandle}, Steam, steam_id::SteamId};

use super::get_player_achievements::Achievement;

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
        stats: HashMap<String, Stat>
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stat {
    pub value: u64
}

impl Steam {
    /// This end point gives a 1:1 output of the api, if you were to put the endpoint into a browser, expect the same layout
    /// ( carbon-copy )
    pub async fn get_user_stats_for_game(&self, steamid: SteamId, appid: u32) -> Result<UserGameStats, SteamUserError> {
        let key = &self.api_key.clone();
        let steamid = steamid.into_u64();
        let args = gen_args!(key, appid, steamid);
        let url = format!("{END_POINT}?{args}");
        Ok(do_http!(url, UserGameStats, ErrorHandle, SteamUserError::GetUserStatsForGame))
    }
}