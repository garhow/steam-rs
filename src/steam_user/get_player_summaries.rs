use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Steam,
    steam_id::{de_steamid_from_str, SteamId},
    macros::do_http,
    errors::{ErrorHandle, SteamUserError},
    BASE};

use super::INTERFACE;

const ENDPOINT: &str = "GetPlayerSummaries";
const VERSION: &str = "0002";

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    #[serde(rename = "steamid")]
    #[serde(deserialize_with = "de_steamid_from_str")]
    pub steam_id: SteamId,
    #[serde(rename = "personaname")]
    pub persona_name: String,
    #[serde(rename = "profileurl")]
    pub profile_url: String,
    pub avatar: String,
    #[serde(rename = "avatarmedium")]
    pub avatar_medium: String,
    #[serde(rename = "avatarfull")]
    pub avatar_full: String,
    #[serde(rename = "personastate")]
    pub persona_state: u8,
    #[serde(rename = "communityvisibilitystate")]
    pub community_visibility_state: u8,
    #[serde(rename = "profilestate")]
    pub profile_state: Option<u8>,
    #[serde(rename = "lastlogoff")]
    pub last_logoff: Option<u64>,
    #[serde(rename = "commentpermission")]
    pub comment_permission: Option<u8>,
    #[serde(rename = "realname")]
    pub real_name: Option<String>,
    #[serde(rename = "primaryclanid")]
    pub primary_clan_id: Option<String>,
    #[serde(rename = "timecreated")]
    pub time_created: Option<u64>,
    #[serde(rename = "gameid")]
    pub game_id: Option<u32>,
    #[serde(rename = "gameserverip")]
    pub game_server_ip: Option<String>,
    #[serde(rename = "gameextrainfo")]
    pub game_extra_info: Option<String>,
    #[serde(rename = "loccountrycode")]
    pub loc_country_code: Option<String>,
    #[serde(rename = "locstatecode")]
    pub loc_state_code: Option<String>,
    #[serde(rename = "loccityid")]
    pub loc_city_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSummary {
    players: Vec<Player>
}

impl Steam {
    pub async fn get_player_summaries(
        &self,
        steam_ids: Vec<SteamId>
    ) -> Result<PlayerSummary, SteamUserError> {
        let steam_ids: String = steam_ids.iter().map(|&id| id.to_string() + ",").collect(); // DO NOT RAYON THIS! - Rayon doesn't protect the order of data!

        let query = format!("?key={}&steamids={}", &self.api_key, steam_ids);
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);

        // Don't make this a one-liner, it will become too unreadable.
        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetPlayerSummaries);
        let player_summary: PlayerSummary = ErrorHandle!(serde_json::from_value(json["response"].to_owned()), SteamUserError::GetPlayerSummaries);

        return Ok(player_summary);
    }
}