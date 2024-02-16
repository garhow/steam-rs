use steam_rs::{steam_id::SteamId, Steam};

#[tokio::main]
async fn main() {
    // Get the Steam API Key as an environment variable.
    let steam_api_key = &std::env::var("STEAM_API_KEY").expect("Missing an API key");

    // Initialize the Steam API client.
    let steam = Steam::new(steam_api_key);

    // Request the player summaries of SteamIDs `76561198136162943` and `76561197960435530`.
    let steam_ids = vec![
        SteamId::new(76561198136162943), // Garrett Howard
        SteamId(76561197960435530), // Robin Walker
    ];

    let player_summaries = steam.get_player_summaries(steam_ids).await.unwrap();

    // Print the recieved information about the players.
    for player in player_summaries {
        println!("{:?}'s SteamID is {:?}", player.persona_name, player.steam_id)
    }
}