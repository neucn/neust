use std::time::Duration;
use neust::{Session, auth::Wechat};
use tokio;

#[tokio::main]
async fn main() {
    let session = Session::new();
    let auth = Wechat::default();
    println!("{}", auth.get_auth_url());
    tokio::time::sleep(Duration::from_secs(20)).await;
    println!("{:?}", session.login_cas_passport(&auth).await);
}
