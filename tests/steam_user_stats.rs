use steam_rs::{steam_id::SteamId, Steam};
mod common;

const EXAMPLE_APP_ID: u32 = 440; // Team Fortress 2
const EXAMPLE_STEAM_ID_PUBLIC: SteamId = SteamId(76561198136162943); // Garrett Howard
const EXAMPLE_STEAM_ID_PRIVATE: SteamId = SteamId(76561197960435530); // Robin Walker

#[tokio::test]
pub async fn get_global_achievement_percentages_for_app() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    assert!(steam
        .get_global_achievement_percentages_for_app(EXAMPLE_APP_ID)
        .await
        .is_ok());
}

#[tokio::test]
pub async fn get_number_of_current_players() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));

    // Expected result
    assert!(steam
        .get_number_of_current_players(EXAMPLE_APP_ID)
        .await
        .is_ok());

    // Error condition (nonexistent app)
    assert!(steam.get_number_of_current_players(1).await.is_err());
}

#[tokio::test]
pub async fn get_player_achievements() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));

    // Expected result (public profile)
    assert!(steam
        .get_player_achievements(EXAMPLE_STEAM_ID_PUBLIC, EXAMPLE_APP_ID, None)
        .await
        .is_ok());

    // Error condition (private profile)
    assert!(steam
        .get_player_achievements(EXAMPLE_STEAM_ID_PRIVATE, EXAMPLE_APP_ID, None)
        .await
        .is_err());
}

#[tokio::test]
pub async fn get_schema_for_game() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));

    assert!(steam
        .get_schema_for_game(EXAMPLE_APP_ID, None)
        .await
        .is_ok());
}
