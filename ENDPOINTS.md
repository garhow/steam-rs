# Supported API endpoints

The following is a list of interfaces and endpoints that are currently supported by this crate.

Interfaces that are prefixed with ✓ are fully implemented!

## ✓ IEconService
  - [ ] FlushAssetAppearanceCache **(requires publisher key)**
  - [ ] FlushContextCache **(requires publisher key)**
  - [ ] FlushInventoryCache **(requires publisher key)**
  - [x] GetTradeHistory
  - [x] GetTradeOffers
  - [x] GetTradeOffer
  - [x] GetTradeOffersSummary
## IGameServersService
  - [x] GetAccountList
  - [ ] CreateAccount
  - [ ] SetMemo
  - [ ] ResetLoginToken
  - [ ] DeleteAccount
  - [x] GetAccountPublicInfo
  - [x] QueryLoginToken
  - [x] GetServerSteamIDsByIP
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
  - [ ] EnumerateUserSubscribedFiles **(requires publisher key)**
  - [x] GetCollectionDetails
  - [x] GetPublishedFileDetails
  - [ ] GetUGCFileDetails
  - [ ] SetUGCUsedByGC **(requires publisher key)**
  - [ ] SubscribePublishedFile **(requires publisher key)**
  - [ ] UnsubscribePublishedFile **(requires publisher key)**
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
## ISteamUserAuth
  - [ ] AuthenticateUser
  - [x] AuthenticateUserTicket
## ISteamUserStats
  - [x] GetGlobalAchievementPercentagesForApp
  - [ ] GetGlobalStatsForGame
  - [x] GetNumberOfCurrentPlayers
  - [x] GetPlayerAchievements
  - [x] GetSchemaForGame
  - [x] GetUserStatsForGame
## ✓ ISteamWebAPIUtil
  - [x] GetServerInfo
  - [x] GetSupportedAPIList
