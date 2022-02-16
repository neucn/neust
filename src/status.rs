use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
#[non_exhaustive]
pub enum UserStatus {
    Active { cookie: String, username: String },
    NeedReset { cookie: String },
    Banned { cookie: String },
    Rejected,
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active { username, .. } => write!(f, "active#{}", username),
            UserStatus::NeedReset { .. } => write!(f, "need reset"),
            UserStatus::Banned { .. } => write!(f, "banned"),
            UserStatus::Rejected => write!(f, "rejected"),
        }
    }
}

impl UserStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, UserStatus::Active { .. })
    }

    pub fn is_rejected(&self) -> bool {
        matches!(self, UserStatus::Rejected)
    }

    pub fn get_username(&self) -> Option<&str> {
        match self {
            UserStatus::Active { username, .. } => Some(username),
            _ => None,
        }
    }

    pub fn get_cookie(&self) -> Option<&str> {
        match self {
            UserStatus::Active { cookie, .. } => Some(cookie),
            UserStatus::Banned { cookie } => Some(cookie),
            UserStatus::NeedReset { cookie } => Some(cookie),
            _ => None,
        }
    }
}

impl UserStatus {
    pub(crate) fn from_response_html(html: &str, cookie: Option<String>) -> UserStatus {
        lazy_static! {
            static ref TITLE_RE: Regex = Regex::new(r"<title>(.+?)</title>").unwrap();
            static ref USERNAME_RE: Regex = Regex::new(r#"var id_number = "(.+?)""#).unwrap();
        }

        let title = TITLE_RE
            .captures(html)
            .and_then(|cap| cap.get(1).map(|s| s.as_str()));

        let username = USERNAME_RE
            .captures(html)
            .and_then(|cap| cap.get(1).map(|s| s.as_str()))
            .map(|s| s.to_owned())
            .unwrap_or_else(|| "".to_owned());

        let cookie = cookie.unwrap_or_else(|| "".into());

        match title {
            Some("智慧东大--统一身份认证") => UserStatus::Rejected,
            Some("智慧东大") => UserStatus::NeedReset { cookie },
            Some("系统提示") => UserStatus::Banned { cookie },
            _ => UserStatus::Active { cookie, username },
        }
    }
}
