//! Implements the `GetBadges` endpoint.

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Badge {
    pub badgeid: u32,
    pub level: u32,
    pub completion_time: u32,
    pub xp: u32,
    pub scarcity: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    /// Gets badges that are owned by a specific user.
    ///
    /// # Arguments
    ///
    /// * `steam_id` - The SteamID of the player we're asking about.
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
