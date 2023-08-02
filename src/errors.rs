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

error!(SteamAppsError{
    GetAppList(String),
    GetServersAtAddress(String),
    UpToDateCheck(String)
});

error!(SteamNewsError{
    GetNews(String)
});

error!(SteamUserError{
    GetFriendList(String),
    GetPlayerBans(String),
    GetPlayerSummaries(String),
    ResolveVanityURL(String),
});

error!(SteamUserStatsError {
    GetGlobalAchievements(String),
    GetPlayerAchievements(String),
    GetUserStatsForGame(String),
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

error!(SteamRemoteStorageError{
    GetCollectionDetails(String),
    GetPublishedFile(String)
});


macro_rules! ErrorHandle {
    ($function:expr, $error:expr) => {
        $function.map_err(move |error| { $error(error.to_string()) })?
    };
}

pub(crate) use ErrorHandle;

