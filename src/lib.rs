//! # steam-rs: Safe bindings for the Steam Web API
//!
//! The `steam-rs` crate provides convenient Rust bindings for the Steam Web API.
//! This crate provides safe and convenient Rust bindings for the Steam Web API.
//! 
//! ### Warning!
//! This crate is currently a work in progress, so please expect breaking changes and instability. Please be careful when using this! **This is not production ready!**

pub mod player_service;
pub mod published_file_service;
pub mod site_license_service;
pub mod steam_apps;
pub mod steam_id;
pub mod steam_news;
pub mod steam_user;
pub mod steam_user_stats;
pub mod steam_webapi_util;

/// IEconService Interface
/// Do Not Use - Not fully implemented!
pub mod econ_service;

pub mod steam_economy;

mod errors; // This remains private - maybe
mod macros; // This remains private

pub const BASE: &str = "https://api.steampowered.com";

pub struct Steam {
    api_key: String,
}

impl Steam {
    pub fn new(api_key: &str) -> Steam {
        Steam { api_key: api_key.to_string() }
    }
}