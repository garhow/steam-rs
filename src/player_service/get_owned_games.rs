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
    pub name: String,
    pub playtime_forever: u64,
    pub img_icon_url: String,
    pub capsule_filename: Option<String>,
    pub has_workshop: bool,
    pub has_market: bool,
    pub has_dlc: bool,
}

impl Steam {
    /// TODO: Extensive testing for each argument
    pub async fn get_owned_games(
        &self,
        steamid: SteamId,
        include_appinfo: bool,
        include_played_free_games: bool,
        appid_filter: u32, // TODO: should this be an option?
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
            appid_filter,
            include_free_sub,
            language,
            include_extended_appinfo
        ) + &optional_argument!(skip_unvetted_apps);
        let url = format!("{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}/?{args}");
        let wrapper = do_http!(url, Wrapper, ErrorHandle, PlayerServiceError::GetOwnedGames);
        Ok(wrapper.response)
    }
}
