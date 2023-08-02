use steam_rs::{Steam, steam_id::SteamId};
mod common;

const EXAMPLE_APP_ID: u32 = 440; // Team Fortress 2
const EXAMPLE_STEAM_ID: SteamId = SteamId(76561197960435530); // Robin Walker

#[test]
pub fn get_global_achievement_percentages_for_app() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_global_achievement_percentages_for_app(EXAMPLE_APP_ID).await.unwrap());
    });
}

#[test]
pub fn get_player_achievements() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_player_achievements(EXAMPLE_STEAM_ID, EXAMPLE_APP_ID, None).await.unwrap());
    });
}