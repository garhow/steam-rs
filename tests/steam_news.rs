use steam_rs::Steam;
mod common;

#[test]
pub fn get_news() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").unwrap());
        let app_id = todo!("app id - I'm too lazy");
        let news = steam.get_news_for_app(app_id, 1, 10).await.unwrap();
        println!("{:?}", news);
    });
}