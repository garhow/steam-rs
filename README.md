# steam-rs
A wrapper for the Steam Web API in Rust.

This is based on the [Steam Web API documentation](https://developer.valvesoftware.com/wiki/Steam_Web_API) that can be found at the Valve Developer Community.

This crate is highly experimental and unfinished so you are advised to be cautious when using it in your projects.

## Supported API endpoints
- [ ] IPlayerService
  - [x] GetOwnedGames
  - [ ] GetRecentlyPlayedGames
- [x] ISteamNews
  - [x] GetNewsForApp
- [ ] ISteamUser
  - [x] GetPlayerSummaries
  - [x] GetFriendList
  - [ ] GetPlayerAchievements
  - [ ] GetUserStatsForGame
  - [ ] GetOwnedGames
- [x] ISteamUserStats
  - [x] GetGlobalAchievementPercentagesForApp