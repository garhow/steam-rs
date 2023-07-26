use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::Steam;

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
    pub async fn get_global_achievement_percentages_for_app(&self, game_id: u32) -> Result<AchievementPercentages> {
        let url = format!("https://api.steampowered.com/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v0002/?gameid={}", game_id);

        let json: Value = reqwest::get(url).await.unwrap().json().await.unwrap(); 

        let response: Response = serde_json::from_value(json).unwrap();

        Ok(response.achievementpercentages)
    }
}