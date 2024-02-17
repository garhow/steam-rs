//! Implements the `GetCommunityBadgeProgress` endpoint.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, PlayerServiceError},
    macros::{do_http, optional_argument},
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetCommunityBadgeProgress";
const VERSION: &str = "1";

// https://wiki.teamfortress.com/wiki/WebAPI/GetCommunityBadgeProgress

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Quest {
    #[serde(rename = "questid")]
    pub quest_id: u32,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommunityBadgeProgress {
    pub quests: Option<Vec<Quest>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: CommunityBadgeProgress,
}

impl Steam {
    /// Gets all the quests needed to get the specified badge, and which are completed.
    ///
    /// # Arguments
    ///
    /// * `steam_id` - The SteamID of the player we're asking about.
    /// * `badge_id` - The badge we're asking about.
    pub async fn get_community_badge_progress(
        &self,
        steam_id: SteamId,
        badge_id: Option<i32>,
    ) -> Result<CommunityBadgeProgress, PlayerServiceError> {
        let query = format!(
            "?key={}&steamid={}{}",
            &self.api_key,
            steam_id,
            optional_argument!(badge_id, "badgeid")
        );
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let json = do_http!(
            url,
            Value,
            ErrorHandle,
            PlayerServiceError::GetCommunityBadgeProgress
        );
        let community_badge_progress: Wrapper = ErrorHandle!(
            serde_json::from_value(json.to_owned()),
            PlayerServiceError::GetCommunityBadgeProgress
        );
        Ok(community_badge_progress.response)
    }
}
