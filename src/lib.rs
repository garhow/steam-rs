pub mod steam_news;
pub mod steam_user;
pub mod steam_user_stats;

pub struct Steam {
    api_key: String,
}

impl Steam {
    pub fn new(steam_api_key: String) -> Steam {
        Steam { api_key: steam_api_key }
    }
}