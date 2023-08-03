//! # Implements the `GetUserGroupList` endpoint

use serde::Deserialize;
use serde_json::{Value, from_value};

use crate::{
    BASE,
    Steam,
    macros::do_http,
    steam_id::SteamId,
    errors::{SteamUserError, ErrorHandle},
};

use super::INTERFACE;

const ENDPOINT: &str = "GetUserGroupList";
const VERSION: &str = "1";

#[derive(Deserialize, Debug)]
struct Wrapper {
    response: Response
}

#[derive(Deserialize, Debug)]
pub struct Response {
    /// Boolean indicating if request was successful.
    pub success: bool,

    /// Error message that is given only if request was unsuccessful
    pub error: Option<String>,

    /// A vector of `Group` objects, representing the groups that the user is a member of
    pub groups: Vec<Group>,
}

/// Represents a Steam group
#[derive(Deserialize, Debug)]
pub struct Group {
    /// The group's ID
    pub gid: String
}

impl Steam {
    /// Get a list of groups that a user is a member of.
    ///
    /// # Arguments
    ///
    /// * `steam_id` - The SteamID of the user.
    ///
    /// # Example
    /// 
    /// ```
    ///     // Creates new `Steam` instance using the environment variable `STEAM_API_KEY`.
    ///     let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    /// 
    ///     // Retrieves a list of groups that user `76561197960435530` is a member of.
    ///     let group_list = steam.get_user_group_list(SteamId(76561197960435530)).await.unwrap();
    /// 
    ///     // Prints the user's first group's ID
    ///     println!("{:?}", group_list.groups[0].gid);
    /// ```
    pub async fn get_user_group_list(
        &self,
        steam_id: SteamId,
    ) -> Result<Response, SteamUserError> {
        let query = format!("?key={}&steamid={}", &self.api_key, steam_id);
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        println!("{}", url);
        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetUserGroupList);
        let wrapper: Wrapper = ErrorHandle!(from_value(json.to_owned()), SteamUserError::GetUserGroupList);

        Ok(wrapper.response)
    }
}