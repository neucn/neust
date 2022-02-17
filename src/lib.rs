//! # neust
//! > NEU CAS binding for Rust.
//!
//! ## Explanation: Endpoints
//!
//! See [documentation for endpoints](crate::doc::endpoint).
//!
//! ## Examples
//!
//! For binary program examples, see
//! [neust/examples](https://github.com/neucn/neust/tree/master/examples).
//!
//! ### Query username using token
//! ```no_run
//! # async fn doc() -> Result<(), neust::Error> {
//! use neust::{Session, auth};
//!
//! let token = auth::Token::new("your_token");
//! let session = Session::new();
//! let status = session.login(&token).await?;
//! let username = status.get_username();
//! # Ok(())
//! # }
//! ```
//!
//! ### Access intranet service via WebVPN
//!
//! Should enable feature: **webvpn**.
//!
//! ```no_run
//! # #[cfg(feature = "webvpn")]
//! # async fn doc() -> Result<(), neust::Error> {
//! use neust::{Session, auth, webvpn};
//!
//! let credential = auth::Credential::new("username", "password");
//! let session = Session::new();
//! if !session.login(&credential).await?.is_active()
//!     || !session.login_via_webvpn(&credential).await?.is_active() {
//!     panic!("fail");
//! }
//! let client = session.client();
//! let request = client
//!     .get(webvpn::encrypt_url(
//!         "http://219.216.96.4/eams/teach/grade/course/person!search.action?semesterId=0",
//!     ))
//!     .build()?;
//! let response = client.execute(request).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Use Wechat to login
//!
//! Should enable feature: **wechat**.
//!
//! ```no_run
//! # #[cfg(feature = "wechat")]
//! # async fn doc() -> Result<(), neust::Error> {
//! use neust::{Session, auth};
//! use tokio::time::{sleep, Duration};
//!
//! let wechat = auth::Wechat::default();
//! let session = Session::new();
//!
//! sleep(Duration::from_secs(10)).await;
//!
//! let status = session.login(&wechat).await?;
//! let username = status.get_username();
//! # Ok(())
//! # }
//! ```
//!
//! ## Optional Features
//!
//! - **webvpn**: supports for WebVPN endpoint.
//! - **wechat**: supports for authorization by Wechat.
//! - **native-tls** *(enabled by default)*: Enables TLS functionality provided by `native-tls`.
//! - **rustls-tls**: Enables TLS functionality provided by `rustls`.
//! - **json**: Provides serialization and deserialization for JSON bodies.
//!
#![deny(missing_debug_implementations, unreachable_pub, missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use reqwest;

pub use self::error::*;
pub use self::session::*;
pub use self::status::*;

mod error;
mod session;
mod status;

mod endpoint;

pub mod auth;

#[cfg(feature = "webvpn")]
pub mod webvpn;

pub mod doc;
