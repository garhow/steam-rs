//! # Implements the `UpToDateCheck` endpoint

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamAppsError},
    macros::do_http,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "UpToDateCheck";
const VERSION: &str = "1";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpToDateResponse {
    /// Boolean indicating if request was successful.
    pub success: bool,

    /// Boolean indicating if the given version number is the most current version.
    pub up_to_date: bool,

    /// Boolean indicating if the given version can be listed in public changelogs. [\[debated\]](https://wiki.teamfortress.com/wiki/WebAPI/UpToDateCheck#cite_note-1)
    pub version_is_listable: bool,

    /// Integer of the most current version of the app available.
    pub required_version: Option<u32>,

    /// A string giving the status message if applicable.
    pub message: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Wrapper {
    response: UpToDateResponse,
}

impl Steam {
    /// Check if a given app version is the most current available.
    ///
    /// # Arguments
    ///
    /// * `appid` - AppID of game.
    /// * `version` - The installed version of the game.
    pub async fn up_to_date_check(
        appid: u32,
        version: u32,
    ) -> Result<UpToDateResponse, SteamAppsError> {
        let url = format!(
            "{}/{}/{}/v{}/?appid={}&version={}",
            BASE, INTERFACE, ENDPOINT, VERSION, appid, version
        );
        let wrapper = do_http!(url, Wrapper, ErrorHandle, SteamAppsError::GetAppList);
        Ok(wrapper.response)
    }
}
