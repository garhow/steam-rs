use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{macros::{gen_args, do_http, optional_argument}, errors::{SteamUserError, ErrorHandle}, Steam, steam_id::SteamId};

use super::get_player_achievements::Achievement;

const END_POINT: &str = "https://api.steampowered.com/IPlayerService/GetOwnedGames/v1";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OwnedGames {
    #[serde(rename = "response")]
    Response {
        game_count: u64,
        games: Vec<OwnedGame>
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OwnedGame {
    pub appid: u32,
    pub name: String,
    pub playtime_forever: u64,
    pub img_icon_url: String,
    pub capsule_filename: Option<String>,
    pub has_workshop: bool,
    pub has_market: bool,
    pub has_dlc: bool
}

impl Steam {
    /// This end point gives a 1:1 output of the api, if you were to put the endpoint into a browser, expect the same layout
    /// ( carbon-copy )
    /// TODO: Extensive testing for each argument
    pub async fn get_owned_game(&self, steamid: SteamId, include_appinfo: bool, include_played_free_games: bool, appid_filter: u32, include_free_sub: bool, skip_unvetted_apps: Option<bool>, language: &str, include_extended_appinfo: bool) -> Result<OwnedGames, SteamUserError> {
        let key = &self.api_key.clone();
        let steamid = steamid.into_u64();
        let args = gen_args!(key, steamid, include_appinfo, include_played_free_games, appid_filter, include_free_sub, language, include_extended_appinfo) + &optional_argument!(skip_unvetted_apps);
        let url = format!("{END_POINT}?{args}");
        Ok(do_http!(url, OwnedGames, ErrorHandle, SteamUserError::GetUserStatsForGame))
    }
}