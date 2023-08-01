use core::fmt;

use serde::Deserialize;
use serde_json::{Value, from_value};

use crate::{
    BASE,
    Steam,
    macros::{do_http, optional_argument},
    steam_id::{de_steamid_from_str, SteamId},
    errors::{SteamUserError, ErrorHandle},
};

use super::INTERFACE;

const ENDPOINT: &str = "GetFriendList";
const VERSION: &str = "1";

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Relationship {
    All,
    Friend
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Relationship::All => write!(f, "all"),
            Relationship::Friend => write!(f, "friends"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Friend {
    #[serde(rename = "steamid")]
    #[serde(deserialize_with = "de_steamid_from_str")]
    pub steam_id: SteamId, // The 64 bit ID of the friend.
    pub relationship: Relationship, // Role in relation to the given SteamID
    pub friend_since: u32 // A unix timestamp of when the friend was added to the list.
}

#[derive(Deserialize, Debug)]
pub struct FriendsList {
    pub friends: Vec<Friend> // A list of objects for each list entry. 
}

#[derive(Deserialize, Debug)]
struct Wrapper {
    #[serde(rename = "friendslist")]
    friends_list: Option<FriendsList> // If the profile is not public or there are no available entries for the given relationship only an empty object will be returned. 
}

impl Steam {
    pub async fn get_friend_list(
        &self,
        steam_id: SteamId,
        relationship: Option<Relationship>
    ) -> Result<FriendsList, SteamUserError> {
        let query = format!("?key={}&steamid={}{}", &self.api_key, steam_id, optional_argument!(relationship));
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetFriendList);
        let wrapper: Wrapper = ErrorHandle!(from_value(json.to_owned()), SteamUserError::GetFriendList);

        return Ok(wrapper.friends_list.unwrap());
    }
}