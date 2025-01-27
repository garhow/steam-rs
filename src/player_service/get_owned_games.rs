//! Implements the `GetOwnedGames` endpoint.

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, PlayerServiceError},
    macros::{do_http, gen_args, optional_argument},
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetOwnedGames";
const VERSION: &str = "1";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Wrapper {
    response: OwnedGames,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OwnedGames {
    pub game_count: u64,
    pub games: Vec<Game>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Game {
    pub appid: u32,
    pub name: Option<String>,
    pub playtime_forever: u64,
    pub img_icon_url: Option<String>,
    pub capsule_filename: Option<String>,
    pub has_workshop: Option<bool>,
    pub has_market: Option<bool>,
    pub has_dlc: Option<bool>,
    pub rtime_last_played: Option<u64>,
}

impl Steam {
    /// Return a list of games owned by the player.
    ///
    /// # Arguments
    ///
    /// * `steamid` - The SteamID of the player we're asking about.
    /// * `include_appinfo` - True if we want additional details (name, icon) about each game.
    /// * `include_played_free_games` - Free games are excluded by default. If this is set, free games the user has played will be returned.
    /// * `appids_filter` - If set, restricts result set to the passed in apps.
    /// * `include_free_sub` - Some games are in the free sub, which are excluded by default.
    /// * `skip_unvetted_apps` - If set, skip unvetted store apps.
    /// * `language` - Will return data in this language (english, french, etc.).
    /// * `include_extended_appinfo` - True if we want even more details (capsule, sortas, and capabilities) about each game. include_appinfo must also be true.
    pub async fn get_owned_games(
        // TODO: Extensive testing for each argument
        &self,
        steamid: SteamId,
        include_appinfo: bool,
        include_played_free_games: bool,
        appids_filter: u32, // TODO: should this be an option?
        include_free_sub: bool,
        skip_unvetted_apps: Option<bool>,
        language: &str,
        include_extended_appinfo: bool,
    ) -> Result<OwnedGames, PlayerServiceError> {
        let key = &self.api_key.clone();
        let steamid = steamid.into_u64();
        let args = gen_args!(
            key,
            steamid,
            include_appinfo,
            include_played_free_games,
            appids_filter,
            include_free_sub,
            language,
            include_extended_appinfo
        ) + &optional_argument!(skip_unvetted_apps);
        let url = format!("{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}/?{args}");
        let wrapper = do_http!(url, Wrapper, ErrorHandle, PlayerServiceError::GetOwnedGames);
        Ok(wrapper.response)
    }
}
