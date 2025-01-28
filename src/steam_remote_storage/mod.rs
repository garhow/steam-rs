//! # Implements the `ISteamRemoteStorage` interface
//!
//! Primary interface for interacting with the Steam Workshop and User Generated Content (UGC).
//!
//! **Note:** This implementation is incomplete! The following endpoints are currently unimplemented
//!
//! - EnumerateUserSubscribedFiles (requires publisher key)
//! - GetUGCFileDetails
//! - SetUGCUsedByGC (requires publisher key)
//! - SubscribePublishedFile (requires publisher key)
//! - UnsubscribePublishedFile (requires publisher key)
//!
//! Endpoints that require a publisher key are not likely to be
//! implemented in the near future, as they cannot be tested by developers.

const INTERFACE: &str = "ISteamRemoteStorage";

pub mod get_collection_details;
pub mod get_published_file;
