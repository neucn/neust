use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("can not parse the page of url {url})")]
    ParsePageError { url: String },

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn parse_page_error(url: impl Into<String>) -> Self {
        Error::ParsePageError { url: url.into() }
    }
}
