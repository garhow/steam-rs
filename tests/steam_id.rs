use steam_rs::steam_id::SteamId;

mod common;

const EXAMPLE_STEAM_ID64: u64 = 76561198307851839; // m1nt_

#[tokio::test]
pub async fn new() {
    println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64));
}

#[tokio::test]
pub async fn into_u64() {
    println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).into_u64());
}

#[tokio::test]
pub async fn get_account_id() {
    println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).get_account_id());
}

#[tokio::test]
pub async fn get_universe() {
    println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).get_universe());
}

#[tokio::test]
pub async fn get_account_type() {
    println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).get_account_type());
}

#[tokio::test]
pub async fn to_id2_string() {
    println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).to_id2_string());
}