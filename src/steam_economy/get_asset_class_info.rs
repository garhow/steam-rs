use std::collections::HashMap;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, SteamEconomyError},
    macros::{do_http, gen_args, optional_argument},
    Steam,
};

const END_POINT: &str = "https://api.steampowered.com/ISteamEconomy/GetAssetClassInfo/v1/?";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
struct UncleanAssetClassInfo {
    pub result: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetClassInfo {
    pub icon_url: String,
    pub icon_url_large: String,
    pub icon_drag_url: String,
    pub name: String,
    pub market_hash_name: String,
    pub market_name: String,
    pub name_color: String,
    pub background_color: String,
    pub r#type: String,
    pub tradable: String,
    pub marketable: String,
    pub market_tradeable_restriction: Option<String>,
    pub market_marketable_restriction: Option<String>,
    pub fraudwarnings: String,
    pub descriptions: HashMap<String, Description>,
    pub actions: HashMap<String, Action>,
    pub market_actions: HashMap<String, Action>,
    pub tags: HashMap<String, Tag>,
    pub app_data: AppData,
    pub class_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Description {
    pub r#type: String,
    pub value: String,
    pub app_data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Action {
    pub name: String,
    pub link: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppData {
    pub def_index: String,
    pub quality: String,
    pub slot: String,
    pub filter_data: HashMap<String, HashMap<String, Value>>,
    pub player_class_ids: HashMap<String, String>,
    pub highlight_color: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub internal_name: String,
    pub category: String,
    pub color: Option<String>,
    pub category_name: String,
}

impl Steam {
    /// This endpoint gets modified to provide easy access to the data - This will not give an exact copy of the data outputed by the API
    pub async fn get_asset_class_info(
        &self,
        appid: u32,
        language: Option<&str>,
        class_count: u32,
        classid0: u64,
        instanceid0: Option<u32>,
    ) -> Result<HashMap<String, AssetClassInfo>, SteamEconomyError> {
        let key = &self.api_key.clone();
        let args = gen_args!(key, appid, class_count, classid0)
            + &optional_argument!(language, instanceid0);
        let url = format!("{END_POINT}{args}");
        let response = do_http!(
            url,
            UncleanAssetClassInfo,
            ErrorHandle,
            SteamEconomyError::GetAssetClassInfo
        )
        .clean()?;
        Ok(response)
    }
}

impl UncleanAssetClassInfo {
    pub fn clean(mut self) -> Result<HashMap<String, AssetClassInfo>, SteamEconomyError> {
        // Discard success status
        self.result.remove("success");

        // TODO: make sure that order isn't needed
        let data = self
            .result
            .par_iter()
            .map(move |(id, data)| -> (String, AssetClassInfo) {
                println!("{data:?}");
                (
                    id.clone(),
                    serde_json::from_value::<AssetClassInfo>(data.to_owned()).unwrap(),
                )
            })
            .collect::<Vec<(String, AssetClassInfo)>>();

        // Convert the data into a hashmap
        let data = data
            .into_iter()
            .collect::<HashMap<String, AssetClassInfo>>();
        Ok(data)
    }
}
