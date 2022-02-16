use neust::{auth::Wechat, Session, UserStatus};
use tokio::time::{interval, timeout, Duration};

#[tokio::main]
async fn main() {
    let session = Session::new();
    let wechat = Wechat::default();
    println!("{}", wechat.get_auth_url());
    timeout(Duration::from_secs(60), async {
        let mut interval = interval(Duration::from_secs(2));

        loop {
            interval.tick().await;
            let status = session.login(&wechat).await.unwrap();
            match status {
                UserStatus::Active { username, .. } => {
                    println!("{}", username);
                    break;
                }
                UserStatus::Rejected => continue,
                _ => {
                    panic!("something wrong: {:?}", status)
                }
            }
        }
    })
    .await
    .unwrap();
}
