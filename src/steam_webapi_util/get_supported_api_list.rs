use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Steam,
    errors::{SteamWebAPIUtilError, ErrorHandle},
    macros::do_http,
    BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetSupportedAPIList";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameter {
    pub name: String,
    pub r#type: String,
    pub optional: bool,
    pub description: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Method {
    pub name: String,
    pub version: u8,
    #[serde(rename = "httpmethod")]
    pub http_method: String,
    pub parameters: Option<Vec<Parameter>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Interface {
    pub name: String,
    pub methods: Vec<Method>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct APIList {
    pub interfaces: Vec<Interface>
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    apilist: APIList
}

impl Steam {
    pub async fn get_supported_api_list(&self) -> Result<APIList, SteamWebAPIUtilError> {
        let url = format!("{}/{}/{}/v{}/?key={}", BASE, INTERFACE, ENDPOINT, VERSION, &self.api_key);
        let json = do_http!(url, Value, ErrorHandle, SteamWebAPIUtilError::GetServerInfo);
        let wrapper: Wrapper = ErrorHandle!(serde_json::from_value(json.to_owned()), SteamWebAPIUtilError::GetServerInfo);
        Ok(wrapper.apilist)
    }
}
