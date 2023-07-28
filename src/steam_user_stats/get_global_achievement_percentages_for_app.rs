use serde_derive::{Deserialize, Serialize};

use crate::{Steam, macros::do_http, errors::{ErrorHandle, SteamUserStatsError}};

#[derive(Deserialize, Serialize, Debug)]
pub struct Achievement {
    name: String,
    percent: f64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AchievementPercentages {
    achievements: Vec<Achievement>
}

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    achievementpercentages: AchievementPercentages
}

impl Steam {
    /// Gets the global achievement percentages for an app.
    pub async fn get_global_achievement_percentages_for_app(&self, game_id: u32) -> Result<AchievementPercentages, SteamUserStatsError> {
        let url = format!("https://api.steampowered.com/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v0002/?gameid={}", game_id);

        let response = do_http!(url, Response, ErrorHandle, SteamUserStatsError::GetGlobalAchievements);

        Ok(response.achievementpercentages)
    }
}