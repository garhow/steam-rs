//! Implements the `QueryFiles` endpoint

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, PublishedFileServiceError},
    macros::{do_http, optional_argument},
    steam_id::{de_steamid_from_str, SteamId},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "QueryFiles";
const VERSION: &str = "1";

#[derive(Debug)]
pub enum PublishedFileQueryType {
    RankedByVote,
    RankedByPublicationDate,
    AcceptedForGameRankedByAcceptanceDate,
    RankedByTrend,
    FavoritedByFriendsRankedByPublicationDate,
    CreatedByFriendsRankedByPublicationDate,
    RankedByNumTimesReported,
    CreatedByFollowedUsersRankedByPublicationDate,
    NotYetRated,
    RankedByTotalUniqueSubscriptions,
    RankedByTotalVotesAsc,
    RankedByVotesUp,
    RankedByTextSearch,
    RankedByPlaytimeTrend,
    RankedByTotalPlaytime,
    RankedByAveragePlaytimeTrend,
    RankedByLifetimeAveragePlaytime,
    RankedByPlaytimeSessionsTrend,
    RankedByLifetimePlaytimeSessions,
    RankedByInappropriateContentRating,
    RankedByBanContentCheck,
    RankedByLastUpdatedDate,
}

impl fmt::Display for PublishedFileQueryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            PublishedFileQueryType::RankedByVote => 0,
            PublishedFileQueryType::RankedByPublicationDate => 1,
            PublishedFileQueryType::AcceptedForGameRankedByAcceptanceDate => 2,
            PublishedFileQueryType::RankedByTrend => 3,
            PublishedFileQueryType::FavoritedByFriendsRankedByPublicationDate => 4,
            PublishedFileQueryType::CreatedByFriendsRankedByPublicationDate => 5,
            PublishedFileQueryType::RankedByNumTimesReported => 6,
            PublishedFileQueryType::CreatedByFollowedUsersRankedByPublicationDate => 7,
            PublishedFileQueryType::NotYetRated => 8,
            PublishedFileQueryType::RankedByTotalUniqueSubscriptions => 9,
            PublishedFileQueryType::RankedByTotalVotesAsc => 10,
            PublishedFileQueryType::RankedByVotesUp => 11,
            PublishedFileQueryType::RankedByTextSearch => 12,
            PublishedFileQueryType::RankedByPlaytimeTrend => 13,
            PublishedFileQueryType::RankedByTotalPlaytime => 14,
            PublishedFileQueryType::RankedByAveragePlaytimeTrend => 15,
            PublishedFileQueryType::RankedByLifetimeAveragePlaytime => 16,
            PublishedFileQueryType::RankedByPlaytimeSessionsTrend => 17,
            PublishedFileQueryType::RankedByLifetimePlaytimeSessions => 18,
            PublishedFileQueryType::RankedByInappropriateContentRating => 19,
            PublishedFileQueryType::RankedByBanContentCheck => 20,
            PublishedFileQueryType::RankedByLastUpdatedDate => 21,
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug)]
pub enum PublishedFileInfoMatchingFileType {
    /// Items.
    Items,

    /// A collection of Workshop items.
    Collections,

    /// Artwork.
    Art,

    /// Videos.
    Videos,

    /// Screenshots
    Screenshots,

    /// Items that can be put inside a collection.
    CollectionEligible,

    /// Unused.
    Games,

    /// Unused.
    Software,

    /// Unused.
    Concepts,

    /// Unused.
    GreenlightItems,

    /// Guides.
    AllGuides,

    /// Steam web guide.
    WebGuides,

    /// Application integrated guide.
    IntegratedGuides,

    UsableInGame,

    /// Workshop merchandise meant to be voted on for the purpose of being sold
    Merch,

    /// Steam Controller bindings.
    ControllerBindings,

    /// Used internally.
    SteamworksAccessInvites,

    /// Workshop items that can be sold in-game.
    ItemsMtx,

    /// Workshop items that can be used right away by the user.
    ItemsReadyToUse,

    WorkshopShowcase,

    /// Managed completely by the game, not the user, and not shown on the web.
    GameManagedItems,
}

impl fmt::Display for PublishedFileInfoMatchingFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            PublishedFileInfoMatchingFileType::Items => 0,
            PublishedFileInfoMatchingFileType::Collections => 1,
            PublishedFileInfoMatchingFileType::Art => 2,
            PublishedFileInfoMatchingFileType::Videos => 3,
            PublishedFileInfoMatchingFileType::Screenshots => 4,
            PublishedFileInfoMatchingFileType::CollectionEligible => 5,
            PublishedFileInfoMatchingFileType::Games => 6,
            PublishedFileInfoMatchingFileType::Software => 7,
            PublishedFileInfoMatchingFileType::Concepts => 8,
            PublishedFileInfoMatchingFileType::GreenlightItems => 9,
            PublishedFileInfoMatchingFileType::AllGuides => 10,
            PublishedFileInfoMatchingFileType::WebGuides => 11,
            PublishedFileInfoMatchingFileType::IntegratedGuides => 12,
            PublishedFileInfoMatchingFileType::UsableInGame => 13,
            PublishedFileInfoMatchingFileType::Merch => 14,
            PublishedFileInfoMatchingFileType::ControllerBindings => 15,
            PublishedFileInfoMatchingFileType::SteamworksAccessInvites => 16,
            PublishedFileInfoMatchingFileType::ItemsMtx => 17,
            PublishedFileInfoMatchingFileType::ItemsReadyToUse => 18,
            PublishedFileInfoMatchingFileType::WorkshopShowcase => 19,
            PublishedFileInfoMatchingFileType::GameManagedItems => 20,
        };
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Preview {
    #[serde(rename = "previewid")]
    preview_id: String,
    #[serde(rename = "sortorder")]
    sort_order: u16,
    url: Option<String>,
    size: Option<u32>,
    #[serde(rename = "filename")]
    file_name: Option<String>,
    preview_type: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    tag: String,
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoteData {
    score: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaytimeStats {
    playtime_seconds: String,
    num_sessions: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    result: u64,
    #[serde(rename = "publishedfileid")]
    published_file_id: String,
    #[serde(deserialize_with = "de_steamid_from_str")]
    creator: SteamId,
    creator_appid: u32,
    consumer_appid: u32,
    consumer_shortcutid: u32,
    #[serde(rename = "filename")]
    file_name: String,
    file_size: String,
    preview_file_size: String,
    preview_url: String,
    url: String,
    hcontent_file: Option<String>,
    hcontent_preview: String,
    title: String,
    short_description: String,
    time_created: u32,
    time_updated: u32,
    visibility: u8,
    flags: u32,
    workshop_file: bool,
    workshop_accepted: bool,
    show_subscribe_all: bool,
    num_comments_public: u64,
    banned: bool,
    ban_reason: String,
    banner: String,
    can_be_deleted: bool,
    app_name: String,
    file_type: u8,
    can_subscribe: bool,
    subscriptions: u64,
    favorited: u64,
    followers: u64,
    lifetime_subscriptions: u64,
    lifetime_favorited: u64,
    lifetime_followers: u64,
    lifetime_playtime: String,
    lifetime_playtime_sessions: String,
    views: u64,
    num_children: u32,
    num_reports: u32,
    previews: Vec<Preview>,
    tags: Vec<Tag>,
    vote_data: VoteData,
    playtime_stats: PlaytimeStats,
    language: u32,
    maybe_inappropriate_sex: bool,
    maybe_inappropriate_violence: bool,
    revision_change_number: String,
    revision: u32,
    available_revisions: Vec<u32>,
    ban_text_check_result: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublishedFiles {
    total: u64,
    #[serde(rename = "publishedfiledetails")]
    published_file_details: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    response: PublishedFiles,
}

impl Steam {
    /// Performs a search query for published files.
    ///
    /// # Arguments
    ///
    /// * `query_type` - Type of the query, see [PublishedFileQueryType](crate::published_file_service::query_files::PublishedFileQueryType).
    /// * `page` - Current page. Currently there is an upper limit of 1000.
    /// * `cursor` - Cursor to paginate through the results (set to '*' for the first request). Prefer this over using the page parameter, as it will allow you to do deep pagination. When used, the page parameter will be ignored. Use the "next_cursor" value returned in the response to set up the next query to get the next set of results.
    /// * `numperpage` - The number of results, per page to return.
    /// * `creator_app_id` - App that created the files.
    /// * `app_id` - App that consumes the files.
    /// * `required_tags` - Tags to match on. See `match_all_tags` parameter.
    /// * `excluded_tags` - Tags that must NOT be present on a published file to satisfy the query.
    /// * `match_all_tags` - If true, then items must have all the tags specified, otherwise they must have at least one of the tags.
    /// * `required_flags` - Required flags that must be set on any returned items.
    /// * `omitted_flags` - Flags that must not be set on any returned items
    /// * `search_text` - Text to match in the item's title or description.
    /// * `file_type` -
    /// * `child_published_file_id` - Find all items that reference the given item.
    /// * `days` - If `query_type` is [RankedByTrend](crate::published_file_service::query_files::PublishedFileQueryType::RankedByTrend), then this is the number of days to get votes for \[1,7\].
    /// * `include_recent_votes_only` - If `query_type` is [RankedByTrend](crate::published_file_service::query_files::PublishedFileQueryType::RankedByTrend), then limit result set just to items that have votes within the day range given.
    /// * `cache_max_age_seconds` - Allow stale data to be returned for the specified number of seconds.
    /// * `language` - Language to search in and also what gets returned. Defaults to English.
    /// * `required_kv_tags` - Required key-value tags to match on.
    /// * `total_only` - If true, only return the total number of files that satisfy this query.
    /// * `ids_only` - If true, only return the published file ids of files that satisfy this query.
    /// * `return_vote_data` - Return vote data.
    /// * `return_tags` - Return tags in the file details.
    /// * `return_kv_tags` - Return key-value tags in the file details.
    /// * `return_previews` - Return preview image and video details in the file details.
    /// * `return_children` - Return child item ids in the file details.
    /// * `return_short_description` - Populate the short_description field instead of file_description.
    /// * `return_for_sale_data` - Return pricing information, if applicable.
    /// * `return_metadata` - Populate the metadata.
    /// * `return_playtime_stats` Return playtime stats for the specified number of days before today.
    pub async fn query_files(
        &self,
        query_type: PublishedFileQueryType,
        page: u32,
        cursor: &str,
        numperpage: Option<u32>, // numperpage
        creator_app_id: u32,
        app_id: u32,
        required_tags: &str,
        excluded_tags: &str,
        match_all_tags: Option<bool>,
        required_flags: &str,
        omitted_flags: &str,
        search_text: &str,
        file_type: PublishedFileInfoMatchingFileType,
        child_published_file_id: u64,
        days: u32,
        include_recent_votes_only: bool,
        cache_max_age_seconds: Option<u32>,
        language: Option<i32>,
        required_kv_tags: &str, // Documentation says this is type {message} ??
        total_only: bool,
        ids_only: bool,
        return_vote_data: bool,
        return_tags: bool,
        return_kv_tags: bool,
        return_previews: bool,
        return_children: bool,
        return_short_description: bool,
        return_for_sale_data: bool,
        return_metadata: Option<bool>,
        return_playtime_stats: u32,
    ) -> Result<PublishedFiles, PublishedFileServiceError> {
        let query = vec![
            format!("?key={}", &self.api_key),
            format!("&query_type={}", query_type),
            format!("&page={}", page),
            format!("&cursor={}", cursor),
            format!("&creator_appid={}", creator_app_id),
            format!("&appid={}", app_id),
            format!("&requiredtags={}", required_tags),
            format!("&excludedtags={}", excluded_tags),
            format!("&required_flags={}", required_flags),
            format!("&omitted_flags={}", omitted_flags),
            format!("&search_text={}", search_text),
            format!("&filetype={}", file_type),
            format!("&child_publishedfileid={}", child_published_file_id),
            format!("&days={}", days),
            format!("&include_recent_votes_only={}", include_recent_votes_only),
            format!("&required_kv_tags={}", required_kv_tags),
            format!("&totalonly={}", total_only),
            format!("&ids_only={}", ids_only),
            format!("&return_vote_data={}", return_vote_data),
            format!("&return_tags={}", return_tags),
            format!("&return_kv_tags={}", return_kv_tags),
            format!("&return_previews={}", return_previews),
            format!("&return_children={}", return_children),
            format!("&return_short_description={}", return_short_description),
            format!("&return_for_sale_data={}", return_for_sale_data),
            format!("&return_playtime_stats={}", return_playtime_stats),
            optional_argument!(numperpage),
            optional_argument!(match_all_tags),
            optional_argument!(cache_max_age_seconds),
            optional_argument!(language),
            optional_argument!(return_metadata),
        ];

        let url = format!(
            "{}/{}/{}/v{}/{}",
            BASE,
            INTERFACE,
            ENDPOINT,
            VERSION,
            query.concat()
        );

        let response = do_http!(
            url,
            Response,
            ErrorHandle,
            PublishedFileServiceError::QueryFiles
        );

        Ok(response.response)
    }
}
