#![allow(dead_code)]

use neust::auth::Credential;
use neust::Result;
use neust::UserStatus;

static USERNAME: &str = env!("NEU_USERNAME", "missing username for test");
static PASSWORD: &str = env!("NEU_PASSWORD", "missing password for test");

pub fn get_credential_auth() -> Credential {
    Credential::new(USERNAME, PASSWORD)
}

pub fn extract_cookie(result: Result<UserStatus>) -> String {
    let status = result.expect("some error occurs due to network or the passport");
    if let UserStatus::Active { username, cookie } = status {
        assert_eq!(USERNAME, username);
        assert_ne!(cookie.len(), 0);
        cookie
    } else {
        panic!("something wrong, please check account status: {}", status);
    }
}
