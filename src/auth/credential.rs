use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Client, Request};
use sealed::sealed;

use crate::error::{Error, Result};
use crate::platform::Platform;
use crate::session::Session;
use crate::status::UserStatus;

#[derive(Debug, Clone)]
pub struct Credential {
    username: String,
    password: String,
}

impl Credential {
    #[allow(dead_code)]
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
    async fn execute(&self, session: &Session, platform: &Platform) -> Result<UserStatus> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"LT-[0-9a-zA-Z-]+-tpass"#).unwrap();
        }

        let client = session.client();

        let pre_body = client
            .execute(client.get(platform.login_url).build()?)
            .await?
            .text()
            .await?;

        let lt = RE
            .find(&pre_body)
            .map(|s| s.as_str())
            .ok_or_else(|| Error::parse_page_error(platform.login_url))?;

        let request = self.build_login_request(client, platform.login_url, lt)?;

        client.execute(request).await?.text().await?;

        session.check_status(platform).await
    }
}

impl Display for Credential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "credential#{}", self.username)
    }
}
