use core::fmt;
use std::sync::LazyLock;

use num_enum::TryFromPrimitive;
use pomsky_macro::pomsky;
use regex::{Match, Regex};
use serde::{Deserialize, Deserializer, Serialize};

const STEAM2_REGEX_STR: &str = pomsky! {
    ^
    "STEAM_"
    :universe(['0'-'4'])
    ":"
    :authServer(['0'-'1'])
    ":"
    :id('0' | ['1'-'9']['0'-'9']{0,9})
    $
};

static STEAM2_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(STEAM2_REGEX_STR).unwrap());

const STEAM3_REGEX_STR: &str = pomsky! {
    ^
    "["
    :type(["AGMPCgcLTIUai"])
    ":"
    :universe(['0'-'4'])
    ":"
    :id('0' | ['1'-'9']['0'-'9']{0,9})
    (":":instance(['0'-'9']+))?
    "]"
    $
};

static STEAM3_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(STEAM3_REGEX_STR).unwrap());

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
///
/// let smart_steam_id = SteamId::new_smart("STEAM_1:0:11101");
/// println!("SteamId: {}", smart_steam_id);
/// ```
///
/// # Conversions
///
/// You can convert a `u64` to a `SteamId` using the `into()` method:
/// ```
/// use steam_rs::steam_id::SteamId;
///
/// let steam_id: SteamId = 76561197960287930.into();
/// println!("SteamId from u64: {}", steam_id);
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SteamIdError {
    Invalid,
    TooBig(String, u64),
}

impl std::fmt::Display for SteamIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Invalid => "Provided SteamID is invalid.",
            Self::TooBig(what, limit) => &format!("{what} cannot be bigger than {limit}."),
        };

        write!(f, "{msg}")
    }
}

impl std::error::Error for SteamIdError {}

impl SteamId {
    /// Creates a new `SteamId` instance with the given 64-bit unsigned integer value.
    pub fn new(value: u64) -> Self {
        SteamId(value)
    }

    /// Creates a new `SteamId` instance from any Steam ID version.
    pub fn new_smart(value: &str) -> Result<Self, SteamIdError> {
        let mut steam_id = Self::new(0);

        let match_to_u64 = |m: Match| m.as_str().parse::<u64>().unwrap();

        if let Some(captures) = STEAM2_REGEX.captures(value) {
            let account_id = captures.name("id").map(match_to_u64).unwrap();
            if account_id > u32::MAX as u64 {
                return Err(SteamIdError::TooBig("SteamID".to_owned(), u32::MAX as u64));
            }

            let mut universe = captures.name("universe").map(match_to_u64).unwrap();
            if universe == Universe::Invalid as u64 {
                universe = Universe::Public as u64;
            }

            let auth_server = captures.name("authServer").map(match_to_u64).unwrap();
            let account_id = (account_id << 1) | auth_server;

            steam_id.set_account_universe(universe)?;
            steam_id.set_account_instance(AccountInstance::Desktop as u64)?;
            steam_id.set_account_type(AccountType::Individual as u64)?;
            steam_id.set_account_id(account_id)?;

            return Ok(steam_id);
        }

        if let Some(captures) = STEAM3_REGEX.captures(value) {
            let account_id = captures.name("id").map(match_to_u64).unwrap();
            if account_id > u32::MAX as u64 {
                return Err(SteamIdError::TooBig("SteamID".to_owned(), u32::MAX as u64));
            }

            let letter_str = captures.name("type").unwrap().as_str();
            let letter = letter_str.chars().next().unwrap();
            let mut instance = match (letter, captures.name("instance")) {
                ('U', None) => AccountInstance::Desktop,
                (_, Some(m)) => {
                    AccountInstance::try_from(match_to_u64(m)).unwrap_or(AccountInstance::All)
                }
                _ => AccountInstance::All,
            };

            match letter {
                'c' => {
                    instance = AccountInstance::FlagClan;
                    steam_id.set_account_type(AccountType::Chat as u64)?;
                }
                'L' => {
                    instance = AccountInstance::FlagLobby;
                    steam_id.set_account_type(AccountType::Chat as u64)?;
                }
                _ => steam_id.set_account_type(AccountType::from(letter) as u64)?,
            }

            let universe = captures.name("universe").map(match_to_u64).unwrap();

            steam_id.set_account_universe(universe)?;
            steam_id.set_account_instance(instance as u64)?;
            steam_id.set_account_id(account_id)?;
            return Ok(steam_id);
        }

        value
            .parse()
            .map(Self::new)
            .map_err(|_| SteamIdError::Invalid)
    }

    /// Set account universe directly.
    pub fn set_account_universe(&mut self, value: u64) -> Result<(), SteamIdError> {
        if value > 0xFF {
            return Err(SteamIdError::TooBig("Account universe".to_owned(), 0xFF));
        }

        self.set(56, 0xFF, value);

        Ok(())
    }

    /// Set account instance directly.
    pub fn set_account_instance(&mut self, value: u64) -> Result<(), SteamIdError> {
        if value > 0xFFFFF {
            return Err(SteamIdError::TooBig("Account instance".to_owned(), 0xFFFFF));
        }

        self.set(32, 0xFFFFF, value);

        Ok(())
    }

    /// Set account type directly.
    pub fn set_account_type(&mut self, value: u64) -> Result<(), SteamIdError> {
        if value > 0xF {
            return Err(SteamIdError::TooBig("Account type".to_owned(), 0xF));
        }

        self.set(52, 0xF, value);

        Ok(())
    }

    /// Set account ID directly.
    pub fn set_account_id(&mut self, value: u64) -> Result<(), SteamIdError> {
        if value > 0xFFFFFFFF {
            return Err(SteamIdError::TooBig("Account ID".to_owned(), 0xFFFFFFFF));
        }

        self.set(0, 0xFFFFFFFF, value);

        Ok(())
    }

    fn set(&mut self, offset: u8, mask: u64, value: u64) {
        self.0 = (self.0 & !(mask << offset)) | ((value & mask) << offset);
    }

    fn get(&self, offset: u8, mask: u64) -> u64 {
        (self.0 >> offset) & mask
    }

    /// Converts the `SteamId` into its underlying 64-bit unsigned integer value.
    pub fn into_u64(&self) -> u64 {
        self.0
    }

    /// Converts the `SteamId` into the unsigned 32-bit account ID used in its SteamID3 (and to some extent in the SteamID2).
    pub fn get_account_id(&self) -> u32 {
        self.get(0, 0xFFFFFFFF) as u32
    }

    /// Get account instance of the `SteamId`.
    ///
    /// Note: To get [AccountInstance] value use `AccountInstance::try_from(u32)`
    pub fn get_account_instance(&self) -> u32 {
        self.get(32, 0xFFFFF) as u32
    }

    /// Get Universe that the `SteamId` belongs to.
    pub fn get_universe(&self) -> Universe {
        Universe::try_from(self.get(56, 0xFF)).unwrap_or(Universe::Invalid)
    }

    /// Get account type of the `SteamId`.
    pub fn get_account_type(&self) -> AccountType {
        AccountType::try_from(self.get(52, 0xF)).unwrap_or(AccountType::Invalid)
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

    pub fn to_id3_string(&self) -> String {
        let account_instance = self.get_account_instance();
        let account_type = self.get_account_type();
        let mut account_type_char: char = account_type.into();

        let mut render_instance = false;

        match account_type {
            AccountType::Chat if (account_instance & AccountInstance::FlagClan as u32) != 0 => {
                account_type_char = 'c'
            }
            AccountType::Chat if (account_instance & AccountInstance::FlagLobby as u32) != 0 => {
                account_type_char = 'L'
            }
            AccountType::AnonGameServer | AccountType::Multiseat => render_instance = true,
            _ => {}
        };

        let universe = self.get_universe() as u32;
        let account_id = self.get_account_id();
        let mut string = format!("[{account_type_char}:{universe}:{account_id}");

        if render_instance {
            string += &format!(":{account_instance}");
        }

        string += "]";

        string
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

#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u64)]
pub enum AccountInstance {
    All,
    Desktop,
    Console,
    Web,

    FlagMMSLobby = 131072,
    FlagLobby = 262144,
    FlagClan = 524288,
}

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

impl From<char> for AccountType {
    fn from(value: char) -> Self {
        match value {
            'I' | 'i' => Self::Invalid,
            'U' => Self::Individual,
            'M' => Self::Multiseat,
            'G' => Self::GameServer,
            'A' => Self::AnonGameServer,
            'P' => Self::Pending,
            'C' => Self::ContentServer,
            'g' => Self::Clan,
            'T' | 'L' | 'c' => Self::Chat,
            'a' => Self::AnonUser,
            _ => Self::Invalid,
        }
    }
}

impl From<AccountType> for char {
    fn from(account_type: AccountType) -> char {
        match account_type {
            AccountType::Invalid => 'I',
            AccountType::Individual => 'U',
            AccountType::Multiseat => 'M',
            AccountType::GameServer => 'G',
            AccountType::AnonGameServer => 'A',
            AccountType::Pending => 'P',
            AccountType::ContentServer => 'C',
            AccountType::Clan => 'g',
            AccountType::Chat => 'T', // Lobby chat is 'L', Clan chat is 'c'
            AccountType::P2PSuperSeeder => ' ',
            AccountType::AnonUser => 'a',
        }
    }
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
