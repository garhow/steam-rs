use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, PlayerServiceError},
    macros::do_http,
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetBadges";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
pub struct Badge {
    badgeid: u32,
    level: u32,
    completion_time: u32,
    xp: u32,
    scarcity: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BadgeResponse {
    pub badges: Vec<Badge>,
    pub player_xp: u32,
    pub player_level: u32,
    pub player_xp_needed_to_level_up: u32,
    pub player_xp_needed_current_level: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: BadgeResponse,
}

impl Steam {
    pub async fn get_badges(&self, steam_id: SteamId) -> Result<BadgeResponse, PlayerServiceError> {
        let query = format!("?key={}&steamid={}", &self.api_key, steam_id);
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let json = do_http!(url, Value, ErrorHandle, PlayerServiceError::GetOwnedGames);
        let wrapper: Wrapper = ErrorHandle!(
            serde_json::from_value(json.to_owned()),
            PlayerServiceError::GetBadges
        );
        Ok(wrapper.response)
    }
}
