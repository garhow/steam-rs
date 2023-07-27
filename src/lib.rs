//! # steam-rs
//!
//! The `steam-rs` crate provides convenient Rust bindings for the Steam Web API.

use std::fmt;

use serde_derive::{Deserialize, Serialize};

pub mod steam_news;
pub mod steam_user;
pub mod steam_user_stats;

/// Struct for Steam IDs
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SteamId(pub u64);
impl fmt::Display for SteamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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