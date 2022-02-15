use tokio;

use neust::{auth::Cookie, Session};

mod test_env;

#[ignore]
#[tokio::test]
async fn test_webvpn_login() {
    let session = Session::new();
    let auth = test_env::get_credential_auth();
    // login webvpn via cas passport first
    test_env::extract_cookie(session.login_cas_passport(&auth).await);
    // login webvpn passport
    let cookie_a = test_env::extract_cookie(session.login_webvpn_passport(&auth).await);
    let cookie_b = test_env::extract_cookie(session.check_webvpn_passport_status().await);

    assert_eq!(cookie_a, cookie_b);

    // login via cookie
    let session = Session::new();
    let auth = Cookie::new(cookie_a.clone());
    let cookie_c = test_env::extract_cookie(session.login_webvpn_passport(&auth).await);
    assert_eq!(cookie_a, cookie_c);
}
