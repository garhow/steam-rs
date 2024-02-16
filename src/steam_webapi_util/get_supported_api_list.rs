//! # Implements the `GetSupportedAPIList` endpoint

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, SteamWebAPIUtilError},
    macros::do_http,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetSupportedAPIList";
const VERSION: &str = "1";

/// Represents a parameter for an WebAPI method.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Parameter {
    /// Name of the parameter.
    pub name: String,

    /// Expected type of value.
    pub r#type: String,

    /// If input is optional for the method, then true.
    pub optional: bool,

    /// API documentation of parameter.
    pub description: Option<String>,
}

/// Represents a method within an API interface.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Method {
    /// Name of the method.
    pub name: String,

    /// Version of the method.
    pub version: u8,

    /// Allowed HTTP method for method (GET, POST).
    #[serde(rename = "httpmethod")]
    pub http_method: String,

    /// Parameters for the method.
    pub parameters: Option<Vec<Parameter>>,
}

/// Represents an API interface
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Interface {
    /// Name of the interface.
    pub name: String,

    /// Methods within the interface.
    pub methods: Vec<Method>,
}

/// List of WebAPI interfaces
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct APIList {
    /// Vector of API interfaces
    pub interfaces: Vec<Interface>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    apilist: APIList,
}

impl Steam {
    /// Lists all supported API calls.
    pub async fn get_supported_api_list(&self) -> Result<APIList, SteamWebAPIUtilError> {
        let url = format!(
            "{}/{}/{}/v{}/?key={}",
            BASE, INTERFACE, ENDPOINT, VERSION, &self.api_key
        );
        let json = do_http!(url, Value, ErrorHandle, SteamWebAPIUtilError::GetServerInfo);
        let wrapper: Wrapper = ErrorHandle!(
            serde_json::from_value(json.to_owned()),
            SteamWebAPIUtilError::GetServerInfo
        );
        Ok(wrapper.apilist)
    }
}
