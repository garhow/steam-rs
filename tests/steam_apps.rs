use steam_rs::Steam;
mod common;

#[tokio::test]
pub async fn get_app_list() {
    println!("{:?}", Steam::get_app_list().await.unwrap());
}

#[tokio::test]
pub async fn get_servers_at_address() {
    println!(
        "{:?}",
        Steam::get_servers_at_address("158.255.2.173:27016")
            .await
            .unwrap()
    );
}

#[tokio::test]
pub async fn up_to_date_check() {
    println!("{:?}", Steam::up_to_date_check(440, 8227024).await.unwrap());
}
