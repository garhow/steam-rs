use steam_rs::Steam;
mod common;

#[tokio::test]
pub async fn get_asset_class_info() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));

    let data = steam
        .get_asset_class_info(440, None, 1, 211447708, None)
        .await
        .unwrap();
    println!("{data:?}")
}

#[tokio::test]
pub async fn get_asset_prices() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));

    let tf2_data = steam.get_asset_prices(440, None, None).await.unwrap();
    println!("{tf2_data:?}");

    let cs2_data = steam
        .get_asset_prices(730, None, None)
        .await
        .unwrap()
        .assets;

    println!("{cs2_data:?}");
}
