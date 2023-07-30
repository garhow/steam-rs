use steam_rs::{Steam, steam_id::SteamId};
mod common;

const EXAMPLE_STEAM_ID: SteamId = SteamId(76561197960434622); // Al Farnsworth

#[test]
pub fn get_owned_games() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_owned_games(EXAMPLE_STEAM_ID, None, None, None).await.unwrap());
    });
}

#[test]
pub fn get_recently_played_games() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_recently_played_games(EXAMPLE_STEAM_ID, Some(2)).await.unwrap());
    });
}