use steam_rs::steam_id::{SteamId, Universe};

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

#[tokio::test]
async fn new_smart() {
    let sid = SteamId::new_smart("STEAM_1:1:161214314");
    assert!(sid.is_ok());

    let mut sid = sid.unwrap();
    assert_eq!(76561198282694357, sid.0);
    assert_eq!(322428629, sid.get_account_id());
    assert_eq!("STEAM_1:1:161214314".to_owned(), sid.to_id2_string());
    assert_eq!("[U:1:322428629]".to_owned(), sid.to_id3_string());
    assert_eq!(Universe::Public, sid.get_universe());

    sid.set_account_universe(Universe::Internal as u64).unwrap();
    assert_eq!(Universe::Internal, sid.get_universe());
}

#[tokio::test]
async fn from_account_id() {
    let sid = SteamId::from_account_id(322428629);
    assert_eq!(Ok(76561198282694357), sid.map(|s| s.0))
}
