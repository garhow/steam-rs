//! # Implements the `GetAppList` endpoint

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamAppsError},
    macros::do_http,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetAppList";
const VERSION: &str = "2";

/// Represents a Steam app.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct App {
    /// App ID of this application.
    pub appid: u32,

    /// The name of this application.
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppList {
    /// The vector containing the applications.
    pub apps: Vec<App>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Wrapper {
    applist: AppList,
}

impl Steam {
    /// Gets the complete list of public apps.
    ///
    /// # Example
    ///
    /// ```
    ///     // Retrieves app list.
    ///     let app_list = Steam::get_app_list().await.unwrap();
    ///
    ///     // Prints the name of the first app in the list.
    ///     println!("{}", app_list.apps[0].name);
    /// ```
    pub async fn get_app_list() -> Result<AppList, SteamAppsError> {
        let url = format!("{}/{}/{}/v{}/", BASE, INTERFACE, ENDPOINT, VERSION);
        let wrapper = do_http!(url, Wrapper, ErrorHandle, SteamAppsError::GetAppList);
        Ok(wrapper.applist)
    }
}
