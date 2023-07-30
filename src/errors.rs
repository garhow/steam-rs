//! Definitions for errors

use crate::macros::error;

error!(PublishedFileServiceError{
    QueryFiles(String)
});

error!(PlayerServiceError{
    GetOwnedGames(String),
    GetRecentlyPlayedGames(String),
    GetSteamLevel(String)
});

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

error!(EconServiceError{
    GetTradeHistory(String),
    GetTradeOffers(String),
});

error!(SteamEconomyError{
    GetAssetPrices(String),
    GetAssetClassInfo(String),
});


macro_rules! ErrorHandle {
    ($function:expr, $error:expr) => {
        $function.map_err(move |error| { $error(error.to_string()) })?
    };
}

pub(crate) use ErrorHandle;

