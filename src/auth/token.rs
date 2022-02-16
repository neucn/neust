use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use sealed::sealed;

use crate::endpoint::Endpoint;
use crate::error::Result;
use crate::session::Session;
use crate::status::UserStatus;

/// An auth method that takes endpoint-specific session token.
///
/// Usually tokens are gotten from logged-in sessions.
///
/// # Examples
///
/// [`Token`] hides detail value in displayed string.
/// ```
/// # use neust::auth::Token;
/// let token = Token::new("xxxx-yyyy-zzzz");
/// let display = format!("{}", token);
/// assert!(display.find("xxxx").is_none())
/// ```
///
/// Pass to [`Session`](crate::session::Session) to login.
/// ```no_run
/// # async fn doc() -> Result<(), neust::Error> {
/// # use neust::auth::Token;
/// # use neust::Session;
/// let session = Session::new();
/// let token = Token::new("xxxx-yyyy-zzzz");
/// let status = session.login(&token).await?;
/// # Ok(())
/// # }
/// ```
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token(String);

impl Token {
    /// Creates a [`Token`].
    pub fn new(token: impl Into<String>) -> Self {
        Token { 0: token.into() }
    }
}

#[sealed]
#[async_trait]
impl crate::session::AuthMethod for Token {
    async fn execute(&self, session: &Session, endpoint: &Endpoint) -> Result<UserStatus> {
        session.cookie_jar().add_cookie_str(
            format!("{}={}", endpoint.cookie_name, self.0).as_str(),
            &endpoint.cookie_url,
        );

        session._check_status(endpoint).await
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "token")
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::Token;

    #[test]
    fn test_token_cmp() {
        let token_a = Token::new("abc");
        let token_b = Token::new("xyz");
        let token_c = Token::new("abc");
        assert_ne!(token_a, token_b);
        assert_eq!(token_a, token_a);
        assert_eq!(token_a, token_c);
        assert_eq!(token_c, token_a);
    }
}
