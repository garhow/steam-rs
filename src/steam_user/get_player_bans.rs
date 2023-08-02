//! Implements the `GetPlayerBans` endpoint

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    Steam,
    steam_id::{de_steamid_from_str, SteamId},
    macros::do_http,
    errors::{ErrorHandle, SteamUserError},
    BASE
};

use super::INTERFACE;

const ENDPOINT: &str = "GetPlayerBans";
const VERSION: &str = "1";

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    /// Vector of player objects for each given (and valid) 64-bit ID. 
    players: Vec<Player>
}

/// Represents a player object with data about bans.
#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    /// A string containing the player's 64-bit ID.
    #[serde(rename = "SteamId")]
    pub steam_id: String, // TODO: Convert to SteamId

    /// Boolean indicating whether or not the player is banned
    /// from the [Steam Community](http://steamcommunity.com/).
    #[serde(rename = "CommunityBanned")]
    pub community_banned: bool,

    /// Boolean indicating whether or not the player has VAC bans on record.
    #[serde(rename = "VACBanned")]
    pub vac_banned: bool,

    /// Number of VAC bans.
    #[serde(rename = "NumberOfVACBans")]
    pub number_of_vac_bans: u32,

    /// Days since the player's last ban.
    #[serde(rename = "DaysSinceLastBan")]
    pub days_since_last_ban: u32,

    /// Number of bans in games.
    #[serde(rename = "NumberOfGameBans")]
    pub number_of_game_bans: u32,

    /// String containing the player's ban status in the economy.
    /// If the player has no bans on record the string will be "none",
    /// if the player is on probation it will say "probation", and so forth.
    #[serde(rename = "EconomyBan")]
    pub economy_ban: String
}

impl Steam {
    /// Get player ban/probation status.
    ///
    /// # Arguments
    ///
    /// * `steam_ids` - A vector of `SteamId` objects
    ///
    /// # Example
    /// 
    /// ```
    ///     // Creates new `Steam` instance using the environment variable `STEAM_API_KEY`.
    ///     let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    /// 
    ///     // Retrieves player ban status of user `76561197960435530`.
    ///     let player_bans = steam.get_player_bans(vec![SteamId(76561197960435530)]).await.unwrap();
    /// 
    ///     // Prints the user's VAC ban status.
    ///     println!("{:?}", player_bans[0].vac_banned);
    /// ```
    pub async fn get_player_bans(
        &self,
        steam_ids: Vec<SteamId>
    ) -> Result<Vec<Player>, SteamUserError> {
        let steam_ids: String = steam_ids.iter().map(|&id| id.to_string() + ",").collect();

        let query = format!("?key={}&steamids={}", &self.api_key, steam_ids);
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);

        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetPlayerBans);
        let wrapper: Wrapper = ErrorHandle!(from_value(json.to_owned()), SteamUserError::GetPlayerBans);

        Ok(wrapper.players)
    }
}