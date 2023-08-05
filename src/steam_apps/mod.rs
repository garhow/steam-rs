//! # Implements the `ISteamApps` interface
//!
//! Used to access data about applications on Steam.
//! 
//! **Note:** This implementation is incomplete!
//! The following endpoints are currently unimplemented:
//!
//! - GetAppBetas (requires publisher key)
//! - GetAppBuilds (requires publisher key)
//! - GetAppDepotVersions (requires publisher key)
//! - GetCheatingReports (requires publisher key)
//! - GetPartnerAppListForWebAPIKey (requires publisher key)
//! - GetPlayersBanned (requires publisher key)
//! - GetSDRConfig
//! - GetServerList (requires publisher key)
//! - SetAppBuildLive (requires publisher key)
//!
//! Endpoints that require a publisher key are not likely to be
//! implemented in the near future, as they cannot be tested by developers.

pub mod get_app_list;
pub mod get_servers_at_address;
pub mod up_to_date_check;

const INTERFACE: &str = "ISteamApps";
