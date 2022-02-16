use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Client, Request};
use sealed::sealed;

use crate::endpoint::Endpoint;
use crate::error::{Error, Result};
use crate::session::Session;
use crate::status::UserStatus;

/// An auth method that takes username and password.
///
/// # Examples
///
/// [`Credential`] hides password in displayed string.
/// ```
/// # use neust::auth::Credential;
/// let credential = Credential::new("username", "password");
/// let display = format!("{}", credential);
/// assert!(display.find("password").is_none())
/// ```
///
/// Pass to [`Session`](crate::session::Session) to login.
/// ```no_run
/// # async fn doc() -> Result<(), neust::Error> {
/// # use neust::auth::Credential;
/// # use neust::Session;
/// let session = Session::new();
/// let credential = Credential::new("username", "password");
/// let status = session.login(&credential).await?;
/// # Ok(())
/// # }
/// ```
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Credential {
    username: String,
    password: String,
}

impl Credential {
    /// Creates a [`Credential`].
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Credential {
            username: username.into(),
            password: password.into(),
        }
    }

    fn build_login_request(&self, client: &Client, url: &str, lt: &str) -> Result<Request> {
        Ok(client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!(
                "rsa={}{}{}&ul={}&pl={}&lt={}&execution=e1s1&_eventId=submit",
                self.username,
                self.password,
                lt,
                self.username.len(),
                self.password.len(),
                lt
            ))
            .build()?)
    }
}

#[sealed]
#[async_trait]
impl crate::session::AuthMethod for Credential {
    async fn execute(&self, session: &Session, endpoint: &Endpoint) -> Result<UserStatus> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"LT-[0-9a-zA-Z-]+-tpass"#).unwrap();
        }

        let client = session.client();

        let pre_response = client
            .execute(client.get(endpoint.login_url).build()?)
            .await?;

        let pre_final_url = pre_response.url().as_str().to_owned();

        if !pre_final_url.starts_with(endpoint.login_url) {
            return Err(Error::StatusConflict);
        }

        let pre_body = pre_response.text().await?;

        let lt = RE
            .find(&pre_body)
            .map(|s| s.as_str())
            .ok_or_else(|| Error::parse_page_error(pre_final_url))?;

        let request = self.build_login_request(client, endpoint.login_url, lt)?;

        client.execute(request).await?.text().await?;

        session._check_status(endpoint).await
    }
}

impl Display for Credential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "credential#{}", self.username)
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::Credential;

    #[test]
    fn test_credential_cmp() {
        let credential_a = Credential::new("20180000", "password");
        let credential_b = Credential::new("20180000", "pass_word");
        let credential_c = Credential::new("20170000", "password");
        let credential_d = Credential::new("20180000", "password");
        assert_ne!(credential_a, credential_b);
        assert_ne!(credential_a, credential_c);
        assert_ne!(credential_b, credential_c);
        assert_eq!(credential_a, credential_d);
        assert_eq!(credential_d, credential_a);
        assert_eq!(credential_a, credential_a);
    }
}
