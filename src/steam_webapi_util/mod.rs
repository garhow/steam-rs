//! # Implements the `ISteamWebAPIUtil` interface
//!
//! Provides miscellaneous Web API related functionality through utility methods.
//!
//! For more info on how to use the Steamworks Web API please see the [Web API Overview](https://partner.steamgames.com/doc/webapi_overview).

const INTERFACE: &str = "ISteamWebAPIUtil";

pub mod get_server_info;
pub mod get_supported_api_list;
