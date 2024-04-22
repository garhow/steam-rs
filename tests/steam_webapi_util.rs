use steam_rs::Steam;

mod common;

#[tokio::test]
pub async fn get_server_info() {
        println!("{:?}", Steam::get_server_info().await.unwrap());
}

#[tokio::test]
pub async fn get_supported_api_list() {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_supported_api_list().await.unwrap());
}
