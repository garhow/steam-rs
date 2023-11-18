use steam_rs::Steam;
mod common;

#[test]
pub fn get_app_list() {
    async_test!(async {
        println!("{:?}", Steam::get_app_list().await.unwrap());
    });
}

#[test]
pub fn get_servers_at_address() {
    async_test!(async {
        println!(
            "{:?}",
            Steam::get_servers_at_address("158.255.2.173:27016")
                .await
                .unwrap()
        );
    });
}

#[test]
pub fn up_to_date_check() {
    async_test!(async {
        println!("{:?}", Steam::up_to_date_check(440, 8227024).await.unwrap());
    });
}
