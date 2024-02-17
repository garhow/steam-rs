use serde::{Deserialize, Serialize};

use crate::{errors::{ErrorHandle, SteamRemoteStorageError}, macros::do_http, Steam, BASE};

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

        let mut params = String::new();

        params.push_str(
            &format!("collectioncount={}", published_fileids.len())
        );

        for (index, fileid) in published_fileids.iter().enumerate() {
            params.push_str(
                &format!("&publishedfileids[{}]={}", index, fileid)
            );
        }

        let wrapper = do_http!(
            url,
            Wrapper,
            ErrorHandle,
            SteamRemoteStorageError::GetCollectionDetails,
            params
        );

        Ok(wrapper.response)
    }
}
