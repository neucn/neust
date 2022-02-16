//! Several implementations for [`AuthMethod`](crate::session::AuthMethod).

pub use credential::Credential;
pub use token::Token;
#[cfg(feature = "wechat")]
pub use wechat::Wechat;

mod credential;
mod token;
#[cfg(feature = "wechat")]
mod wechat;
