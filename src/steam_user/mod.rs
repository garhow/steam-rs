//! # Implements the `ISteamUser` interface
//! 
//! **Note:** This implementation is incomplete!
//! The following endpoints are currently unimplemented:
//! 
//! - CheckAppOwnership (requires publisher key)
//! - GetAppPriceInfo (requires publisher key)
//! - GetDeletedSteamIDs (requires publisher key)
//! - GetPublisherAppOwnership (requires publisher key)
//! - GetPublisherAppOwnershipChanges (requires publisher key)
//! - GetUserGroupList
//! - GrantPackage (requires publisher key)
//! - RevokePackage (requires publisher key)
//! 
//! Endpoints that require a publisher key are not likely to be
//! implemented in the near future, as they cannot be tested by developers.

const INTERFACE: &str = "ISteamUser";

pub mod get_friend_list;
pub mod get_player_bans;
pub mod get_player_summaries;
pub mod resolve_vanity_url;