use steam_rs::Steam;
mod common;


#[test]
pub fn get_asset_class_info() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        
        let data = steam.get_asset_class_info(440, None, 1, 211447708, None).await.unwrap();
        println!("{data:?}");
    });
}

#[test]
pub fn get_asset_prices() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        
        steam.get_asset_prices(440, None, None).await.unwrap();

    });
}
