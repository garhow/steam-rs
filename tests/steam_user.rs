use steam_rs::{Steam, steam_id::SteamId};
mod common;

const EXAMPLE_STEAM_ID: SteamId = SteamId(76561197960435530); // Robin Walker

#[test]
pub fn get_friend_list() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_friend_list(EXAMPLE_STEAM_ID, None).await.unwrap());
    });
}

#[test]
pub fn get_player_summaries() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_player_summaries(vec![EXAMPLE_STEAM_ID]).await.unwrap());
    });
}