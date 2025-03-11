//! # Implements the `GetUserGroupList` endpoint

use serde::Deserialize;
use serde_json::{from_value, Value};

use crate::{
    errors::{ErrorHandle, SteamUserError},
    macros::{do_http, EndPoint},
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetUserGroupList";
const VERSION: &str = "1";

#[derive(Deserialize, Debug)]
struct Wrapper {
    response: UserGroupsResponse,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserGroupsResponse {
    /// Boolean indicating if request was successful.
    pub success: bool,

    /// Error message that is given only if request was unsuccessful
    pub error: Option<String>,

    /// A vector of `Group` objects, representing the groups that the user is a member of
    pub groups: Vec<Group>,
}

/// Represents a Steam group
#[derive(Deserialize, Debug, Clone)]
pub struct Group {
    /// The group's ID
    pub gid: String,
}

EndPoint!(
    get_user_group_list,
    GetUserGroupListReq,
    format!("{}/{}/{}/v{}/", BASE, INTERFACE, ENDPOINT, VERSION),
    UserGroupsResponse,
    ( steam_id: SteamId ),
    [ ],
    async fn internal(url: String) -> Result<UserGroupsResponse, SteamUserError> {
        let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetUserGroupList);
        let wrapper: Wrapper = ErrorHandle!(
            from_value(json.to_owned()),
            SteamUserError::GetUserGroupList
        );

        Ok(wrapper.response)
    }
);

// impl Steam {
//     /// Get a list of groups that a user is a member of.
//     ///
//     /// # Arguments
//     ///
//     /// * `steam_id` - The SteamID of the user.
//     pub async fn get_user_group_list(&self, steam_id: SteamId) -> Result<Response, SteamUserError> {
//         let query = format!("?key={}&steamid={}", &self.api_key, steam_id);
//         let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
//         println!("{}", url);
        // let json = do_http!(url, Value, ErrorHandle, SteamUserError::GetUserGroupList);
        // let wrapper: Wrapper = ErrorHandle!(
        //     from_value(json.to_owned()),
        //     SteamUserError::GetUserGroupList
        // );

        // Ok(wrapper.response)
//     }
// }
