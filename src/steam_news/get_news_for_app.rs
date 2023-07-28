use serde_derive::{Serialize, Deserialize};
use anyhow::Result;
use crate::{Steam, AppId, errors::{SteamNewsError, ErrorHandle}, macros::do_http};

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
    appid: u32,
    newsitems: Vec<NewsItem>,
    count: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    appnews: AppNews
}

impl Steam {
    pub async fn get_news_for_app(&self, appid: AppId, count: u32, max_length: u32) -> Result<AppNews, SteamNewsError> {
        let url = format!("https://api.steampowered.com/ISteamNews/GetNewsForApp/v0002/?appid={}&count={}&maxlength={}",
            appid,
            count,
            max_length,
        );

        let response = do_http!(url, Response, ErrorHandle, SteamNewsError::GetNews);

        Ok(response.appnews)
    }
}
