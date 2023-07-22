//! A library to easily explore music theory principles.
//!
//! # Examples
//!
//! ```
//! use rtzlib::get_timezone;
//!
//! // Query a time zone for a given `(lng,lat)`.
//! assert_eq!(
//!     get_timezone(-121., 46.)
//!         .unwrap()
//!         .friendly_name
//!         .as_ref()
//!         .unwrap(),
//!     "America/Los_Angeles"
//! );
//! ```

// Directives.

#![warn(rustdoc::broken_intra_doc_links, rust_2018_idioms, clippy::all, missing_docs)]
#![allow(incomplete_features)]
#![feature(async_closure)]
#![feature(test)]
#![feature(string_remove_matches)]
#![feature(fs_try_exists)]

// Modules.

pub mod base;
pub use crate::base::geo::{generate_bincodes, get_timezone, get_timezone_via_full_lookup};
pub use crate::base::types::Void;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "web")]
pub mod web;
#[cfg(feature = "web")]
pub use crate::web::server_start;