use steam_rs::Steam;
mod common;

const EXAMPLE_COLLECTION: u64 = 532551393;

#[tokio::test]
pub async fn get_collection_details() {
    println!(
        "{:?}",
        Steam::get_collection_details(&[EXAMPLE_COLLECTION])
            .await
            .unwrap()
    );
}

#[tokio::test]
pub async fn get_published_file() {
    println!(
        "{:?}",
        Steam::get_published_file(&[EXAMPLE_COLLECTION])
            .await
            .unwrap()
    );
}
