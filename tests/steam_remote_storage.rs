use steam_rs::Steam;
mod common;

const EXAMPLE_COLLECTION: u64 = 532551393;

#[test]
pub fn get_collection_details() {
    async_test!(async {
        println!("{:?}", Steam::get_collection_details(&[EXAMPLE_COLLECTION]).await.unwrap());
    });
}