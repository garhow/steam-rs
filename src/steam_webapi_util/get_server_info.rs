use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Steam,
    errors::{SteamWebAPIUtilError, ErrorHandle},
    macros::do_http,
    BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetServerInfo";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerInfo {
    #[serde(rename = "servertime")]
    server_time: u32,
    #[serde(rename = "servertimestring")]
    server_time_string: String,
}

impl Steam {
    pub async fn get_server_info() -> Result<ServerInfo, SteamWebAPIUtilError> {
        let url = format!("{}/{}/{}/v{}/", BASE, INTERFACE, ENDPOINT, VERSION);
        let json = do_http!(url, Value, ErrorHandle, SteamWebAPIUtilError::GetServerInfo);
        let server_info: ServerInfo = ErrorHandle!(serde_json::from_value(json.to_owned()), SteamWebAPIUtilError::GetServerInfo);
        Ok(server_info)
    }
}
