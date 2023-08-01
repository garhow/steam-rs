# Supported API endpoints

The following is a list of interfaces and endpoints that are currently supported by this crate.

Interfaces that are prefixed with ✓ are fully implemented!

## ✓ IEconService
  - [x] GetTradeHistory
  - [x] GetTradeOffers
  - [x] GetTradeOffer
  - [x] GetTradeOffersSummary
## IGameServersService
  - [ ] GetAccountList
  - [ ] CreateAccount
  - [ ] SetMemo
  - [ ] ResetLoginToken
  - [ ] DeleteAccount
  - [ ] GetAccountPublicInfo
  - [ ] QueryLoginToken
  - [ ] GetServerSteamIDsByIP
  - [ ] GetServerIPsBySteamID
## ✓ IPlayerService
  - [x] GetOwnedGames
  - [x] GetRecentlyPlayedGames
  - [x] GetSteamLevel
  - [x] GetBadges
  - [x] GetCommunityBadgeProgress
## ✓ IPublishedFileService
  - [x] QueryFiles
## ✓ ISiteLicenseService
  - [x] GetCurrentClientConnections
  - [x] GetTotalPlaytime
## ✓ ISteamApps
  - [x] GetAppList
  - [x] GetServersAtAddress
  - [x] UpToDateCheck
## ✓ ISteamEconomy
  - [x] GetAssetClassInfo
  - [x] GetAssetPrices
## ✓ ISteamNews
  - [x] GetNewsForApp
## ✓ ISteamRemoteStorage
  - [x] GetCollectionDetails
  - [x] GetPublishedFileDetails
## ISteamUser
  - [ ] GetDeletedSteamIDs
  - [x] GetFriendList
  - [x] GetPlayerAchievements
  - [ ] GetPlayerBans
  - [x] GetPlayerSummaries
  - [ ] GetUserGroupList
  - [x] GetUserStatsForGame
  - [x] ResolveVanityURL
## ISteamUserStats
  - [x] GetGlobalAchievementPercentagesForApp
  - [ ] GetGlobalStatsForGame
  - [ ] GetNumberOfCurrentPlayers
  - [ ] GetPlayerAchievements
  - [ ] GetSchemaForGame
  - [ ] GetUserStatsForGame
## ✓ ISteamWebAPIUtil
  - [x] GetServerInfo
  - [x] GetSupportedAPIList
