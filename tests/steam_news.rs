use steam_rs::Steam;
mod common;

const EXAMPLE_APP_ID: u32 = 440; // Team Fortress 2

#[test]
pub fn get_news() {
    async_test!(async {
        // let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!(
            "{:?}",
            Steam::get_news_for_app(EXAMPLE_APP_ID, Some(200), None, Some(10), None)
                .await
                .unwrap()
        );
    });
}
