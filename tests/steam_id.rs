use std::str::FromStr;
use steam_rs::steam_id::SteamId;

mod common;

const EXAMPLE_STEAM_ID64: u64 = 76561198307851839; // m1nt_

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

#[test]
pub fn into_u32() {
    async_test!(async {
        println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).into_u32());
    });
}

#[test]
pub fn get_universe() {
    async_test!(async {
        println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).get_universe());
    });
}

#[test]
pub fn get_account_type() {
    async_test!(async {
        println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).get_account_type());
    });
}