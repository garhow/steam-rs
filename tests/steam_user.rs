use steam_rs::{Steam, steam_id::SteamId};

mod common;

const EXAMPLE_STEAM_ID: SteamId = SteamId(76561197960435530); // Robin Walker
const EXAMPLE_VANITY_URLS: [&'static str; 2] = [
    "gabelogannewell", // Represents a working vanity URL
    "!@#$%^&*()" // Represents a broken vanity URL that would fail
];

/// Test the `GetFriendList` endpoint
#[test]
pub fn get_friend_list() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));

        // Test a working SteamID
        println!("{:?}", steam.get_friend_list(EXAMPLE_STEAM_ID, None).await.unwrap());

        // TODO: support safe error handling? need a second opinion
        // Test a broken SteamID
        // println!("{:?}", steam.get_friend_list(SteamId(20), None).await.unwrap());
    });
}

/// Test the `GetPlayerSummaries` endpoint
#[test]
pub fn get_player_summaries() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_player_summaries(vec![EXAMPLE_STEAM_ID]).await.unwrap());
    });
}

/// Test the `ResolveVanityURL` endpoint
#[test]
pub fn resolve_vanity_url() {
    async_test!(async {
        for url in EXAMPLE_VANITY_URLS {
            let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
            println!("{:?}", steam.resolve_vanity_url(url, None).await.unwrap());
        }
    });
}