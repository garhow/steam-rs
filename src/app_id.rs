use core::fmt;

use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};

/// Struct for Steam App IDs
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AppId(pub u32);
impl fmt::Display for AppId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}