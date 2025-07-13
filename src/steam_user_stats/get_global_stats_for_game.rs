//! Implements the `GetGlobalStatsForGame` endpoint
//! This is a manual implementation (no macros used) due to the quirks of the steam web api

use std::collections::HashMap;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, SteamUserStatsError}, gen_args, macros::{do_http, optional_argument, EndPoint}, Steam, BASE
};

use super::INTERFACE;

const ENDPOINT: &str = "GetGlobalStatsForGame";
const VERSION: &str = "1";

// This is the best thing I could come up with
#[derive(Deserialize, Serialize, Debug)]
pub struct GlobalStats {
    pub globalstats: HashMap<String, Stat>,
    pub result: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Stat {
    pub total: f64
}

impl Steam {
    #[doc(hidden)]
    #[deprecated(
        note = "This function is implemented but not tested yet."
    )]
    pub async fn get_global_stats_for_game(&self, appid: u32, count: u32, names: Vec<String>) -> Result<GlobalStats, SteamUserStatsError> {
        let url = format!("{}/{}/{}/v{}/", BASE, INTERFACE, ENDPOINT, VERSION);
        #[cfg(test)] println!("{}", url);

        let mut args = gen_args!( appid, count);
        #[cfg(test)] println!("{:?}", args);
        for (i, name) in names.iter().enumerate() {
            args.push_str(&format!("&name[{i}]={}", name));
        }
        #[cfg(test)] println!("{:?}", args);

        let url = format!("{url}?{args}");
        #[cfg(test)] println!("{}", url);
        // Most a carbon copy of the macro
        if let Ok(response) = reqwest::get(url).await {

            let status = response.status();

            match status {
                reqwest::StatusCode::OK => {
                    let txt = ErrorHandle!(response.text().await, SteamUserStatsError::GetGlobalStatsForGame);
                    let data = ErrorHandle!(serde_json::from_str::<Value>(&txt), SteamUserStatsError::GetGlobalStatsForGame);
                    if let Ok(data) = serde_json::from_value::<GlobalStats>(data.clone()) {
                        return Ok(data);
                    } else {
                        return Err(SteamUserStatsError::GetGlobalStatsForGame(format!("{}", data["response"]["error"].as_str().unwrap())));
                    }

                },
                _ => {
                    let data = response.text().await;
                    let remark;
                    if data.is_ok() {
                        remark = format!(", alongside data: '{}'", data.unwrap());
                    } else {
                        remark = String::new();
                    }

                    return Err(SteamUserStatsError::GetGlobalStatsForGame(format!(
                        "Expected 200 Status, got {}{}",
                        status, remark
                    )));
                }
            }
            
        } else {
            return Err(SteamUserStatsError::GetGlobalStatsForGame("HTTPS Error".to_string()));
        }

    }
}
