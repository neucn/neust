//! # neust: NEU passport binding for Rust.
//! Neust is an reqwest-based client, to help developers easily build
//! apps for NEU.
//!
#![deny(missing_debug_implementations, unreachable_pub)]

pub use reqwest;

pub use self::error::*;
pub use self::session::*;
pub use self::status::*;

mod platform;

mod error;
mod session;
mod status;

pub mod auth;

#[cfg(feature = "webvpn")]
pub mod webvpn;
