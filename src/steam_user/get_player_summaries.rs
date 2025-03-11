//! Implements the `GetPlayerSummaries` endpoint

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    errors::{ErrorHandle, SteamUserError},
    macros::do_http,
    steam_id::{de_steamid_from_str, SteamId},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetPlayerSummaries";
const VERSION: &str = "0002";

/// Represents a user profile object.
///
/// Contained information varies depending on whether or not the user has their profile set to Friends only or Private.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    /// The user's 64-bit ID
    #[serde(rename = "steamid")]
    #[serde(deserialize_with = "de_steamid_from_str")]
    pub steam_id: SteamId,

    /// User's display name.
    #[serde(rename = "personaname")]
    pub persona_name: String,

    /// The URL to the user's Steam Community profile.
    #[serde(rename = "profileurl")]
    pub profile_url: String,

    /// The URL to the user's avatar as a 32x32 image
    pub avatar: String,

    /// The URL to the user's avatar as a 64x64 image
    #[serde(rename = "avatarmedium")]
    pub avatar_medium: String,

    /// The URL to the user's avatar as a 184x184 image
    #[serde(rename = "avatarfull")]
    pub avatar_full: String,

    /// Hash of the user's avatar
    #[serde(rename = "avatarhash")]
    pub avatar_hash: String,

    /// The user's status
    /// - 0: Offline (Also set when the profile is Private)
    /// - 1: Online
    /// - 2: Busy
    /// - 3: Away
    /// - 4: Snooze
    /// - 5: Looking to trade
    /// - 6: Looking to play
    #[serde(rename = "personastate")]
    pub persona_state: u8,

    /// An integer that describes the access setting of the profile.
    /// - 1: Private
    /// - 2: Friends only
    /// - 3: Friends of Friends
    /// - 4: Users Only
    /// - 5: Public
    #[serde(rename = "communityvisibilitystate")]
    pub community_visibility_state: u8,

    /// If set to 1, the user has configured the profile.
    #[serde(rename = "profilestate")]
    pub profile_state: Option<u8>,

    /// A unix timestamp of when the user was last online.
    #[serde(rename = "lastlogoff")]
    pub last_logoff: Option<u64>,

    /// If present the profile allows public comments.
    #[serde(rename = "commentpermission")]
    pub comment_permission: Option<u8>,

    /// The user's real name.
    #[serde(rename = "realname")]
    pub real_name: Option<String>,

    /// The 64-bit ID of the user's primary group.
    #[serde(rename = "primaryclanid")]
    pub primary_clan_id: Option<String>,

    /// A unix timestamp of the date the profile was created.
    #[serde(rename = "timecreated")]
    pub time_created: Option<u64>,

    /// If the user is in game this will be set to it's app ID as a string.
    #[serde(rename = "gameid")]
    pub game_id: Option<String>,

    /// The server URL given as an IP address and port number separated by a colon.
    /// This will not be present or set to "0.0.0.0:0" if none is available.
    #[serde(rename = "gameserverip")]
    pub game_server_ip: Option<String>,

    /// The title of the game that the user is playing.
    #[serde(rename = "gameextrainfo")]
    pub game_extra_info: Option<String>,

    /// ISO 3166 code of where the user is located.
    #[serde(rename = "loccountrycode")]
    pub loc_country_code: Option<String>,

    /// Variable length code representing the state the user is located in.
    #[serde(rename = "locstatecode")]
    pub loc_state_code: Option<String>,

    /// An integer ID internal to Steam representing the user's city.
    #[serde(rename = "loccityid")]
    pub loc_city_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerSummary {
    /// A list of profile objects. Contained information varies depending on
    /// whether or not the user has their profile set to Friends only or Private.
    players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    response: PlayerSummary,
}

// TODO: This requires a macro rework
impl Steam {
    /// Get user profile data.
    ///
    /// # Arguments
    ///
    /// * `steam_ids` - A vector of `SteamId` objects
    pub async fn get_player_summaries(
        &self,
        steam_ids: Vec<SteamId>,
    ) -> Result<Vec<Player>, SteamUserError> {
        let steam_ids: String = steam_ids.iter().map(|&id| id.to_string() + ",").collect();

        let query = format!("?key={}&steamids={}", &self.api_key, steam_ids);
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);

        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetPlayerSummaries);
        let wrapper: Wrapper = ErrorHandle!(
            from_value(json.to_owned()),
            SteamUserError::GetPlayerSummaries
        );

        Ok(wrapper.response.players)
    }
}
