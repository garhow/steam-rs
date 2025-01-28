//! # steam-rs
//!
//! Safe and convenient Rust bindings for the Steam Web API.
//!
//! **Warning**: This crate is still a work in progress. Breaking changes and instability are to be expected. Use with caution—**this is not production-ready**.
//!
//! The core of this crate is the [`Steam`] struct, which interacts with the Steam Web API. It typically[^1] needs to be initialized with a valid Steam API key.
//!
//! ```
//! // Retrieve the Steam API key from an environment variable.
//! let steam_api_key = &std::env::var("STEAM_API_KEY").expect("Missing an API key");
//!
//! // Initialize the Steam API client.
//! let steam = Steam::new(steam_api_key);
//! ```
//!
//! Another key component of this crate is the [`SteamId`](`steam_id::SteamId`) struct. It represents a Steam user ID[^2], which is often used when querying user data.
//!
//! ```
//! let steam_ids = vec![
//!     SteamId::new(76561198136162943),
//!     SteamId(76561197960435530),
//! ];
//!
//! // Request the player summaries of SteamIDs `76561198136162943` and `76561197960435530`.
//! let player_summaries = steam.get_player_summaries(steam_ids).await.unwrap();
//! ```
//!
//! [^1]: Not all API endpoints require an API key, and in that case providing one is optional.
//! [^2]: Specifically, [`SteamId`](`steam_id::SteamId`) represents a SteamID64 type, but more types, such as SteamID and SteamID3 are planned in future releases.

pub mod econ_service;
pub mod game_servers_service;
pub mod player_service;
pub mod published_file_service;
pub mod site_license_service;
pub mod steam_apps;
pub mod steam_economy;
pub mod steam_id;
pub mod steam_news;
pub mod steam_remote_storage;
pub mod steam_user;
pub mod steam_user_auth;
pub mod steam_user_stats;
pub mod steam_webapi_util;

pub mod errors;
mod macros; // This remains private

const BASE: &str = "https://api.steampowered.com";

pub struct Steam {
    api_key: String,
}

impl Steam {
    pub fn new(api_key: &str) -> Steam {
        Steam {
            api_key: api_key.to_string(),
        }
    }
}
