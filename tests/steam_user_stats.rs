use steam_rs::{Steam, AppId};
mod common;

const EXAMPLE_APP_ID: AppId = AppId(440); // Team Fortress 2

#[test]
pub fn get_global_achievement_percentages_for_app() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_global_achievement_percentages_for_app(EXAMPLE_APP_ID).await.unwrap());
    });
}