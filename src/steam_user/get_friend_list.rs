use anyhow::Result;
use serde_derive::Deserialize;
use serde_json::Value;

use crate::{Steam, SteamId};

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Relationship {
    All,
    Friend
}

#[derive(Deserialize, Debug)]
pub struct Friend {
    pub steamid: SteamId,
    pub relationship: Relationship,
    pub friend_since: u32
}

impl Steam {
    pub async fn get_friend_list(&self, steam_id: &str, relationship: Relationship) -> Result<Vec<Friend>> {
        let url = format!("https://api.steampowered.com/ISteamUser/GetFriendList/v0001/?key={}&steamid={}&relationship={}",
            &self.api_key,
            steam_id,
            if relationship == Relationship::All { "all" } else { "friends" }
        );
        let json: Value = reqwest::get(url).await.unwrap().json().await.unwrap();
        
        let mut friend_list: Vec<Friend> = Vec::new();
    
        for friend in json["friendslist"]["friends"].as_array().unwrap().to_owned() {
            let friend: Friend = serde_json::from_value(friend).unwrap();
            friend_list.push(friend);
        };
    
        return Ok(friend_list);
    }
}