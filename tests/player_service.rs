use steam_rs::{steam_id::SteamId, Steam};
mod common;

const EXAMPLE_STEAM_ID: SteamId = SteamId(76561197960434622); // Al Farnsworth

#[tokio::test]
/// In wrong Test file!!
/// TODO: Move this
pub async fn get_badges() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    println!("{:?}", steam.get_badges(EXAMPLE_STEAM_ID).await.unwrap());
}

#[tokio::test]
pub async fn get_community_badge_progress() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    println!(
        "{:?}",
        steam
            .get_community_badge_progress(EXAMPLE_STEAM_ID, None)
            .await
            .unwrap()
    );
}

#[tokio::test]
pub async fn get_owned_games() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    println!(
        "{:?}",
        steam
            .get_owned_games(
                EXAMPLE_STEAM_ID,
                true,
                true,
                440,
                true,
                None,
                "english",
                true
            )
            .await
            .unwrap()
    );
}

#[tokio::test]
pub async fn get_recently_played_games() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    println!(
        "{:?}",
        steam
            .get_recently_played_games(EXAMPLE_STEAM_ID, Some(2))
            .await
            .unwrap()
    );
}

#[tokio::test]
pub async fn get_player_achievements() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    let req = steam.get_player_achievements(EXAMPLE_STEAM_ID, 440);
    println!(
        "{:?}",
        req
    );
}

#[tokio::test]
pub async fn get_user_stats_for_game() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    println!(
        "{:?}",
        steam
            .get_user_stats_for_game(EXAMPLE_STEAM_ID, 440)
            .await
            .unwrap()
    );
}

// #[test]
// TODO: Extensive testing for each argument
// pub fn get_owned_game() {
//    async_test!(async {
//        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
//        println!("{:?}", steam.get_owned_games(EXAMPLE_STEAM_ID, true, true, 440, true, None, "english", true).await.unwrap());
//    });
// }
