use steam_rs::{Steam, steam_id::SteamId};
mod common;

const EXAMPLE_STEAM_ID: SteamId = SteamId(76561197960434622); // Al Farnsworth

#[test]
/// In wrong Test file!!
/// TODO: Move this
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

#[test]
pub fn get_player_achievements() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_player_achievements(EXAMPLE_STEAM_ID, 440, None).await.unwrap());
    });
}

#[test]
pub fn get_user_stats_for_game() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_user_stats_for_game(EXAMPLE_STEAM_ID, 440).await.unwrap());
    });
}

#[test]
/// TODO: Extensive testing for each argument
pub fn get_owned_game() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_owned_game(EXAMPLE_STEAM_ID, true, true, 440, true, None, "english", true).await.unwrap());
    });
}