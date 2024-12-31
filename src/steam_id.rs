use core::fmt;

use num_enum::TryFromPrimitive;
use serde::{Deserialize, Deserializer, Serialize};

/// Represents a SteamID64 type which is used to uniquely identify users on the Steam platform.
/// SteamID64 is a 64-bit unsigned integer.
/// For more information on SteamID and SteamID64, refer to the official Steam documentation:
/// <https://developer.valvesoftware.com/wiki/SteamID>
///
/// # Examples
///
/// ```
/// use steam_rs::steam_id::SteamId;
///
/// let steam_id = SteamId::new(76561197960287930);
/// println!("SteamId: {}", steam_id);
/// ```
///
/// # Conversions
///
/// You can convert a `SteamId` to a `u64` using the `into()` method:
/// ```
/// use steam_rs::steam_id::SteamId;
///
/// let steam_id: SteamId = 76561197960287930.into();
/// println!("SteamId as u64: {}", steam_id);
/// ```
///
/// # Formatting
///
/// The `SteamId` can be formatted as a string for display purposes:
/// ```
/// use steam_rs::steam_id::SteamId;
///
/// let steam_id = SteamId::new(76561197960287930);
/// println!("Formatted SteamId: {}", steam_id.to_string());
/// ```
///
/// # Parsing
///
/// You can parse a `SteamId` from a string:
/// ```
/// use steam_rs::steam_id::SteamId;
///
/// let steam_id = SteamId::from(
///     "76561197960287930".to_string()
/// );
///
/// println!("Parsed SteamId: {}", steam_id);
/// ```
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
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

    /// Converts the `SteamId` into its underlying 64-bit unsigned integer value.
    pub fn into_u64(&self) -> u64 {
        self.0
    }

    /// Converts the `SteamId` into the unsigned 32-bit account ID used in its SteamID3 (and to some extent in the SteamID2).
    pub fn get_account_id(&self) -> u32 {
        (self.0 & 0xFFFFFFFF) as u32
    }

    /// Get Universe that the `SteamId` belongs to.
    pub fn get_universe(&self) -> Universe {
        Universe::try_from((self.0 >> 56) & 0xF).unwrap_or(Universe::Invalid)
    }

    /// Get account type of the `SteamId`.
    pub fn get_account_type(&self) -> AccountType {
        AccountType::try_from((self.0 >> 52) & 0xF).unwrap_or(AccountType::Invalid)
    }

    /// Get the `SteamId`'s SteamID2 string representation.
    pub fn to_id2_string(&self) -> String {
        let id = self.get_account_id();
        format!(
            "STEAM_{}:{}:{}",
            self.get_universe() as u64,
            id & 1,
            id >> 1
        )
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
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(SteamId::from(s))
}

fn callback<'de, D>(deserializer: D) -> Result<SteamId, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(SteamId::from(String::deserialize(deserializer)?))
}

#[derive(Debug, Deserialize)]
struct WrappedSteamId(#[serde(deserialize_with = "callback")] SteamId);

pub fn de_steamid_from_str_opt<'de, D>(deserializer: D) -> Result<Option<SteamId>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<WrappedSteamId>::deserialize(deserializer).map(
        |opt_wrapped: Option<WrappedSteamId>| opt_wrapped.map(|wrapped: WrappedSteamId| wrapped.0),
    )
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

/// Denotes what kind of account the SteamID belongs to.
#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u64)]
pub enum AccountType {
    /// Used for bots and accounts which do not belong to another class.
    Invalid,
    /// Single user account.
    Individual,
    /// Multiseat (e.g. cybercafe) account.
    Multiseat,
    /// Game server account.
    GameServer,
    /// Anonymous game server account.
    AnonGameServer,
    /// Sometimes used to temporarily refer to Individual accounts until their credentials are verified with Steam.
    Pending,
    ContentServer,
    Clan,
    Chat,
    /// Fake SteamID for local PSN account on PS3 or Live account on 360, etc.
    P2PSuperSeeder,
    AnonUser,
}

/// An "Universe" is an instance of Steam an account can belong to. "Public" is probably the one you'll be interacting with.
#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u64)]
pub enum Universe {
    Invalid,
    Public,
    Beta,
    Internal,
    Dev,
}
