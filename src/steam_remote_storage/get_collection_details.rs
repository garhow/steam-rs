use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{macros::{gen_args, do_http, optional_argument}, errors::{SteamEconomyError, ErrorHandle}, Steam};

const END_POINT: &str = "https://api.steampowered.com/ISteamRemoteStorage/GetCollectionDetails/v1";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetPrices {
    Result {
        success: bool,
        assets: Vec<Asset>,
        tags: HashMap<String, String>,
        tag_ids: HashMap<u64, u64>,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Asset {
    pub prices: HashMap<String, u64>,
    pub name: String,
    pub date: String,
    pub class: Vec<HashMap<String, String>>,
    pub classid: String,
    pub tags: Vec<String>,
    pub tag_ids: Vec<u64>,
}

impl Steam {
    /// This end point gives a 1:1 output of the api, if you were to put the endpoint into a browser, expect the same layout
    /// ( carbon-copy )
    pub async fn get_collection_details(&self, publishedfileids: &[u64]) -> Result<String, SteamEconomyError> {
        let key = &self.api_key.clone();

        let collectioncount = publishedfileids.len();

        let mut arg_buffer = String::new();
        for field in publishedfileids { arg_buffer.push_str(&format!("{field},")); }
        if arg_buffer != "" {arg_buffer.remove(arg_buffer.len() - 1);}

        let args = gen_args!(key, arg_buffer, collectioncount);

        let body = json!({}).to_string();

        let url = format!("{END_POINT}?{args}");
        Ok(do_http!(url, AssetPrices, ErrorHandle, SteamEconomyError::GetAssetPrices, body, true))
    }
}
