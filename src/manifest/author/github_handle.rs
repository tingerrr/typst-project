use thiserror::Error;
use unscanny::Scanner;

use crate::{define_conversions, define_formatting, define_serde};

fn is_valid_github_handle(s: &str) -> Result<(), ParseGitHubHandleError> {
    if s.len() > 39 {
        return Err(ParseGitHubHandleError::TooLong);
    }

    if s.starts_with('-') {
        return Err(ParseGitHubHandleError::StartedWithHyphen);
    }

    if s.ends_with('-') {
        return Err(ParseGitHubHandleError::EndedWithHyphen);
    }

    let mut s = Scanner::new(s);

    while !s.done() {
        s.eat_while(|c: char| c.is_ascii_alphanumeric());

        let Some(c) = s.eat() else {
            break;
        };

        if c == '-' {
            if s.eat_if('-') {
                return Err(ParseGitHubHandleError::ContainsConsecutiveHyphens);
            }
        } else {
            return Err(ParseGitHubHandleError::ContainsInvalidChar(c));
        }
    }

    Ok(())
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GitHubHandle(String);

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseGitHubHandleError {
    #[error("handle must not be longer than 39 characters")]
    TooLong,

    #[error("handle must only contain alpha numeric characters and '-', contained {0:?}")]
    ContainsInvalidChar(char),

    #[error("handle must not start with a '-'")]
    StartedWithHyphen,

    #[error("handle must not end with a '-'")]
    EndedWithHyphen,

    #[error("handle must not contain '--'")]
    ContainsConsecutiveHyphens,
}

define_formatting!(GitHubHandle);
define_conversions!(GitHubHandle, ParseGitHubHandleError, is_valid_github_handle);
define_serde!(
    GitHubHandle,
    ParseGitHubHandleError,
    is_valid_github_handle,
    "a github handle"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_hyphens() {
        assert_eq!(
            is_valid_github_handle("-reknih"),
            Err(ParseGitHubHandleError::StartedWithHyphen)
        );
        assert_eq!(
            is_valid_github_handle("reknih-"),
            Err(ParseGitHubHandleError::EndedWithHyphen)
        );
        assert_eq!(
            is_valid_github_handle("r--knih"),
            Err(ParseGitHubHandleError::ContainsConsecutiveHyphens)
        );
    }

    #[test]
    fn invalid_char() {
        assert_eq!(
            is_valid_github_handle("rek nih"),
            Err(ParseGitHubHandleError::ContainsInvalidChar(' '))
        );
        assert_eq!(
            is_valid_github_handle("@reknih"),
            Err(ParseGitHubHandleError::ContainsInvalidChar('@'))
        );
        assert_eq!(
            is_valid_github_handle("räknih"),
            Err(ParseGitHubHandleError::ContainsInvalidChar('ä'))
        );
    }

    #[test]
    fn valid() {
        assert!(is_valid_github_handle("reknih").is_ok());
        assert!(is_valid_github_handle("tingerrr").is_ok());
    }
}
