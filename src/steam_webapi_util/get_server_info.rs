//! # Implements the `GetServerInfo` endpoint

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, SteamWebAPIUtilError},
    macros::{do_http, EndPoint},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetServerInfo";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerInfo {
    /// Returns Unix timestamp of WebAPI server time.
    #[serde(rename = "servertime")]
    pub server_time: u32,

    /// Returns time string of WebAPI server time.
    #[serde(rename = "servertimestring")]
    pub server_time_string: String,
}

// EndPoint!(
//     get_server_info,
//     ServerInfoReq,
//     format!("{}/{}/{}/v{}/", BASE, INTERFACE, ENDPOINT, VERSION),
//     ServerInfo,
//     ( ),
//     [ ],
//     async fn internal(url: String) -> Result<ServerInfo, SteamWebAPIUtilError> {
//         let json = do_http!(url, Value, ErrorHandle, SteamWebAPIUtilError::GetServerInfo);
//         let server_info: ServerInfo = ErrorHandle!(
//             serde_json::from_value(json.to_owned()),
//             SteamWebAPIUtilError::GetServerInfo
//         );
//         Ok(server_info)
//     }
// );

// This requires a macro change
impl Steam {
    /// Returns WebAPI server time & checks server status.
    pub async fn get_server_info() -> Result<ServerInfo, SteamWebAPIUtilError> {
        let url = format!("{}/{}/{}/v{}/", BASE, INTERFACE, ENDPOINT, VERSION);
        let json = do_http!(url, Value, ErrorHandle, SteamWebAPIUtilError::GetServerInfo);
        let server_info: ServerInfo = ErrorHandle!(
            serde_json::from_value(json.to_owned()),
            SteamWebAPIUtilError::GetServerInfo
        );
        Ok(server_info)
    }
}
