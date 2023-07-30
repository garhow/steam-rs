use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Steam,
    steam_id::SteamId,
    errors::{PlayerServiceError, ErrorHandle},
    macros::do_http,
    BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetSteamLevel";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
struct Response { pub player_level: u16 }

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper { response: Response }

impl Steam {
    pub async fn get_steam_level(&self, steam_id: SteamId) -> Result<u16, PlayerServiceError> {
        let query = format!("?key={}&steamid={}", &self.api_key, steam_id);
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let json = do_http!(url, Value, ErrorHandle, PlayerServiceError::GetOwnedGames);
        let player_level: Wrapper = ErrorHandle!(serde_json::from_value(json.to_owned()), PlayerServiceError::GetOwnedGames);
        Ok(player_level.response.player_level)
    }
}
