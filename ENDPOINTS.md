# Supported API endpoints

The following is a list of interfaces and endpoints that are currently supported by this crate.

Interfaces that are prefixed with ✓ are fully implemented!

## IEconService
  - [ ] GetTradeHistory
  - [ ] GetTradeOffers
  - [ ] GetTradeOffer
  - [ ] GetTradeOffersSummary
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
## ISiteLicenseService
  - [ ] GetCurrentClientConnections
  - [ ] GetTotalPlaytime
## ISteamApps
  - [ ] GetAppList
  - [ ] GetServersAtAddress
  - [ ] UpToDateCheck
## ✓ ISteamEconomy
  - [x] GetAssetClassInfo
  - [x] GetAssetPrices
## ✓ ISteamNews
  - [x] GetNewsForApp
## ISteamRemoteStorage
  - [ ] GetCollectionDetails
  - [ ] GetPublishedFileDetails
## ISteamUser
  - [ ] GetDeletedSteamIDs
  - [x] GetFriendList
  - [ ] GetPlayerAchievements
  - [ ] GetPlayerBans
  - [x] GetPlayerSummaries
  - [ ] GetUserGroupList
  - [ ] GetUserStatsForGame
  - [ ] ResolveVanityURL
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
