# steam-rs

[![Crate at crates.io](https://img.shields.io/crates/v/steam-rs.svg)](https://crates.io/crates/steam-rs)
[![Documentation at docs.rs](https://docs.rs/steam-rs/badge.svg)](https://docs.rs/steam-rs)
[![MIT licensed](https://img.shields.io/crates/l/steam-rs.svg)](./LICENSE)
[![CI](https://github.com/garhow/steam-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/garhow/steam-rs/actions/workflows/ci.yml)

This crate provides safe and convenient Rust bindings for the Steam Web API, as documented in the [Steamworks Web API Reference](https://partner.steamgames.com/doc/webapi) and [Better Steam Web API Documentation](https://steamwebapi.azurewebsites.net/).

### This is not production ready!
This crate is highly experimental and unfinished so you are advised to be cautious when using it in your projects.

### Supported API endpoints
For a list of all supported API interfaces and endpoints, please see [ENDPOINTS.md](./ENDPOINTS.md).

## Usage


```rust
use std::env;
use steam_rs::{Steam, SteamId};

#[tokio::main]
async fn main() {
    // Get the Steam API Key as an environment variable
    let steam_api_key = env::var("STEAM_API_KEY").expect("Missing an API key")

    // Initialize the Steam API client
    let steam = Steam::new(steam_api_key);

    // Request the recently played games of SteamID 76561197960434622
    let steam_id = SteamId::new(76561197960434622);
    let recently_played_games = steam.get_recently_played_games(steam_id, None).await.unwrap();

    // Print the total count of the user's recently played games
    println!("{}", recently_played_games.total_count);
}
```

## Contributing
This project is in early stages of development, so bug reports, suggestions, and pull requests are highly appreciated!

## License
This project is licensed under the [MIT License](./LICENSE).
