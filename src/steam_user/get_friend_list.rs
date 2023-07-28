use core::fmt;

use serde_derive::Deserialize;
use serde_json::Value;
use rayon::prelude::*;

use crate::{Steam, SteamId, macros::do_http, errors::{SteamUserError, ErrorHandle}};

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
    pub async fn get_friend_list(&self, steam_id: &str, relationship: Relationship) -> Result<Vec<Friend>, SteamUserError> {
        let url = format!("https://api.steampowered.com/ISteamUser/GetFriendList/v0001/?key={}&steamid={}&relationship={}",
            &self.api_key,
            steam_id,
            relationship
        );
        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetFriendList);

        let buffer = json["friendslist"]["friends"].as_array();
        if let None = buffer { return Err(SteamUserError::GetFriendList("Invalid JSON".to_owned()) );}

        let friend_list: Vec<Friend> = buffer.unwrap().par_iter().map(|item | {
            println!("TODO: Handle invalid json data! (src/steam_user/get_friend_list.rs:35) ");
            serde_json::from_value(item.clone()).unwrap()
        }).collect();
            
        return Ok(friend_list);
    }
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Relationship::All => write!(f,"all"),
            Relationship::Friend => write!(f,"friends"),
        }
    }
}