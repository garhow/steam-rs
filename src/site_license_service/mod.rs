//! # Implements the `ISiteLicenseService` interface
//!
//! Provides access to services related to operating sites which are part of the Steam PC Cafe program.

pub mod get_current_client_connections;
pub mod get_total_playtime;

use serde::{Deserialize, Serialize};

const INTERFACE: &str = "ISiteLicenseService";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameName {
    /// ID number of entry.
    pub id: u32,

    /// Game name.
    pub name: String,
}
