use core::fmt;

use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};

/// Struct for Steam IDs
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SteamId(pub u64);
impl fmt::Display for SteamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SteamId {
    fn from(value: String) -> Self {
        SteamId(value.parse::<u64>().unwrap())
    }
}

pub fn de_steamid_from_str<'de, D>(deserializer: D) -> Result<SteamId, D::Error>
    where D: serde::Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    Ok(SteamId::from(s))
}