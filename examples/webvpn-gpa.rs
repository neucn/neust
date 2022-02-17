//! This example requires feature **neust/webvpn**

use neust::{auth::Credential, webvpn, Session};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("please provide username and password")
    }

    let credential = Credential::new(&args[1], &args[2]);

    let session = Session::new();

    let status = session.login(&credential).await.unwrap();
    if !status.is_active() {
        panic!("error: {:?}", status)
    }

    let status = session.login_via_webvpn(&credential).await.unwrap();
    if !status.is_active() {
        panic!("error: {:?}", status)
    }

    let client = session.client();
    let request = client
        .get(webvpn::encrypt_url(
            "http://219.216.96.4/eams/teach/grade/course/person!search.action?semesterId=0",
        ))
        .build()
        .unwrap();
    let body = client.execute(request).await.unwrap().text().await.unwrap();

    let re = regex::Regex::new(r"<div>总平均绩点：([0-9.]+)</div>").unwrap();
    println!("{}", re.captures(&body).unwrap().get(1).unwrap().as_str());
}
