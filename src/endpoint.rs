use lazy_static::lazy_static;
use reqwest::Url;

#[derive(Debug)]
pub struct Endpoint {
    pub login_url: &'static str,
    pub cookie_name: &'static str,
    pub wechat_verify_url: &'static str,
    pub cookie_url: Url,
}

lazy_static! {
    pub(crate) static ref ENDPOINT_DIRECT: Endpoint = Endpoint {
        login_url: "https://pass.neu.edu.cn/tpass/login",
        cookie_name: "CASTGC",
        wechat_verify_url: "https://pass.neu.edu.cn/tpass/checkQRCodeScan",
        cookie_url: Url::parse("https://pass.neu.edu.cn/tpass/").unwrap(),
    };
}

#[cfg(feature = "webvpn")]
lazy_static! {
    pub(crate) static ref ENDPOINT_WEBVPN: Endpoint  = Endpoint {
        login_url: "https://webvpn.neu.edu.cn/https/77726476706e69737468656265737421e0f6528f693e6d45300d8db9d6562d/tpass/login",
        cookie_name: "wengine_vpn_ticketwebvpn_neu_edu_cn",
        wechat_verify_url: "https://webvpn.neu.edu.cn/https/77726476706e69737468656265737421e0f6528f693e6d45300d8db9d6562d/tpass/checkQRCodeScan",
        cookie_url: Url::parse("https://webvpn.neu.edu.cn/").unwrap(),
    };
}
