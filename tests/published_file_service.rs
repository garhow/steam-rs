use steam_rs::{
    published_file_service::query_files::{
        PublishedFileInfoMatchingFileType, PublishedFileQueryType,
    },
    Steam,
};

mod common;

const EXAMPLE_APP_ID: u32 = 440; // Team Fortress 2
static EXAMPLE_TAG: &str = "Capture the Flag";

#[tokio::test]
pub async fn query_files() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    let query = steam
        .query_files(
            PublishedFileQueryType::RankedByVote,
            0,
            "*",
            Some(5),
            EXAMPLE_APP_ID,
            EXAMPLE_APP_ID,
            vec![],
            vec![],
            None,
            vec![],
            vec![],
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

    let no_tag = query.total;
    println!("Total Results: {no_tag}");

    let query = steam
        .query_files(
            PublishedFileQueryType::RankedByVote,
            0,
            "*",
            Some(5),
            EXAMPLE_APP_ID,
            EXAMPLE_APP_ID,
            vec![EXAMPLE_TAG.to_string()],
            vec![],
            Some(true),
            vec![],
            vec![],
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
    let with_include_tag = query.total;
    println!("Includes tag '{EXAMPLE_TAG}': {with_include_tag}");

    let query = steam
        .query_files(
            PublishedFileQueryType::RankedByVote,
            0,
            "*",
            Some(5),
            EXAMPLE_APP_ID,
            EXAMPLE_APP_ID,
            vec![],
            vec![EXAMPLE_TAG.to_string()],
            Some(true),
            vec![],
            vec![],
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
    let with_exclude_tag = query.total;
    println!("Excludes tag '{EXAMPLE_TAG}': {with_exclude_tag}");

    assert_ne!(
        with_include_tag, with_exclude_tag,
        "results should not be equal"
    );
    assert_eq!(
        no_tag,
        with_include_tag + with_exclude_tag,
        "mutually exclusive results should return the complete total"
    );
}
