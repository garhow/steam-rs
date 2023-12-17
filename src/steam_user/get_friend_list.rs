//! # Implements the `GetFriendList` endpoint

use core::fmt;

use serde::Deserialize;
use serde_json::{from_value, Value};

use crate::{
    errors::{ErrorHandle, SteamUserError},
    macros::{do_http, optional_argument},
    steam_id::{de_steamid_from_str, SteamId},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetFriendList";
const VERSION: &str = "1";

/// Represents the types of relationships a user can have with their friends on Steam.
#[derive(PartialEq, Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Relationship {
    /// All relationships.
    All,
    /// Friends relationship.
    Friend,
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Relationship::All => write!(f, "all"),
            Relationship::Friend => write!(f, "friends"),
        }
    }
}

/// Represents a friend of a Steam user.
#[derive(Deserialize, Debug, Clone)]
pub struct Friend {
    /// The 64 bit ID of the friend.
    #[serde(rename = "steamid")]
    #[serde(deserialize_with = "de_steamid_from_str")]
    pub steam_id: SteamId,

    /// Role in relation to the given SteamID.
    pub relationship: Relationship,

    /// A unix timestamp of when the friend was added to the list.
    pub friend_since: u32,
}

// Represents the user's friend list.
//
// **Note:** If the profile is not public or there are no available entries for the given relationship only an empty object will be returned.
#[derive(Deserialize, Debug)]
struct FriendsList {
    /// A vector of Friend objects.
    friends: Vec<Friend>,
}

#[derive(Deserialize, Debug)]
struct Wrapper {
    /// If the profile is not public or there are no available entries for the given relationship only an empty object will be returned.
    #[serde(rename = "friendslist")]
    friends_list: Option<FriendsList>,
}

impl Steam {
    /// Get a user's friend list.
    ///
    /// # Arguments
    ///
    /// * `steam_id` - The SteamID of the user.
    /// * `relationship` - Optional relationship type (e.g., `Relationship::Friend`).
    ///
    /// # Example
    ///
    /// ```
    ///     // Creates new `Steam` instance using the environment variable `STEAM_API_KEY`.
    ///     let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    ///
    ///     // Retrieves friend list of user `76561197960435530`.
    ///     let friend_list = steam.get_friend_list(SteamId(76561197960435530), None).await.unwrap();
    ///
    ///     // Prints their first friend's SteamID
    ///     println!("{:?}", friend_list[0].steamid);
    /// ```
    pub async fn get_friend_list(
        &self,
        steam_id: SteamId,                  // SteamID of user
        relationship: Option<Relationship>, // relationship type (ex: Relationship::Friend)
    ) -> Result<Vec<Friend>, SteamUserError> {
        let query = format!(
            "?key={}&steamid={}{}",
            &self.api_key,
            steam_id,
            optional_argument!(relationship)
        );
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetFriendList);
        let wrapper: Wrapper =
            ErrorHandle!(from_value(json.to_owned()), SteamUserError::GetFriendList);

        Ok(wrapper.friends_list.unwrap().friends)
    }
}
