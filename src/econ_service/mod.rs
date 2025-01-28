//! # Implements the `IEconService` interface
//!
//! Additional Steam Economy methods that provide access to Steam Trading.
//!
//! **Note:** This implementation is incomplete! The following endpoints are currently unimplemented
//!
//! - FlushAssetAppearanceCache (requires publisher key)
//! - FlushContextCache (requires publisher key)
//! - FlushInventoryCache (requires publisher key)
//!
//! Endpoints that require a publisher key are not likely to be
//! implemented in the near future, as they cannot be tested by developers.

pub mod get_trade_history;
pub mod get_trade_offer;
pub mod get_trade_offers;
pub mod get_trade_offers_summary;
