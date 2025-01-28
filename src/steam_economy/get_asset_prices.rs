//! Implements the `GetAssetPrices` endpoint

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    errors::{ErrorHandle, SteamEconomyError},
    macros::{do_http, gen_args, optional_argument},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetAssetPrices";
const VERSION: &str = "1";

/// Represents asset prices for a specific app.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct AssetPrices {
    /// Indicates whether the request to retrieve asset prices was successful.
    pub success: bool,

    /// A vector containing information about each asset.
    pub assets: Vec<Asset>,

    /// Optional tags associated with the assets.
    pub tags: Option<HashMap<String, String>>,

    /// Optional tag IDs associated with the assets.
    pub tag_ids: Option<HashMap<u64, u64>>,
}

/// Represents an asset.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Asset {
    /// Prices associated with the asset.
    ///
    /// This maps the price of the asset in different currencies (USD, GBP, EUR, etc.).
    pub prices: HashMap<String, u64>,

    /// The name of the asset.
    pub name: String,

    /// The date of the asset's creation.
    pub date: String,

    /// The classes associated with the asset.
    pub class: Vec<HashMap<String, String>>,

    /// The class ID of the asset.
    pub classid: String,

    /// Optional tags associated with the asset.
    pub tags: Option<Vec<String>>,

    /// Optional tag IDs associated with the asset.
    pub tag_ids: Option<Vec<u64>>,
}

#[derive(Deserialize, Debug)]
struct Wrapper {
    result: Option<AssetPrices>,
}

impl Steam {
    /// Retrieves asset prices for a specified app.
    ///
    /// # Arguments
    ///
    /// * `appid` - The ID of the application (game) for which to retrieve asset prices. Must be a Steam economy app.
    /// * `language` - An optional parameter specifying the user's local language.
    /// * `currency` - An optional parameter specifying the currency to filter for.
    pub async fn get_asset_prices(
        &self,
        appid: u32,
        language: Option<&str>,
        currency: Option<&str>,
    ) -> Result<AssetPrices, SteamEconomyError> {
        let key = &self.api_key.clone();
        let args = gen_args!(key, appid) + &optional_argument!(language, currency);
        let url = format!("{}/{}/{}/v{}/?{}", BASE, INTERFACE, ENDPOINT, VERSION, args);

        let json = do_http!(url, Value, ErrorHandle, SteamEconomyError::GetAssetPrices);
        let response: Wrapper = ErrorHandle!(
            from_value(json.to_owned()),
            SteamEconomyError::GetAssetPrices
        );

        Ok(response.result.unwrap())
    }
}
