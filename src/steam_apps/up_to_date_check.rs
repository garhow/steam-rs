use serde::{Deserialize, Serialize};

use crate::{
    Steam,
    BASE,
    macros::do_http,
    errors::{ErrorHandle, SteamAppsError}
};

use super::INTERFACE;

const ENDPOINT: &str = "UpToDateCheck";
const VERSION: &str = "1";

#[derive(Deserialize, Serialize, Debug)]
pub struct UpToDateResponse {
    pub success: bool,
    pub up_to_date: bool,
    pub version_is_listable: bool,
    pub required_version: Option<u32>,
    pub message: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Wrapper {
    response: UpToDateResponse
}

impl Steam {
    /// Checks if the app is up to date.
    pub async fn up_to_date_check(
        appid: u32,
        version: u32
    ) -> Result<UpToDateResponse, SteamAppsError> {
        let url = format!("{}/{}/{}/v{}/?appid={}&version={}", BASE, INTERFACE, ENDPOINT, VERSION, appid, version);
        let wrapper = do_http!(url, Wrapper, ErrorHandle, SteamAppsError::GetAppList);
        Ok(wrapper.response)
    }
}