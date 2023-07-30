//! # steam-rs
//!
//! The `steam-rs` crate provides convenient Rust bindings for the Steam Web API.

use std::fmt;

use serde::{Deserialize, Serialize};

pub mod player_service;
pub mod steam_news;
pub mod steam_user;
pub mod steam_user_stats;

/// IEconService Interface
/// Do Not Use - Not fully implemented!
pub mod econ_service;

pub mod steam_economy;

mod errors; // This remains private - maybe
mod macros; // This remains private

pub const BASE: &str = "https://api.steampowered.com";

/// Struct for Steam IDs
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SteamId(pub u64);
impl fmt::Display for SteamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SteamId {
    fn from(value: String) -> Self {
        SteamId(value.parse::<u64>().unwrap())
    }
}

fn de_steamid_from_str<'de, D>(deserializer: D) -> Result<SteamId, D::Error>
    where D: serde::Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    Ok(SteamId::from(s))
}

/// Struct for Steam App IDs
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AppId(pub u32);
impl fmt::Display for AppId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

pub struct Steam {
    api_key: String,
}

impl Steam {
    pub fn new(api_key: &str) -> Steam {
        Steam { api_key: api_key.to_string() }
    }
}