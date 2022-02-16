use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use sealed::sealed;

use crate::error::Result;
use crate::platform::Platform;
use crate::session::Session;
use crate::status::UserStatus;

#[derive(Debug, Clone)]
pub struct Cookie(String);

impl Cookie {
    #[allow(dead_code)]
    pub fn new(token: impl Into<String>) -> Self {
        Cookie { 0: token.into() }
    }
}

#[sealed]
#[async_trait]
impl crate::session::AuthMethod for Cookie {
    async fn execute(&self, session: &Session, platform: &Platform) -> Result<UserStatus> {
        session.cookie_jar().add_cookie_str(
            format!("{}={}", platform.cookie_name, self.0).as_str(),
            &platform.cookie_url,
        );

        session.check_status(platform).await
    }
}

impl Display for Cookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cookie")
    }
}
