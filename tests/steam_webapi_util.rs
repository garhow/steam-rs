use steam_rs::Steam;

mod common;

#[test]
pub fn get_server_info() {
    async_test!(async {
        println!("{:?}", Steam::get_server_info().await.unwrap());
    });
}

#[test]
pub fn get_supported_api_list() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_supported_api_list().await.unwrap());
    });
}
