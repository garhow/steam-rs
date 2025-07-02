//! Implements the `QueryFiles` endpoint

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, PublishedFileServiceError},
    macros::{do_http, optional_argument},
    steam_id::SteamId,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "QueryFiles";
const VERSION: &str = "1";

/// Represents the query types used when querying published files on Steam Workshop.
#[derive(Debug)]
pub enum PublishedFileQueryType {
    /// Ranked by vote.
    RankedByVote,
    /// Ranked by publication date.
    RankedByPublicationDate,
    /// Accepted for the game, ranked by acceptance date.
    AcceptedForGameRankedByAcceptanceDate,
    /// Ranked by trend.
    RankedByTrend,
    /// Favorited by friends, ranked by publication date.
    FavoritedByFriendsRankedByPublicationDate,
    /// Created by friends, ranked by publication date.
    CreatedByFriendsRankedByPublicationDate,
    /// Ranked by number of times reported.
    RankedByNumTimesReported,
    /// Created by followed users, ranked by publication date.
    CreatedByFollowedUsersRankedByPublicationDate,
    /// Not yet rated.
    NotYetRated,
    /// Ranked by total unique subscriptions.
    RankedByTotalUniqueSubscriptions,
    /// Ranked by total votes ascending.
    RankedByTotalVotesAsc,
    /// Ranked by votes up.
    RankedByVotesUp,
    /// Ranked by text search.
    RankedByTextSearch,
    /// Ranked by playtime trend.
    RankedByPlaytimeTrend,
    /// Ranked by total playtime.
    RankedByTotalPlaytime,
    /// Ranked by average playtime trend.
    RankedByAveragePlaytimeTrend,
    /// Ranked by lifetime average playtime.
    RankedByLifetimeAveragePlaytime,
    /// Ranked by playtime sessions trend.
    RankedByPlaytimeSessionsTrend,
    /// Ranked by lifetime playtime sessions.
    RankedByLifetimePlaytimeSessions,
    /// Ranked by inappropriate content rating.
    RankedByInappropriateContentRating,
    /// Ranked by ban content check.
    RankedByBanContentCheck,
    /// Ranked by last updated date.
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

/// Represents the matching file type for published file information.
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
    /// Usable in-game.
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
    /// Workshop showcase.
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

/// Represents a preview associated with a file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preview {
    /// The ID of the preview.
    #[serde(rename = "previewid")]
    pub preview_id: String,
    /// The sort order of the preview.
    #[serde(rename = "sortorder")]
    pub sort_order: u16,
    /// The URL of the preview, if available.
    pub url: Option<String>,
    /// The size of the preview, if available.
    pub size: Option<u32>,
    /// The filename of the preview, if available.
    #[serde(rename = "filename")]
    pub file_name: Option<String>,
    /// The type of the preview.
    pub preview_type: u8,
}

/// Represents a tag associated with a file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    /// The tag string.
    pub tag: String,
    /// The display name of the tag.
    pub display_name: String,
}

/// Represents voting data associated with a file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoteData {
    /// The score associated with the vote.
    pub score: f32,
}

/// Represents playtime statistics.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaytimeStats {
    /// The total playtime in seconds.
    pub playtime_seconds: String,
    /// The number of play sessions.
    pub num_sessions: String,
}

/// Represents file information retrieved from the Steam Workshop.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    /// Result status of the file.
    pub result: u64,
    /// The published file ID.
    #[serde(rename = "publishedfileid")]
    pub published_file_id: String,
    /// The Steam ID of the creator.
    pub creator: SteamId,
    /// The ID of the application (game) that created the file.
    pub creator_appid: u32,
    /// The ID of the application (game) that consumes the file.
    pub consumer_appid: u32,
    /// The ID of the shortcut used to create the file.
    pub consumer_shortcutid: u32,
    /// The name of the file.
    #[serde(rename = "filename")]
    pub file_name: String,
    /// The size of the file.
    pub file_size: String,
    /// The size of the preview file.
    pub preview_file_size: String,
    /// The URL of the preview.
    pub preview_url: String,
    /// The URL of the file.
    pub url: String,
    /// The content file.
    pub hcontent_file: Option<String>,
    /// The content preview.
    pub hcontent_preview: String,
    /// The title of the file.
    pub title: String,
    /// The short description of the file.
    pub short_description: String,
    /// The time the file was created.
    pub time_created: u32,
    /// The time the file was last updated.
    pub time_updated: u32,
    /// The visibility status of the file.
    pub visibility: u8,
    /// Flags associated with the file.
    pub flags: u32,
    /// Indicates if the file is from the Steam Workshop.
    pub workshop_file: bool,
    /// Indicates if the file has been accepted on the Steam Workshop.
    pub workshop_accepted: bool,
    /// Indicates if all subscribers are shown.
    pub show_subscribe_all: bool,
    /// The number of public comments.
    pub num_comments_public: u64,
    /// Indicates if the file has been banned.
    pub banned: bool,
    /// The reason for the ban.
    pub ban_reason: String,
    /// The banner of the file.
    pub banner: String,
    /// Indicates if the file can be deleted.
    pub can_be_deleted: bool,
    /// The name of the application (game).
    pub app_name: String,
    /// The file type.
    pub file_type: u8,
    /// Indicates if the user can subscribe to the file.
    pub can_subscribe: bool,
    /// The number of subscriptions.
    pub subscriptions: u64,
    /// The number of favorites.
    pub favorited: u64,
    /// The number of followers.
    pub followers: u64,
    /// The lifetime number of subscriptions.
    pub lifetime_subscriptions: u64,
    /// The lifetime number of favorites.
    pub lifetime_favorited: u64,
    /// The lifetime number of followers.
    pub lifetime_followers: u64,
    /// The lifetime playtime.
    pub lifetime_playtime: String,
    /// The lifetime playtime sessions.
    pub lifetime_playtime_sessions: String,
    /// The number of views.
    pub views: u64,
    /// The number of children.
    pub num_children: u32,
    /// The number of reports.
    pub num_reports: u32,
    /// Previews associated with the file.
    #[serde(default)]
    pub previews: Vec<Preview>,
    /// Tags associated with the file.
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Vote data associated with the file.
    pub vote_data: VoteData,
    /// Playtime statistics associated with the file.
    pub playtime_stats: PlaytimeStats,
    /// The language of the file.
    pub language: u32,
    /// Indicates if the file may contain inappropriate content related to sex.
    pub maybe_inappropriate_sex: bool,
    /// Indicates if the file may contain inappropriate content related to violence.
    pub maybe_inappropriate_violence: bool,
    /// The revision change number.
    pub revision_change_number: String,
    /// The revision number.
    pub revision: u32,
    /// Available revisions for the file.
    #[serde(default)]
    pub available_revisions: Vec<u32>,
    /// Ban text check result.
    pub ban_text_check_result: u32,
}

/// Represents published files information.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublishedFiles {
    /// The total number of published files.
    pub total: u64,
    /// Details of the published files.
    #[serde(rename = "publishedfiledetails")]
    pub published_file_details: Vec<File>,
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
    /// * `query_type` - Type of the query, see [PublishedFileQueryType].
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
