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
  - [ ] CheckAppOwnership **(requires publisher key)**
  - [ ] GetAppPriceInfo **(requires publisher key)**
  - [ ] GetDeletedSteamIDs **(requires publisher key)**
  - [x] GetFriendList
  - [x] GetPlayerAchievements
  - [x] GetPlayerBans
  - [x] GetPlayerSummaries
  - [ ] GetPublisherAppOwnership **(requires publisher key)**
  - [ ] GetPublisherAppOwnershipChanges **(requires publisher key)**
  - [x] GetUserGroupList
  - [ ] GrantPackage **(requires publisher key)**
  - [x] ResolveVanityURL
  - [ ] RevokePackage **(requires publisher key)**
## ISteamUserStats
  - [x] GetGlobalAchievementPercentagesForApp
  - [ ] GetGlobalStatsForGame
  - [ ] GetNumberOfCurrentPlayers
  - [x] GetPlayerAchievements
  - [ ] GetSchemaForGame
  - [x] GetUserStatsForGame
## ✓ ISteamWebAPIUtil
  - [x] GetServerInfo
  - [x] GetSupportedAPIList
