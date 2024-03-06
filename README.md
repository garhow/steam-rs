# steam-rs

[![Crate at crates.io](https://img.shields.io/crates/v/steam-rs.svg)](https://crates.io/crates/steam-rs)
[![Documentation at docs.rs](https://docs.rs/steam-rs/badge.svg)](https://docs.rs/steam-rs)
[![MIT licensed](https://img.shields.io/crates/l/steam-rs.svg)](./LICENSE)
[![CI](https://github.com/garhow/steam-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/garhow/steam-rs/actions/workflows/ci.yml)

Provides safe and convenient Rust bindings for the Steam Web API.

> [!IMPORTANT]
> As this project is still a work-in-progress, not all API endpoints are currently supported. Most unsupported endpoints require a Steamworks Publisher API key, something that none of the developers of this project currently have. For a list of all supported API interfaces and endpoints, please see [ENDPOINTS.md](./ENDPOINTS.md).

> [!NOTE]
> This project is in early stages of development, so bug reports, suggestions, and pull requests are highly appreciated!

## Usage

```rust
use std::env;
use steam_rs::{Steam, SteamId};

#[tokio::main]
async fn main() {
    // Get the Steam API Key as an environment variable
    let steam_api_key = env::var("STEAM_API_KEY").expect("Missing an API key");

    // Initialize the Steam API client
    let steam = Steam::new(steam_api_key);

    // Request the recently played games of SteamID `76561197960434622`
    let steam_id = SteamId::new(76561197960434622);
    let recently_played_games = steam.get_recently_played_games(steam_id, None).await.unwrap();

    // Print the total count of the user's recently played games
    println!("{}", recently_played_games.total_count);
}
```

## Acknowledgements

The following resources were used in the developement in this crate:
- [Better Steam Web API Documentation](https://steamwebapi.azurewebsites.net/)
- [Steam Web API - Valve Developer Community](https://developer.valvesoftware.com/wiki/Steam_Web_API)
- [Steamworks Web API Reference](https://partner.steamgames.com/doc/webapi)
- [WebAPI - Official TF2 Wiki](https://wiki.teamfortress.com/wiki/WebAPI)
- [xpaw's Steam Web API Documentation](https://steamapi.xpaw.me/)

Special thanks to [@harvey298](https://github.com/harvey298/) for developing the custom macros and error types used in this project.
