use steam_rs::{
    published_file_service::query_files::{
        PublishedFileInfoMatchingFileType, PublishedFileQueryType,
    },
    Steam,
};

mod common;

const EXAMPLE_APP_ID: u32 = 440; // Team Fortress 2

#[test]
pub fn query_files() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        let query = steam
            .query_files(
                PublishedFileQueryType::RankedByVote,
                0,
                "*",
                Some(5),
                EXAMPLE_APP_ID,
                EXAMPLE_APP_ID,
                "",
                "",
                None,
                "",
                "",
                "",
                PublishedFileInfoMatchingFileType::Items,
                0,
                7,
                false,
                None,
                None,
                "",
                false,
                false,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                Some(true),
                10,
            )
            .await
            .unwrap();
        println!("{:?}", query);
    });
}
