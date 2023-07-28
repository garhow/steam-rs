//! Definitions for errors

use crate::macros::error;

error!(SteamNewsError{
    GetNews(String)
});

error!(SteamUserError{
    GetFriendList(String),
    GetPlayerSummaries(String)
});

error!(SteamUserStatsError {
    GetGlobalAchievements(String)
});

macro_rules! ErrorHandle {
    ($function:expr, $error:expr) => {
        $function.map_err(move |error| { $error(error.to_string()) })?
    };
}

pub(crate) use ErrorHandle;

