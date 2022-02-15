use std::fmt::{Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use rand;
use rand::Rng;
use sealed::sealed;

use crate::error::Result;
use crate::platform::Platform;
use crate::platform::WECHAT_AUTH_URL;
use crate::session::{Session, UserStatus};

#[derive(Debug, Clone)]
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
    #[allow(dead_code)]
    fn new(uuid: Option<String>) -> Self {
        match uuid {
            Some(uuid) => Wechat { uuid },
            None => Wechat::default(),
        }
    }
}

#[sealed]
#[async_trait]
impl crate::session::AuthMethod for Wechat {
    async fn execute(&self, session: &Session, platform: &Platform) -> Result<UserStatus> {
        let client = session.client();

        let verify_request = client
            .get(self.get_verify_url(platform.wechat_verify_url))
            .build()?;

        let body = client.execute(verify_request).await?.text().await?;

        match body.len() {
            0 => Ok(UserStatus::Rejected),
            _ => session.check_status(platform).await,
        }
    }
}

impl Wechat {
    #[allow(dead_code)]
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

    let mut uuid = String::new();

    for i in 0..36 {
        uuid.push(match i {
            8 | 13 | 18 | 23 => '-',
            _ => {
                let r = (d + rand_gen.gen::<f64>() * 16f64) as usize % 16;
                d = (d / 16f64).floor();
                match i {
                    14 => '4',
                    19 => HEX[(r & 0x3 | 0x8)],
                    _ => HEX[r],
                }
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
