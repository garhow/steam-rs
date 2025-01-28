//! # Implements the `IGameServersService` interface
//!
//! Provides addtional methods for administration of Steam Game Servers
//! It's worth noting that steam ids aren't just for user accounts, servers also have steam ids
//! These aren't well differentiated within the docs for the steam api
//!
//! **Note:** This implementation is incomplete! The following endpoints are currently unimplemented
//!
//! - CreateAccount
//! - SetMemo
//! - ResetLoginToken
//! - DeleteAccount
//! - GetServerSteamIDsByIP
//! - GetServerIPsBySteamID

const INTERFACE: &str = "IGameServersService";

pub mod get_account_list;
pub mod get_account_public_info;
pub mod query_login_token;
