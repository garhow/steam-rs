use std::str::FromStr;
use steam_rs::steam_id::SteamId;

mod common;

const EXAMPLE_STEAM_ID64: u64 = 76561197960435530; // Robin Walker

#[test]
pub fn new() {
    async_test!(async {
        println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64));
    });
}

#[test]
pub fn from_str() {
    async_test!(async {
        println!("{:?}", SteamId::from_str(&EXAMPLE_STEAM_ID64.to_string()).unwrap());
    });
}

#[test]
pub fn into_u64() {
    async_test!(async {
        println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).into_u64());
    });
}