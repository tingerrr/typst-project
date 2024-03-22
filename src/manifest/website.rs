use thiserror::Error;

// TODO: maybe simply use the url crate here

use crate::{define_conversions, define_formatting, define_serde};

fn is_valid_website(s: &str) -> Result<(), ParseWebsiteError> {
    fn is_legal_in_website(c: u8) -> bool {
        c.is_ascii_alphanumeric() || b"-_.~:/?#[]@!$&'()*+,;=".contains(&c)
    }

    if s.as_bytes().iter().copied().all(is_legal_in_website) {
        Ok(())
    } else {
        Err(ParseWebsiteError::ContainsInvalidChar)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Website(String);

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseWebsiteError {
    #[error("url contained invalid byte")]
    ContainsInvalidChar,
}

define_formatting!(Website);
define_conversions!(Website, ParseWebsiteError, is_valid_website);
define_serde!(Website, ParseWebsiteError, is_valid_website, "a website");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_err, assert_ok};

    #[test]
    fn invalid() {
        assert_err!(is_valid_website("http://mha ug"));
        assert_err!(is_valid_website("http://mhä.ug"));
    }

    #[test]
    fn valid() {
        assert_ok!(is_valid_website("https://mha.ug"));
        assert_ok!(is_valid_website("https://github.com/tingerrr/hydra"));
    }
}
