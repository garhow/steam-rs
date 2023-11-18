use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, SteamEconomyError},
    macros::{do_http, gen_args, optional_argument},
    Steam,
};

const END_POINT: &str = "https://api.steampowered.com/ISteamEconomy/GetAssetPrices/v1/?";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetPrices {
    Result {
        success: bool,
        assets: Vec<Asset>,
        tags: HashMap<String, String>,
        tag_ids: HashMap<u64, u64>,
    },
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
    pub async fn get_asset_prices(
        &self,
        appid: u32,
        language: Option<&str>,
        currency: Option<&str>,
    ) -> Result<AssetPrices, SteamEconomyError> {
        let key = &self.api_key.clone();
        let args = gen_args!(key, appid) + &optional_argument!(language, currency);
        let url = format!("{END_POINT}{args}");
        Ok(do_http!(
            url,
            AssetPrices,
            ErrorHandle,
            SteamEconomyError::GetAssetPrices
        ))
    }
}
