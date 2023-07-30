use steam_rs::{Steam};
mod common;


#[test]
pub fn get_trade_history() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        
        steam.get_trade_history(10, 1000, 1000, false, false, "english", false, false).await.unwrap();

    });
}

#[test]
pub fn get_trade_offers() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        
        steam.get_trade_offers(true, true, true, "english", false, false, 10000).await.unwrap();

    });
}