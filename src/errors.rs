//! Definitions for errors

use crate::macros::error;

error!(PublishedFileServiceError{
    QueryFiles(String)
});

error!(PlayerServiceError{
    GetBadges(String),
    GetCommunityBadgeProgress(String),
    GetOwnedGames(String),
    GetRecentlyPlayedGames(String),
    GetSteamLevel(String)
});

error!(SiteLicenseServiceError{
    GetCurrentClientConnections(String),
    GetTotalPlaytime(String)
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
    GetTradeOffer(String),
    GetTradeOffersSummary(String),
});

error!(SteamEconomyError{
    GetAssetPrices(String),
    GetAssetClassInfo(String),
});

error!(SteamWebAPIUtilError{
    GetServerInfo(String),
    GetSupportedAPIList(String),
});


macro_rules! ErrorHandle {
    ($function:expr, $error:expr) => {
        $function.map_err(move |error| { $error(error.to_string()) })?
    };
}

pub(crate) use ErrorHandle;

