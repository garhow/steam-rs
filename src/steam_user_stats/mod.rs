//! # Implements the `ISteamUserStats` interface
//!
//! Provides various statistics about Steam users and applications.
//!
//! **Note:** This implementation is incomplete!
//! The following endpoints are currently unimplemented:
//!
//! - GetGlobalStatsForGame
//! - GetSchemaForGame

const INTERFACE: &str = "ISteamUserStats";

pub mod get_global_achievement_percentages_for_app;
pub mod get_number_of_current_players;
pub mod get_player_achievements;
pub mod get_user_stats_for_game;
