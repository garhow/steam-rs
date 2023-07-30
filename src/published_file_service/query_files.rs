use core::fmt;

use serde_derive::{Deserialize, Serialize};

//use serde_derive::{Serialize, Deserialize};
use crate::{Steam,
    errors::{PublishedFileServiceError, ErrorHandle},
    macros::{do_http, optional_argument},
    BASE, steam_id::{SteamId, de_steamid_from_str},
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
    RankedByLastUpdatedDate
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
            PublishedFileQueryType::RankedByLastUpdatedDate => 21
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug)]
pub enum PublishedFileInfoMatchingFileType {
    Items,
    Collections,
    Art,
    Videos,
    Screenshots,
    CollectionEligible,
    Games,
    Software,
    Concepts,
    GreenlightItems,
    AllGuides,
    WebGuides,
    IntegratedGuides,
    UsableInGame,
    Merch,
    ControllerBindings,
    SteamworksAccessInvites,
    ItemsMtx,
    ItemsReadyToUse,
    WorkshopShowcase,
    GameManagedItems
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
            PublishedFileInfoMatchingFileType::GameManagedItems => 20
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
    url: String,
    size: u32,
    #[serde(rename = "filename")]
    file_name: String,
    preview_type: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    tag: String,
    display_name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoteData {
    score: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaytimeStats {
    playtime_seconds: String,
    num_sessions: String
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
    hcontent_file: String,
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
    ban_text_check_result: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublishedFiles {
    total: u64,
    #[serde(rename = "publishedfiledetails")]
    published_file_details: Vec<File>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    response: PublishedFiles
}

impl Steam {
    pub async fn query_files(
        &self,
        query_type: PublishedFileQueryType,
        page: u32,
        cursor: &str,
        num_per_page: Option<u32>, // numperpage
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
        return_playtime_stats: u32
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
            optional_argument!(num_per_page, "numperpage"),
            optional_argument!(match_all_tags),
            optional_argument!(cache_max_age_seconds),
            optional_argument!(language),
            optional_argument!(return_metadata),
        ];

        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query.concat());

        let response = do_http!(url, Response, ErrorHandle, PublishedFileServiceError::QueryFiles);

        Ok(response.response)
    }
}
