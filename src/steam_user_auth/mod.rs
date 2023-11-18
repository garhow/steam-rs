//! # Implements the `ISteamUserAuth` interface
//!
//! Used to authenticate users in your application.
//!
//! **Note:** This implementation is incomplete!
//! The following endpoints are currently unimplemented:
//!
//! - AuthenticateUser

const INTERFACE: &str = "ISteamUserAuth";

pub mod authenticate_user_ticket;
