use thiserror::Error;

/// Errors may occur during session operations.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// Errors occur when the internal state does not meet expectations.
    ///
    /// Common causes:
    /// - login via [`DirectEndpoint`] in sessions that already have logged-in user via [`DirectEndpoint`]
    /// - login via [`WebVPNEndpoint`] in sessions that have no logged-in user via [`DirectEndpoint`]
    /// - login via [`WebVPNEndpoint`] in sessions that already have logged-in user via [`WebVPNEndpoint`]
    /// - the page redirect behavior is changed
    ///
    /// See also [documentation for endpoints](crate::doc::endpoint).
    ///
    /// [`DirectEndpoint`]: crate::doc::endpoint::DirectEndpoint
    /// [`WebVPNEndpoint`]: crate::doc::endpoint::WebVPNEndpoint
    #[error("conflict status")]
    StatusConflict,

    /// Errors caused in page parsing process.
    ///
    /// Common causes:
    /// - CAS service is upgraded
    /// - the structure of auth-related pages are changed
    /// - the page redirect behavior is changed
    #[error("can not parse the page of url {url})")]
    ParsePageError {
        /// The url causes parsing process failed
        url: String,
    },

    /// Errors from reqwest layer.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

/// A `Result` alias where the `Err` case is [`neust::Error`](crate::error::Error).
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn parse_page_error(url: impl Into<String>) -> Self {
        Error::ParsePageError { url: url.into() }
    }
}
