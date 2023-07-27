use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Steam, SteamId, AppId};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub steamid: SteamId,
    pub personaname: String,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    pub personastate: u8,
    pub communityvisibilitystate: u8,
    pub profilestate: Option<u8>,
    pub lastlogoff: u64,
    pub commentpermission: Option<u8>,
    pub realname: Option<String>,
    pub primaryclanid: Option<String>,
    pub timecreated: Option<u64>,
    pub gameid: Option<AppId>,
    pub gameserverip: Option<String>,
    pub gameextrainfo: Option<String>,
    pub loccountrycode: Option<String>,
    pub locstatecode: Option<String>,
    pub loccityid: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSummary {
    players: Vec<Player>
}

impl Steam {
    pub async fn get_player_summaries(&self, steam_ids: Vec<&str>) -> Result<PlayerSummary> {
        let steam_ids: String = steam_ids.iter().map(|&id| id.to_string() + ",").collect();
        
        let url = format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
            &self.api_key,
            steam_ids
        );

        let response = reqwest::get(url).await.unwrap();
        let json: Value = response.json().await.unwrap(); 
        
        let player_summary: PlayerSummary = serde_json::from_value(json["response"].to_owned()).unwrap();

        return Ok(player_summary);
    }
}