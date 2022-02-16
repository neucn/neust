#![allow(dead_code)]

use tokio;

use neust::auth::Credential;
use neust::Result;
use neust::UserStatus;
use neust::{auth::Token, Session};

#[ignore]
#[tokio::test]
async fn test_login() {
    let session = Session::new();
    let auth = get_credential();
    let token_a = extract_token(session.login(&auth).await);
    let token_b = extract_token(session.check_status().await);
    assert_eq!(token_a, token_b);

    let session = Session::new();
    let auth = Token::new(token_a.clone());
    let token_c = extract_token(session.login(&auth).await);
    assert_eq!(token_a, token_c);
}

#[cfg(feature = "webvpn")]
#[ignore]
#[tokio::test]
async fn test_login_via_webvpn() {
    let session = Session::new();
    let auth = get_credential();
    extract_token(session.login(&auth).await);

    let token_a = extract_token(session.login_via_webvpn(&auth).await);
    let token_b = extract_token(session.check_status_via_webvpn().await);

    assert_eq!(token_a, token_b);

    let session = Session::new();
    let auth = Token::new(token_a.clone());
    let token_c = extract_token(session.login_via_webvpn(&auth).await);
    assert_eq!(token_a, token_c);
}

macro_rules! read_env {
    ($name:expr) => {
        std::env::var($name).expect(concat!("missing env `", $name, "` for test"))
    };
}

fn extract_token(result: Result<UserStatus>) -> String {
    let status = result.expect("some error occurs due to network or the CAS");
    if let UserStatus::Active { username, token } = status {
        assert_eq!(read_env!("TEST_USERNAME"), username);
        assert_ne!(token.len(), 0);
        token
    } else {
        panic!("something wrong, please check account status: {}", status);
    }
}

fn get_credential() -> Credential {
    Credential::new(read_env!("TEST_USERNAME"), read_env!("TEST_PASSWORD"))
}
