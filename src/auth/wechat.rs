use std::fmt::{Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use rand;
use rand::Rng;
use sealed::sealed;

use crate::endpoint::Endpoint;
use crate::error::Result;
use crate::session::Session;
use crate::status::UserStatus;

static WECHAT_AUTH_URL: &str = "https://pass.neu.edu.cn/tpass/qyQrLogin";

/// An auth method that takes authorization from Wechat.
///
/// # Examples
///
/// ```no_run
/// # async fn doc() -> Result<(), neust::Error> {
/// # use neust::auth::Wechat;
/// # use neust::Session;
/// # use neust::UserStatus;
/// use tokio::time::{interval, timeout, Duration};
///
/// let session = Session::new();
/// let wechat = Wechat::default();
///
/// // let user visit auth_url on Wechat and authorize.
/// let auth_url = wechat.get_auth_url();
///
/// // check whether user has authorized.
/// timeout(Duration::from_secs(60), async {
///     let mut interval = interval(Duration::from_secs(2));
///     loop {
///         interval.tick().await;
///         match session.login(&wechat).await {
///             Ok(status) => match status {
///                 UserStatus::Active { .. } => break,
///                 UserStatus::Rejected => continue,
///                 _ => panic!("something wrong: {:?}", status)
///             },
///             Err(e) => panic!("something wrong: {:?}", e)
///         }
///     }
/// }).await;
/// # Ok(())
/// # }
/// ```
///
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(docsrs, doc(cfg(feature = "wechat")))]
pub struct Wechat {
    uuid: String,
}

impl Default for Wechat {
    fn default() -> Self {
        Wechat {
            uuid: generate_uuid(),
        }
    }
}

impl Wechat {
    /// Creates a [`Wechat`] with an existed UUID.
    ///
    /// If the provided UUID is [`None`], a new random UUID will be used instead. Or you can
    /// use [`Wechat::default`] directly.
    pub fn new(uuid: Option<String>) -> Self {
        match uuid {
            Some(uuid) => Wechat { uuid },
            None => Wechat::default(),
        }
    }
}

#[sealed]
#[async_trait]
impl crate::session::AuthMethod for Wechat {
    async fn execute(&self, session: &Session, endpoint: &Endpoint) -> Result<UserStatus> {
        let client = session.client();

        let verify_request = client
            .get(self.get_verify_url(endpoint.wechat_verify_url))
            .build()?;

        let body = client.execute(verify_request).await?.text().await?;

        match body.len() {
            0 => Ok(UserStatus::Rejected),
            _ => session._check_status(endpoint).await,
        }
    }
}

impl Wechat {
    /// Get the url for authorization on Wechat.
    ///
    /// See also examples of [`Wechat`].
    pub fn get_auth_url(&self) -> String {
        format!("{}?uuid={}", WECHAT_AUTH_URL, self.uuid)
    }

    pub(crate) fn get_verify_url(&self, base_url: &str) -> String {
        format!(
            "{}?random={}&uuid={}",
            base_url,
            rand::random::<f64>(),
            self.uuid
        )
    }
}

fn generate_uuid() -> String {
    static HEX: &[char; 16] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    let mut d = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("fail to get current time")
        .as_millis() as f64;

    let mut rand_gen = rand::thread_rng();

    let mut uuid = String::with_capacity(36);

    for i in 0..36 {
        uuid.push(match i {
            8 | 13 | 18 | 23 => '-',
            _ => {
                let r = (d + rand_gen.gen::<f64>() * 16f64) as usize % 16;
                d = (d / 16f64).floor();
                HEX[r]
            }
        });
    }

    uuid
}

impl Display for Wechat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wechat#{}", self.uuid)
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::Wechat;

    #[test]
    fn test_wechat_cmp() {
        let uuid_a= "a";
        let uuid_b= "b";
        let wechat_a = Wechat::new(Some(uuid_a.to_owned()));
        let wechat_b = Wechat::new(Some(uuid_b.to_owned()));
        let wechat_c = Wechat::new(Some(uuid_a.to_owned()));
        assert_ne!(wechat_a, wechat_b);
        assert_eq!(wechat_a, wechat_a);
        assert_eq!(wechat_a, wechat_c);
        assert_eq!(wechat_c, wechat_a);
    }
}