//! Implements the `GetAssetClassInfo` endpoint

use std::collections::HashMap;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, SteamEconomyError},
    macros::{do_http, gen_args, optional_argument},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetAssetClassInfo";
const VERSION: &str = "1";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
struct UncleanAssetClassInfo {
    pub result: HashMap<String, Value>,
}

/// Represents information about an asset class.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetClassInfo {
    /// The URL of the asset class icon.
    pub icon_url: String,
    /// The URL of the large asset class icon.
    pub icon_url_large: String,
    /// The URL used for dragging the asset class icon.
    pub icon_drag_url: String,
    /// The name of the asset class.
    pub name: String,
    /// The market hash name of the asset class.
    pub market_hash_name: String,
    /// The market name of the asset class.
    pub market_name: String,
    /// The color of the name associated with the asset class.
    pub name_color: String,
    /// The background color associated with the asset class.
    pub background_color: String,
    /// The type of the asset class.
    pub r#type: String,
    /// Indicates if the asset class is tradable.
    pub tradable: String,
    /// Indicates if the asset class is marketable.
    pub marketable: String,
    /// Optional market tradeable restriction of the asset class.
    pub market_tradeable_restriction: Option<String>,
    /// Optional market marketable restriction of the asset class.
    pub market_marketable_restriction: Option<String>,
    /// Fraud warnings associated with the asset class.
    pub fraudwarnings: String,
    /// Descriptions associated with the asset class.
    pub descriptions: HashMap<String, Description>,
    /// Actions associated with the asset class.
    pub actions: HashMap<String, Action>,
    /// Market actions associated with the asset class.
    pub market_actions: HashMap<String, Action>,
    /// Tags associated with the asset class.
    pub tags: HashMap<String, Tag>,
    /// Application data associated with the asset class.
    pub app_data: AppData,
    /// Optional class ID of the asset class.
    pub class_id: Option<String>,
}

/// Represents a description associated with an asset class.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Description {
    /// The type of the description.
    pub r#type: String,
    /// The value of the description.
    pub value: String,
    /// Application data associated with the description.
    pub app_data: String,
}

/// Represents an action associated with an asset class.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Action {
    /// The name of the action.
    pub name: String,
    /// The link associated with the action.
    pub link: String,
}

/// Represents application data associated with an asset class.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppData {
    pub def_index: String,
    pub quality: String,
    pub slot: String,
    pub filter_data: HashMap<String, HashMap<String, Value>>,
    pub player_class_ids: HashMap<String, String>,
    pub highlight_color: String,
}

/// Represents a tag associated with an asset class.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    /// The name of the tag.
    pub name: String,
    /// The internal name of the tag.
    pub internal_name: String,
    /// The category of the tag.
    pub category: String,
    /// The color of the tag, if available.
    pub color: Option<String>,
    /// The name of the category associated with the tag.
    pub category_name: String,
}

impl Steam {
    /// Retrieves asset class information for a specified app.
    ///
    /// # Arguments
    ///
    /// * `appid` - The ID of the application (game) for which to retrieve asset class information. Must be a steam economy app.
    /// * `language` - An optional parameter specifying the user's local language.
    /// * `class_count` - Number of classes requested. Must be at least one.
    /// * `classid0` - Class ID of the nth class.
    /// * `instanceid0` - Instance ID of the nth class.
    ///
    /// Note: This endpoint gets modified to provide easier access to the data! This will not give an exact copy of the data outputed by the API.
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
        let url = format!("{}/{}/{}/v{}/?{}", BASE, INTERFACE, ENDPOINT, VERSION, args);

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
