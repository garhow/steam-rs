use serde::{Deserialize, Serialize};

use crate::{
    Steam,
    BASE,
    macros::do_http,
    errors::{ErrorHandle, SteamAppsError}
};

use super::INTERFACE;

const ENDPOINT: &str = "GetAppList";
const VERSION: &str = "2";

#[derive(Deserialize, Serialize, Debug)]
pub struct App {
    appid: u32,
    name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppList {
    apps: Vec<App>
}

#[derive(Deserialize, Serialize, Debug)]
struct Wrapper {
    applist: AppList
}

impl Steam {
    /// Gets the complete list of public apps.
    pub async fn get_app_list() -> Result<AppList, SteamAppsError> {
        let url = format!("{}/{}/{}/v{}/", BASE, INTERFACE, ENDPOINT, VERSION);
        let wrapper = do_http!(url, Wrapper, ErrorHandle, SteamAppsError::GetAppList);
        Ok(wrapper.applist)
    }
}