use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{errors::SteamRemoteStorageError, Steam, BASE};

use super::INTERFACE;

const ENDPOINT: &str = "GetCollectionDetails";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CollectionDetails {
    #[serde(rename = "publishedfileid")]
    published_file_id: String,
    result: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response {
    pub result: u32,
    #[serde(rename = "resultcount")]
    pub result_count: u32,
    #[serde(rename = "collectiondetails")]
    pub collection_details: Vec<CollectionDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: Response,
}

impl Steam {
    pub async fn get_collection_details(
        published_fileids: &[u64],
    ) -> Result<Response, SteamRemoteStorageError> {
        let url = format!("{BASE}/{INTERFACE}/{ENDPOINT}/v{VERSION}");

        let mut params = HashMap::new();
        params.insert(
            "collectioncount".to_string(),
            published_fileids.len().to_string(),
        );

        for (index, fileid) in published_fileids.iter().enumerate() {
            params.insert(format!("publishedfileids[{index}]"), fileid.to_string());
        }

        let client = reqwest::Client::new();

        let request = client.post(url).form(&params).send().await.unwrap();

        let json: Value = request.json().await.unwrap();

        let wrapper: Wrapper = from_value(json).unwrap();

        Ok(wrapper.response)
    }
}
