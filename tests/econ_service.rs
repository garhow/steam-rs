use steam_rs::Steam;
mod common;

#[tokio::test]
pub async fn get_trade_history() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    steam
        .get_trade_history(10, 0, 0, false, true, "english", true, true)
        .await
        .unwrap();
}

#[tokio::test]
pub async fn get_trade_offers() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    steam
        .get_trade_offers(true, true, true, "english", false, false, 10000)
        .await
        .unwrap();
}

/// This Test is inhertintly flawed because trade offers expire after 2 weeks.
/// See: https://help.steampowered.com/en/faqs/view/1115-91C5-050C-1D60
#[tokio::test]
#[ignore = "Test is flawed. See: https://help.steampowered.com/en/faqs/view/1115-91C5-050C-1D60"]
pub async fn get_trade_offer() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    steam.get_trade_offer(6271878669, "english").await.unwrap();
}

#[tokio::test]
pub async fn get_trade_offers_summary() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    steam.get_trade_offers_summary(0).await.unwrap();
}
