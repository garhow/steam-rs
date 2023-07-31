use steam_rs::Steam;
mod common;

const EXAMPLE_APP_ID: u32 = 440; // Team Fortress 2

#[test]
pub fn get_collection_details() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_collection_details(&[]).await.unwrap());
    });
}