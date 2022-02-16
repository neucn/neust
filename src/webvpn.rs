//! Provide helper functions for operations on WebVPN.

use aes::{
    cipher::{AsyncStreamCipher, KeyIvInit},
    Aes128,
};
use cfb_mode::Encryptor;

/// Encrypts a service url so that it can be accessed
/// via [`WebVPNEndpoint`](crate::doc::endpoint).
///
/// # Note
///
/// The scheme of URL will be inferred according to the following rules:
/// - if url starts with "https://" => https
/// - if url starts with "http://" or "//" or without scheme => http
///
/// # Examples
/// ```
/// # async fn doc() {
/// assert_eq!(
///     neust::webvpn::encrypt_url("http://219.216.96.4/eams/homeExt.action"),
///     "https://webvpn.neu.edu.cn/http/77726476706e69737468656265737421a2a618d275613e1e275ec7f8/eams/homeExt.action"
/// )
/// # }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "webvpn")))]
pub fn encrypt_url(url: impl AsRef<str>) -> String {
    let mut scheme = "http";

    let url = url.as_ref();

    // get scheme and url
    let url = match url.strip_prefix("https://") {
        Some(u) => {
            scheme = "https";
            u
        }
        None => url
            .strip_prefix("http://")
            .or_else(|| url.strip_prefix("//"))
            .unwrap_or(url),
    };

    // get hostname and port
    let segments = url
        .split('?')
        .next()
        .unwrap()
        .split(':')
        .collect::<Vec<&str>>();

    let (url, port) = if segments.len() > 1 {
        let hostname_len = segments[0].len();
        let port = segments[1].split('/').next().unwrap();
        (
            format!(
                "{}{}",
                &url[..hostname_len],
                &url[hostname_len + port.len() + 1..]
            ),
            Some(port),
        )
    } else {
        (url.into(), None)
    };

    // encrypt
    let encrypted_url = match url.find('/') {
        Some(index) => format!("{}{}", encrypt(url[..index].as_bytes()), &url[index..]),
        None => encrypt(url.as_bytes()),
    };

    match port {
        Some(port) => format!(
            "https://webvpn.neu.edu.cn/{}-{}/{}",
            scheme, port, encrypted_url
        ),
        None => format!("https://webvpn.neu.edu.cn/{}/{}", scheme, encrypted_url),
    }
}

type Aes128Cfb = Encryptor<Aes128>;

fn encrypt(plaintext: &[u8]) -> String {
    static KEY: &[u8] = b"wrdvpnisthebest!";

    let mut buf = plaintext.to_vec();
    Aes128Cfb::new(KEY.into(), KEY.into()).encrypt(&mut buf);
    format!("{}{}", hex::encode(KEY), hex::encode(buf))
}

#[cfg(test)]
mod tests {
    use crate::webvpn::encrypt_url;

    #[test]
    fn test_encrypt_webvpn_url() {
        let table = vec![
            ("http://219.216.96.4/eams/homeExt.action", "https://webvpn.neu.edu.cn/http/77726476706e69737468656265737421a2a618d275613e1e275ec7f8/eams/homeExt.action"),
            ("http://219.216.96.4/eams/", "https://webvpn.neu.edu.cn/http/77726476706e69737468656265737421a2a618d275613e1e275ec7f8/eams/"),
            ("https://portal.neu.edu.cn/", "https://webvpn.neu.edu.cn/https/77726476706e69737468656265737421e0f85388263c265e7b1dc7a99c406d369a/"),
            ("//ipgw.neu.edu.cn", "https://webvpn.neu.edu.cn/http/77726476706e69737468656265737421f9e7468b693e6d45300d8db9d6562d"),
            ("http://210.30.200.128:8080/system/caslogin.jsp", "https://webvpn.neu.edu.cn/http-8080/77726476706e69737468656265737421a2a611d2746026022e58c7fdca0d/system/caslogin.jsp"),
            ("http://202.118.8.7:8991/F/29DK3KT4SV9VBRI548R8UD3MBIT991BXE4HLXENCFEGE54551T-22111?func=find-b-0", "https://webvpn.neu.edu.cn/http-8991/77726476706e69737468656265737421a2a713d27661301e2646de/F/29DK3KT4SV9VBRI548R8UD3MBIT991BXE4HLXENCFEGE54551T-22111?func=find-b-0"),
        ];

        for (case, expected) in table {
            assert_eq!(encrypt_url(case), expected)
        }
    }
}
