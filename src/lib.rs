//! # neust: NEU passport binding for Rust.
//! Neust is an reqwest-based client, to help developers easily build
//! apps for NEU.
//!
// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub use reqwest;

mod platform;

mod error;
mod session;

pub use self::error::*;
pub use self::session::*;

pub mod auth;

#[cfg(feature = "webvpn")]
mod webvpn;
#[cfg(feature = "webvpn")]
pub use self::webvpn::*;
