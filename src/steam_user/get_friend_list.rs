use core::fmt;

use serde_derive::Deserialize;
use serde_json::Value;
use rayon::prelude::*;

use crate::{
    BASE,
    Steam,
    macros::{do_http, optional_argument},
    steam_id::{de_steamid_from_str, SteamId},
    errors::{SteamUserError, ErrorHandle},
};

use super::INTERFACE;

const ENDPOINT: &str = "GetFriendList";
const VERSION: &str = "0001";

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
    pub steam_id: SteamId,
    pub relationship: Relationship,
    pub friend_since: u32
}

impl Steam {
    pub async fn get_friend_list(
        &self,
        steam_id: SteamId,
        relationship: Option<Relationship>
    ) -> Result<Vec<Friend>, SteamUserError> {

        let query = format!("?key={}&steamid={}{}", &self.api_key, steam_id, optional_argument!(relationship));

        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        
        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetFriendList);

        let buffer = json["friendslist"]["friends"].as_array();
        if buffer == None { return Err(SteamUserError::GetFriendList("Invalid JSON".to_owned()) );}

        let friend_list: Vec<Friend> = buffer.unwrap().par_iter().map(|item | {
            println!("TODO: Handle invalid json data! (src/steam_user/get_friend_list.rs:35) ");
            serde_json::from_value(item.clone()).unwrap()
        }).collect();
            
        return Ok(friend_list);
    }
}