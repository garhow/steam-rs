use serde_derive::{Serialize, Deserialize};
use crate::{Steam,
    app_id::AppId,
    errors::{SteamNewsError, ErrorHandle},
    macros::{do_http, optional_argument},
    BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetNewsForApp";
const VERSION: &str = "0002";

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsItem {
    gid: String,
    title: String,
    url: String,
    is_external_url: bool,
    author: String,
    contents: String,
    feedlabel: String,
    date: u32,
    feedname: String,
    feed_type: u8,
    appid: u32,
    tags: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppNews {
    appid: u32, // TODO: Convert to AppId
    newsitems: Vec<NewsItem>,
    count: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    appnews: AppNews
}

impl Steam {
    pub async fn get_news_for_app(
        app_id: AppId,
        max_length: Option<u32>,
        end_date: Option<u32>,
        count: Option<u32>,
        feeds: Option<Vec<&str>>
    ) -> Result<AppNews, SteamNewsError> {
        let feeds: Option<String> = if let Some(feeds) = feeds {
            Some(feeds.iter().map(|&feed| feed.to_string() + ",").collect())
        } else { None }; // DO NOT RAYON THIS! - Rayon doesn't protect the order of data!

        let optional_arguments = vec![
            optional_argument!(max_length, "maxlength"),
            optional_argument!(end_date, "enddate"),
            optional_argument!(count),
            optional_argument!(feeds),
        ];

        println!("{}", optional_arguments[3]);

        let query = format!("?appid={}{}", app_id, optional_arguments.concat());
        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);

        let response = do_http!(url, Response, ErrorHandle, SteamNewsError::GetNews);

        Ok(response.appnews)
    }
}
