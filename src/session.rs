use std::sync::Arc;

use async_trait::async_trait;
use reqwest::{
    cookie::{CookieStore, Jar},
    Client, ClientBuilder,
};
use sealed::sealed;

use crate::error::Result;
#[cfg(feature = "webvpn")]
use crate::platform::PLATFORM_WEBVPN;
use crate::platform::{Platform, PLATFORM_CAS};
use crate::status::UserStatus;

#[sealed(pub(crate))]
#[async_trait]
pub trait AuthMethod {
    async fn execute(&self, session: &Session, platform: &Platform) -> Result<UserStatus>;
}

#[derive(Debug, Clone)]
pub struct Session {
    client: Client,
    cookie_jar: Arc<Jar>,
}

impl Session {
    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn cookie_jar(&self) -> &Jar {
        &self.cookie_jar
    }
}

impl AsRef<Client> for Session {
    fn as_ref(&self) -> &Client {
        &self.client
    }
}

impl Default for Session {
    fn default() -> Self {
        Session::with_client_builder(|b| b)
    }
}

impl Session {
    pub fn new() -> Self {
        Session::default()
    }
}

impl Session {
    pub fn with_client_builder<B>(build: B) -> Self
    where
        B: FnOnce(ClientBuilder) -> ClientBuilder,
    {
        static UA: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        let cookie_jar = Arc::new(Jar::default());

        let client = build(ClientBuilder::new().user_agent(UA))
            .cookie_provider(cookie_jar.clone())
            .build()
            .expect("cannot initialize TLS backend, or cannot load the system configuration");

        Session { client, cookie_jar }
    }
}

impl Session {
    pub(crate) async fn check_status(&self, platform: &Platform) -> Result<UserStatus> {
        let request = self.client.get(platform.login_url).build()?;

        let response_body = self.client.execute(request).await?.text().await?;

        let cookie = self
            .cookie_jar
            .cookies(&platform.cookie_url)
            .and_then(|h| h.to_str().map(|s| s.to_owned()).ok())
            .and_then(|s| find_cookie_value(&s, platform.cookie_name));

        Ok(UserStatus::from_response_html(&response_body, cookie))
    }
}

impl Session {
    pub async fn login_cas_passport<A: AuthMethod>(&self, auth: &A) -> Result<UserStatus> {
        auth.execute(self, &PLATFORM_CAS).await
    }

    pub async fn check_cas_passport_status(&self) -> Result<UserStatus> {
        self.check_status(&PLATFORM_CAS).await
    }
}

#[cfg(feature = "webvpn")]
impl Session {
    pub async fn login_webvpn_passport<A: AuthMethod>(&self, auth: &A) -> Result<UserStatus> {
        auth.execute(self, &PLATFORM_WEBVPN).await
    }

    pub async fn check_webvpn_passport_status(&self) -> Result<UserStatus> {
        self.check_status(&PLATFORM_WEBVPN).await
    }
}

fn find_cookie_value(raw: &str, cookie_name: &str) -> Option<String> {
    match raw.find(cookie_name) {
        None => None,
        Some(i) => {
            let start_index = i + cookie_name.len() + 1;
            let sub = &raw[start_index..];
            Some(match sub.find(';') {
                None => sub.to_owned(),
                Some(end_index) => sub[..end_index].to_owned(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::session::find_cookie_value;

    #[test]
    fn test_find_cookie_value() {
        let table = vec![
            ("refresh=1; wengine_vpn_ticketwebvpn_neu_edu_cn=3c2cca8a854e8122", "wengine_vpn_ticketwebvpn_neu_edu_cn", Some("3c2cca8a854e8122".to_owned())),
            ("CASTGC=TGT-20180000-1827000-izbHeCI9y53RyIpMoYKxKbdyjtkgmfOy0NwbJHHiwXQabRYYKK-tpass; Language=zh_CN; jsessionid_tpass=ZLr9vBLe0xcX0nPsDfv3WASFiziyH-sMuy4CDoiIcqJkASjw136y!-1701433832", "CASTGC", Some("TGT-20180000-1827000-izbHeCI9y53RyIpMoYKxKbdyjtkgmfOy0NwbJHHiwXQabRYYKK-tpass".to_owned())),
            ("CASTGC=TGT-20180000-1827000-izbHeCI9y53RyIpMoYKxKbdyjtkgmfOy0NwbJHHiwXQabRYYKK-tpass; Language=zh_CN; jsessionid_tpass=ZLr9vBLe0xcX0nPsDfv3WASFiziyH-sMuy4CDoiIcqJkASjw136y!-1701433832", "jsessionid_tpass", Some("ZLr9vBLe0xcX0nPsDfv3WASFiziyH-sMuy4CDoiIcqJkASjw136y!-1701433832".to_owned())),
            ("", "wengine_vpn_ticketwebvpn_neu_edu_cn", None),
        ];

        for (raw, name, expected) in table {
            assert_eq!(find_cookie_value(raw, name), expected)
        }
    }
}
