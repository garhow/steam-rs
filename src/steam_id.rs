use core::fmt;

use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};

use crate::Steam;

/// Represents a SteamID64 type which is used to uniquely identify users on the Steam platform.
/// SteamID64 is a 64-bit unsigned integer.
/// For more information on SteamID and SteamID64, refer to the official Steam documentation:
/// <https://developer.valvesoftware.com/wiki/SteamID>
///
/// # Examples
///
/// ```
/// use steam-rs::steam_id::SteamId;
///
/// let steam_id = SteamId::new(76561197960287930);
/// println!("SteamId: {}", steam_id);
/// ```
///
/// # Conversions
///
/// You can convert a `SteamId` to a `u64` using the `into()` method:
/// ```
/// use steam-rs::steam_id::SteamId;
///
/// let steam_id: SteamId = 76561197960287930.into();
/// println!("SteamId as u64: {}", steam_id.into());
/// ```
///
/// # Formatting
///
/// The `SteamId` can be formatted as a string for display purposes:
/// ```
/// use steam-rs::steam_id::SteamId;
///
/// let steam_id = SteamId::new(76561197960287930);
/// println!("Formatted SteamId: {}", steam_id.to_string());
/// ```
///
/// # Parsing
///
/// You can parse a `SteamId` from a string representation:
/// ```
/// use steam-rs::steam_id::SteamId;
///
/// let steam_id_str = "76561197960287930";
/// let steam_id = SteamId::from_str(steam_id_str).unwrap();
/// println!("Parsed SteamId: {}", steam_id);
/// ```
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SteamId(pub u64);
impl fmt::Display for SteamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SteamId {
    /// Creates a new `SteamId` instance with the given 64-bit unsigned integer value.
    pub fn new(value: u64) -> Self {
        SteamId(value)
    }

    /// Parses a `SteamId` from a string representation.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the parsing fails.
    pub fn from_str(s: &str) -> Result<Self, ParseSteamIdError> {
        match s.parse::<u64>() {
            Ok(value) => Ok(SteamId(value)),
            Err(_) => Err(ParseSteamIdError),
        }
    }

    /// Converts the `SteamId` into its underlying 64-bit unsigned integer value.
    pub fn into_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for SteamId {
    /// Converts a `u64` into a `SteamId`.
    fn from(value: u64) -> Self {
        SteamId(value)
    }
}

impl From<String> for SteamId {
    /// Formats the `SteamId` as a `String`.
    fn from(value: String) -> Self {
        SteamId(value.parse::<u64>().unwrap())
    }
}

/// Deserializes the `SteamId` from a `String`
pub fn de_steamid_from_str<'de, D>(deserializer: D) -> Result<SteamId, D::Error>
    where D: serde::Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    Ok(SteamId::from(s))
}

/// Error type for parsing a `SteamId` from a `string`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseSteamIdError;

impl std::fmt::Display for ParseSteamIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse SteamID")
    }
}

impl std::error::Error for ParseSteamIdError {}