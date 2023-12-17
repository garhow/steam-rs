use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamUserStatsError},
    macros::do_http,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetGlobalAchievementPercentagesForApp";
const VERSION: &str = "0002";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Achievement {
    name: String,
    percent: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AchievementPercentages {
    achievements: Vec<Achievement>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    achievementpercentages: AchievementPercentages,
}

impl Steam {
    /// Gets the global achievement percentages for an app.
    pub async fn get_global_achievement_percentages_for_app(
        &self,
        game_id: u32,
    ) -> Result<AchievementPercentages, SteamUserStatsError> {
        let query = format!("?gameid={}", game_id);
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let response = do_http!(
            url,
            Response,
            ErrorHandle,
            SteamUserStatsError::GetGlobalAchievements
        );
        Ok(response.achievementpercentages)
    }
}
