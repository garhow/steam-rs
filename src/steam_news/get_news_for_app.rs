//! Implements the `GetNewsForApp` endpoint

use crate::{
    errors::{ErrorHandle, SteamNewsError},
    macros::{do_http, optional_argument},
    Steam, BASE,
};
use serde::{Deserialize, Serialize};

use super::INTERFACE;

const ENDPOINT: &str = "GetNewsForApp";
const VERSION: &str = "0002";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewsItem {
    pub gid: String,
    pub title: String,
    pub url: String,
    pub is_external_url: bool,
    pub author: String,
    pub contents: String,
    pub feedlabel: String,
    pub date: u32,
    pub feedname: String,
    pub feed_type: u8,
    pub appid: u32,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppNews {
    pub appid: u32, // TODO: Convert to AppId
    pub newsitems: Vec<NewsItem>,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    appnews: AppNews,
}

impl Steam {
    /// Get the news for the specified app.
    ///
    /// # Arguments
    ///
    /// * `appid` - The ID of the application (game) for which to retrieve news for.
    /// * `max_length` - Maximum length for the content to return, if this is 0 the full content is returned, if it's less then a blurb is generated to fit.
    /// * `end_date` - Retrieve posts earlier than this date (unix epoch timestamp).
    /// * `count` - Number of posts to retrieve (default 20).
    /// * `feeds` - Comma-seperated list of feed names to return news for.
    pub async fn get_news_for_app(
        appid: u32,
        max_length: Option<u32>,
        end_date: Option<u32>,
        count: Option<u32>,
        feeds: Option<Vec<&str>>,
    ) -> Result<AppNews, SteamNewsError> {
        // DO NOT RAYON THIS! - Rayon doesn't protect the order of data!
        let feeds: Option<String> =
            feeds.map(|feeds| feeds.iter().map(|&feed| feed.to_string() + ",").collect());

        let optional_arguments = vec![
            optional_argument!(max_length, "maxlength"),
            optional_argument!(end_date, "enddate"),
            optional_argument!(count),
            optional_argument!(feeds),
        ];

        let query = format!("?appid={}{}", appid, optional_arguments.concat());
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);

        let response = do_http!(url, Response, ErrorHandle, SteamNewsError::GetNews);

        Ok(response.appnews)
    }
}
