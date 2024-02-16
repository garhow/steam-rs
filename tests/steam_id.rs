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
pub fn into_u64() {
    async_test!(async {
        println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).into_u64());
    });
}

#[test]
pub fn get_account_id() {
    async_test!(async {
        println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).get_account_id());
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

#[test]
pub fn to_id2_string() {
    async_test!(async {
        println!("{:?}", SteamId::new(EXAMPLE_STEAM_ID64).to_id2_string());
    });
}