//! # Implements the `IPublishedFileService` interface
//!
//! **Note:** This implementation is incomplete!
//! The following endpoints are currently unimplemented:
//!
//! - CanSubscribe (undocumented)
//! - Delete (undocumented)
//! - GetDetails
//! - GetSubSectionData
//! - GetUserFileCount
//! - GetUserFiles
//! - GetUserVoteSummary
//! - Publish (undocumented)
//! - RefreshVotingQueue (requires publisher key)
//! - SetDeveloperMetadata (requires publisher key)
//! - Subscribe (requires publisher key)
//! - Unsubscribe (requires publisher key)
//! - Update (requires publisher key)
//! - UpdateAppUGCBan (requires publisher key)
//! - UpdateBanStatus (requires publisher key)
//! - UpdateIncompatibleStatus (requires publisher key)
//! - UpdateKeyValueTags (requires publisher key)
//! - UpdateTags (requires publisher key)
//! - Vote (requires publisher key)
//!
//! Endpoints that require a publisher key are not likely to be
//! implemented in the near future, as they cannot be tested by developers.

pub mod query_files;

const INTERFACE: &str = "IPublishedFileService";
