use std::sync::Arc;

use async_trait::async_trait;
use reqwest::{
    cookie::{CookieStore, Jar},
    Client, ClientBuilder,
};
use sealed::sealed;

#[cfg(feature = "webvpn")]
use crate::endpoint::ENDPOINT_WEBVPN;
use crate::endpoint::{Endpoint, ENDPOINT_DIRECT};
use crate::error::Result;
use crate::status::UserStatus;

/// An abstraction of auth method used in [`Session`].
///
/// It's made sealed intentionally so custom implementations are disallowed.
#[sealed(pub(crate))]
#[async_trait]
pub trait AuthMethod {
    /// Execute auth process.
    async fn execute(&self, session: &Session, endpoint: &Endpoint) -> Result<UserStatus>;
}

/// An asynchronous reqwest client with cookie management.
///
/// To create instances, use [Session::default], [Session::new] or
/// [Session::with_client_builder]
///
/// You do **not** have to wrap it in an [`Rc`] or [`Arc`] to **reuse** it,
/// because it already uses an [`Arc`] internally.
///
/// [`Arc`]: std::sync::Arc
/// [`Rc`]: std::rc::Rc
#[derive(Debug, Clone)]
pub struct Session {
    client: Client,
    cookie_jar: Arc<Jar>,
}

impl Session {
    /// Get [`Client`] to send requests.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn doc() -> Result<(), neust::Error> {
    /// # use neust::Session;
    /// # let session = Session::default();
    /// let client = session.client();
    /// let request = client.get("https://google.com").build()?;
    /// let response = client.execute(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get internal [`Jar`].
    ///
    /// # Warn
    ///
    /// Cookies are processed automatically
    /// and different sessions should use different [`Session`]s,
    /// so operating cookies directly should be avoided unless for special purpose.
    ///
    /// # Examples
    ///
    /// ```
    /// # use neust::{Session, reqwest::Url};
    /// # let session = Session::default();
    /// let cookie_jar = session.cookie_jar();
    /// let url = Url::parse("https://example.com").unwrap();
    /// cookie_jar.add_cookie_str("some_new_cookie=123", &url);
    /// ```
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
    /// Creates a [`Session`] using default configurations.
    pub fn new() -> Self {
        Session::default()
    }
}

impl Session {
    /// Customize the client inside created [`Session`].
    ///
    /// Changes on [cookie_provider](crate::reqwest::ClientBuilder::cookie_provider) and
    /// [cookie_store](crate::reqwest::ClientBuilder::cookie_store)
    /// will be ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # use neust::Session;
    /// # use std::time::Duration;
    /// let session = Session::with_client_builder(
    ///     |builder| builder.connect_timeout(Duration::from_secs(10))
    /// );
    /// ```
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
    pub(crate) async fn _check_status(&self, endpoint: &Endpoint) -> Result<UserStatus> {
        let request = self.client.get(endpoint.login_url).build()?;

        let response_body = self.client.execute(request).await?.text().await?;

        let token = self
            .cookie_jar
            .cookies(&endpoint.cookie_url)
            .and_then(|h| h.to_str().map(|s| s.to_owned()).ok())
            .and_then(|s| find_cookie_value(&s, endpoint.cookie_name));

        Ok(UserStatus::from_response_html(&response_body, token))
    }
}

impl Session {
    /// Login to the CAS via [`DirectEndpoint`](crate::doc::endpoint::DirectEndpoint).
    ///
    /// # Warn
    ///
    /// Repeated login in sessions that already have logged-in user may cause
    /// [`Error::StatusConflict`](crate::error::Error::StatusConflict), because
    /// login won't clear the auth-related cookies every time it is called.
    ///
    /// This is an intentionally reserved **feature** to avoid the reuse of sessions.
    ///
    /// # Example
    /// ```no_run
    /// # async fn doc() -> Result<(), neust::Error> {
    /// # use neust::auth;
    /// # use neust::{Session, UserStatus};
    /// let session = Session::new();
    /// let credential = auth::Credential::new("username", "password");
    /// let status = session.login(&credential).await?;
    /// match status {
    ///     UserStatus::Active { username, .. } => println!("{}", username),
    ///     UserStatus::Rejected => println!("wrong username or password"),
    ///     _ => println!("something wrong: {:?}", status)
    /// };
    /// # Ok(())
    /// # }
    /// ```
    pub async fn login<A: AuthMethod>(&self, auth: &A) -> Result<UserStatus> {
        auth.execute(self, &ENDPOINT_DIRECT).await
    }

    /// Check user status on the CAS via [`DirectEndpoint`](crate::doc::endpoint::DirectEndpoint)
    /// in the session.
    ///
    /// # Example
    /// ```no_run
    /// # async fn doc() -> Result<(), neust::Error> {
    /// # use neust::{Session, UserStatus};
    /// # let session = Session::new();
    /// let status = session.check_status().await?;
    /// let current_username = status.get_username();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_status(&self) -> Result<UserStatus> {
        self._check_status(&ENDPOINT_DIRECT).await
    }
}

#[cfg(feature = "webvpn")]
#[cfg_attr(docsrs, doc(cfg(feature = "webvpn")))]
impl Session {
    /// Login to the CAS via [`WebVPNEndpoint`].
    ///
    /// # Warn
    ///
    /// Caller should ensure that there is **one** user logged in via [`DirectEndpoint`] and
    /// **no** user logged in via [`WebVPNEndpoint`]. Otherwise an [`Error::StatusConflict`]
    /// will be returned.
    ///
    /// # Example
    /// ```no_run
    /// # async fn doc() -> Result<(), neust::Error> {
    /// # use neust::auth;
    /// # use neust::Session;
    /// let session = Session::new();
    /// let credential = auth::Credential::new("username", "password");
    /// let status = session.login(&credential).await?;
    /// if status.is_active() {
    ///     session.login_via_webvpn(&credential).await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See also [documentation for endpoints](crate::doc::endpoint).
    ///
    /// [`DirectEndpoint`]: crate::doc::endpoint::DirectEndpoint
    /// [`WebVPNEndpoint`]: crate::doc::endpoint::WebVPNEndpoint
    /// [`Error::StatusConflict`]: crate::error::Error::StatusConflict
    pub async fn login_via_webvpn<A: AuthMethod>(&self, auth: &A) -> Result<UserStatus> {
        auth.execute(self, &ENDPOINT_WEBVPN).await
    }

    /// Check user status on the CAS via
    /// [`WebVPNEndpoint`](crate::doc::endpoint::WebVPNEndpoint) in the session.
    ///
    /// # Example
    /// ```no_run
    /// # async fn doc() -> Result<(), neust::Error> {
    /// # use neust::{Session, UserStatus};
    /// # let session = Session::new();
    /// let status = session.check_status_via_webvpn().await?;
    /// let current_username = status.get_username();
    /// # Ok(())
    /// # }
    /// ```
    /// See also [documentation for endpoints](crate::doc::endpoint).
    pub async fn check_status_via_webvpn(&self) -> Result<UserStatus> {
        self._check_status(&ENDPOINT_WEBVPN).await
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
