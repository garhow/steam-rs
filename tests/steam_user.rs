use steam_rs::{steam_id::SteamId, Steam};

mod common;

const EXAMPLE_STEAM_ID: SteamId = SteamId(76561197960435530); // Robin Walker
const EXAMPLE_VANITY_URLS: [&str; 2] = [
    "gabelogannewell", // Represents a working vanity URL
    "!@#$%^&*()",      // Represents a broken vanity URL that would fail
];

#[tokio::test]
pub async fn get_friend_list() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    println!(
        "{:?}",
        steam.get_friend_list(EXAMPLE_STEAM_ID, None).await.unwrap()
    );
}

#[tokio::test]
pub async fn get_player_bans() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    println!(
        "{:?}",
        steam.get_player_bans(vec![EXAMPLE_STEAM_ID]).await.unwrap()
    );
}

#[tokio::test]
pub async fn get_player_summaries() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    println!(
        "{:?}",
        steam
            .get_player_summaries(vec![EXAMPLE_STEAM_ID])
            .await
            .unwrap()
    );
}

#[tokio::test]
pub async fn get_user_group_list() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    println!(
        "{:?}",
        steam.get_user_group_list(EXAMPLE_STEAM_ID).await.unwrap()
    );
}

#[tokio::test]
pub async fn resolve_vanity_url() {
    for url in EXAMPLE_VANITY_URLS {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!(
            "{:?}",
            steam.resolve_vanity_url(url, Some(1)).await.unwrap()
        );
    }
}
