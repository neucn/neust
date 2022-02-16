//! # neust: NEU passport binding for Rust.
//! Neust is an reqwest-based client, to help developers easily build
//! apps for NEU.
//!
#![deny(missing_debug_implementations, unreachable_pub)]

pub use reqwest;

pub use self::error::*;
pub use self::session::*;
#[cfg(feature = "webvpn")]
pub use self::webvpn::*;

mod platform;

mod error;
mod session;

pub mod auth;

#[cfg(feature = "webvpn")]
mod webvpn;
