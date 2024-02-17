//! # Implements the `IPlayerService` interface
//!
//! Provides additional methods for interacting with Steam Users.

const INTERFACE: &str = "IPlayerService";

pub mod get_badges;
pub mod get_community_badge_progress;
pub mod get_owned_games;
pub mod get_recently_played_games;
pub mod get_steam_level;
