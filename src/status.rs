use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;
use regex::Regex;

/// The endpoint-specific user status in a [`Session`](crate::session::Session).
#[derive(Debug)]
#[non_exhaustive]
pub enum UserStatus {
    /// User is online and the account is active.
    Active {
        /// The endpoint-specific token.
        ///
        /// Can be an empty string iff the CAS has breaking changes.
        token: String,
        /// The username of the logged-in user.
        ///
        /// Can be an empty string iff the portal (the default service under the CAS) has changes.
        username: String,
    },
    /// User is online but the account needs reset.
    NeedReset {
        /// The endpoint-specific token.
        ///
        /// Can be an empty string iff the CAS has breaking changes.
        token: String,
    },
    /// User is online but the account is banned.
    Banned {
        /// The endpoint-specific token.
        ///
        /// Can be an empty string iff the CAS has breaking changes.
        token: String,
    },
    /// As a result of login action, it may mean:
    /// - the credential is wrong
    /// - the token is expired
    /// - wechat has not authorized the login request yet
    ///
    /// As a result of check status action, it may mean:
    /// - the user session is expired
    /// - no user has logged in
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
    /// Returns `true` if the status is [`Active`](UserStatus::Active).
    ///
    /// # Examples
    ///
    /// ```
    /// # use neust::UserStatus;
    /// let x = UserStatus::Active { username: "".to_owned(), token: "".to_owned() };
    /// assert_eq!(x.is_active(), true);
    ///
    /// let x = UserStatus::Rejected;
    /// assert_eq!(x.is_active(), false);
    /// ```
    pub fn is_active(&self) -> bool {
        matches!(self, UserStatus::Active { .. })
    }

    /// Returns `true` if the status is [`Rejected`](UserStatus::Rejected).
    ///
    /// # Examples
    ///
    /// ```
    /// # use neust::UserStatus;
    /// let x = UserStatus::Rejected;
    /// assert_eq!(x.is_rejected(), true);
    ///
    /// let x = UserStatus::Active { username: "".to_owned(), token: "".to_owned() };
    /// assert_eq!(x.is_rejected(), false);
    /// ```
    pub fn is_rejected(&self) -> bool {
        matches!(self, UserStatus::Rejected)
    }

    /// Get the username iff the status is [`Active`](UserStatus::Active).
    ///
    /// # Examples
    ///
    /// ```
    /// # use neust::UserStatus;
    /// let x = UserStatus::Active { username: "".to_owned(), token: "".to_owned() };
    /// assert!(matches!(x.get_username(), Some(_)));
    ///
    /// let x = UserStatus::Banned { token: "".to_owned() };
    /// assert!(matches!(x.get_username(), None));
    ///
    /// let x = UserStatus::Rejected;
    /// assert!(matches!(x.get_username(), None));
    /// ```
    pub fn get_username(&self) -> Option<&str> {
        match self {
            UserStatus::Active { username, .. } => Some(username),
            _ => None,
        }
    }

    /// Get the token.
    /// Returns [`None`] iff the status is [`Rejected`](UserStatus::Rejected).
    ///
    /// # Examples
    ///
    /// ```
    /// # use neust::UserStatus;
    /// let x = UserStatus::Active { username: "".to_owned(), token: "".to_owned() };
    /// assert!(matches!(x.get_token(), Some(_)));
    ///
    /// let x = UserStatus::Banned { token: "".to_owned() };
    /// assert!(matches!(x.get_token(), Some(_)));
    ///
    /// let x = UserStatus::Rejected;
    /// assert!(matches!(x.get_token(), None));
    /// ```
    pub fn get_token(&self) -> Option<&str> {
        match self {
            UserStatus::Active { token, .. } => Some(token),
            UserStatus::Banned { token } => Some(token),
            UserStatus::NeedReset { token } => Some(token),
            _ => None,
        }
    }
}

impl UserStatus {
    pub(crate) fn from_response_html(html: &str, token: Option<String>) -> UserStatus {
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

        let token = token.unwrap_or_else(|| "".into());

        match title {
            Some("智慧东大--统一身份认证") => UserStatus::Rejected,
            Some("智慧东大") => UserStatus::NeedReset { token },
            Some("系统提示") => UserStatus::Banned { token },
            _ => UserStatus::Active { token, username },
        }
    }
}
