use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Steam, SteamId, AppId, macros::do_http, errors::{ErrorHandle, SteamUserError}};

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
    pub async fn get_player_summaries(&self, steam_ids: Vec<&str>) -> Result<PlayerSummary, SteamUserError> {
        let steam_ids: String = steam_ids.iter().map(|&id| id.to_string() + ",").collect(); // DO NOT RAYON THIS! - Rayon doesn't protect the order of data!
        
        let url = format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
            &self.api_key,
            steam_ids
        );

        // Don't make this a one-liner, it will become too unreadable.
        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetPlayerSummaries);
        let player_summary: PlayerSummary = ErrorHandle!(serde_json::from_value(json["response"].to_owned()), SteamUserError::GetPlayerSummaries);

        return Ok(player_summary);
    }
}