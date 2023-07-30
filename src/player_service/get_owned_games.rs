use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Steam,
    steam_id::SteamId,
    errors::{PlayerServiceError, ErrorHandle},
    macros::{do_http, optional_argument},
    BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetOwnedGames";
const VERSION: &str = "0001";

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub appid: u32,
    pub name: Option<String>,
    pub playtime_2weeks: Option<u32>,
    pub playtime_forever: u32,
    pub img_icon_url: Option<String>,
    pub img_logo_url: Option<String>,
    pub has_community_visible_stats: Option<bool>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OwnedGames {
    pub game_count: u32,
    pub games: Vec<Game>
}

#[derive(Debug, Deserialize, Serialize)]
struct Response { response: OwnedGames }

impl Steam {
    pub async fn get_owned_games(
        &self,
        steam_id: SteamId,
        include_appinfo: Option<bool>,
        include_played_free_games: Option<bool>,
        appids_filter: Option<Vec<u32>>
    ) -> Result<OwnedGames, PlayerServiceError> {
        let appids_filter: Option<String> = if let Some(appids_filter) = appids_filter {
            Some(appids_filter.iter().map(|&appid| appid.to_string() + ",").collect())
        } else { None }; // DO NOT RAYON THIS! - Rayon doesn't protect the order of data!

        let optional_arguments = [
            optional_argument!(include_appinfo),
            optional_argument!(include_played_free_games),
            optional_argument!(appids_filter)
        ];

        let query = format!("?key={}&steamid={}{}", &self.api_key, steam_id, optional_arguments.concat());
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);

        let json = do_http!(url, Value, ErrorHandle, PlayerServiceError::GetOwnedGames);
        
        let owned_games: Response = ErrorHandle!(serde_json::from_value(json.to_owned()), PlayerServiceError::GetOwnedGames);

        Ok(owned_games.response)
    }
}
